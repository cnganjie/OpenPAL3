use std::{cell::RefCell, rc::Rc};

#[cfg(enable_debug)]
use super::debug::{DebugIpcClient, Notification, Request};

use super::{
    global_context::{GlobalFunctionContinuation, ScriptGlobalContext},
    module::{ScriptFunction, ScriptModule},
};

#[derive(Clone)]
pub(crate) struct ScriptFunctionContext {
    pub(crate) module: Rc<RefCell<ScriptModule>>,
    function_index: usize,
    pc: usize,
}

impl ScriptFunctionContext {
    pub fn new(module: Rc<RefCell<ScriptModule>>, function_index: usize) -> Self {
        Self {
            module,
            function_index,
            pc: 0,
        }
    }
}

pub struct ScriptVm<TAppContext: 'static> {
    pub(crate) app_context: TAppContext,
    pub(crate) g: Rc<RefCell<ScriptGlobalContext<TAppContext>>>,
    pub(crate) context: Option<ScriptFunctionContext>,

    #[cfg(enable_debug)]
    debug_client: DebugIpcClient,

    call_stack: Vec<ScriptFunctionContext>,

    pub(crate) heap: Vec<Option<String>>,
    pub(crate) robj: usize,

    stack: Vec<u8>,
    sp: usize,
    fp: usize,
    r1: u32,
    r2: u32,

    yield_func: Vec<GlobalFunctionContinuation<TAppContext>>,
    pub(crate) imm: bool,
}

impl<TAppContext: 'static> ScriptVm<TAppContext> {
    const DEFAULT_STACK_SIZE: usize = 1024;

    pub fn new(
        g: Rc<RefCell<ScriptGlobalContext<TAppContext>>>,
        module: Rc<RefCell<ScriptModule>>,
        function_index: usize,
        app_context: TAppContext,
    ) -> Self {
        let mut vm = Self {
            app_context,
            g,
            context: Some(ScriptFunctionContext::new(module, function_index)),
            call_stack: vec![],
            heap: vec![],
            r1: 0,
            r2: 0,
            robj: 0,
            yield_func: vec![],

            #[cfg(enable_debug)]
            debug_client: DebugIpcClient::new(),

            stack: vec![0; Self::DEFAULT_STACK_SIZE],
            sp: Self::DEFAULT_STACK_SIZE,
            fp: Self::DEFAULT_STACK_SIZE,

            imm: true,
        };

        vm.debug_update_module();
        vm
    }

    pub fn app_context(&self) -> &TAppContext {
        &self.app_context
    }

    pub fn app_context_mut(&mut self) -> &mut TAppContext {
        &mut self.app_context
    }

    pub fn set_function(&mut self, module: Rc<RefCell<ScriptModule>>, index: usize) {
        if self.context.is_some() {
            self.call_stack.push(self.context.clone().unwrap());
        }

        self.context = Some(ScriptFunctionContext::new(module, index));

        self.debug_update_module();
    }

    pub fn set_function_by_name2(&mut self, module: Rc<RefCell<ScriptModule>>, name: &str) {
        for (i, f) in module.borrow().functions.iter().enumerate() {
            if f.name.as_str() == name {
                self.set_function(module.clone(), i)
            }
        }
    }

    pub fn stack_peek<T: std::marker::Copy>(&mut self) -> Option<T> {
        if self.sp < self.stack.len() - std::mem::size_of::<T>() {
            let ret: T = unsafe { self.read_stack(self.sp) };
            Some(ret)
        } else {
            None
        }
    }

    pub fn stack_pop<T: std::marker::Copy>(&mut self) -> T {
        let ret: T = unsafe { self.read_stack(self.sp) };
        self.sp += std::mem::size_of::<T>();
        ret
    }

    pub fn stack_push<T: std::marker::Copy>(&mut self, ret: T) {
        self.sp -= std::mem::size_of::<T>();
        unsafe { self.write_stack(self.sp, ret) };
    }

    pub fn push_object(&mut self, object: String) -> usize {
        for i in 0..self.heap.len() {
            if self.heap[i].is_none() {
                self.heap[i] = Some(object);
                return i;
            }
        }

        self.heap.push(Some(object));
        return self.heap.len() - 1;
    }

    pub fn execute(&mut self, delta_sec: f32) {
        loop {
            if self.context.is_none() {
                return;
            }

            let module = self.context.as_ref().unwrap().module.clone();
            let module_ref = module.borrow();
            let function =
                module_ref.functions[self.context.as_ref().unwrap().function_index].clone();
            let mut reg: u32 = 0;

            self.debug_update_context();
            self.wait_for_action();

            let mut wait = false;
            let mut new_funcs = vec![];
            while let Some(mut cont) = self.yield_func.pop() {
                match cont(self, delta_sec) {
                    crate::scripting::angelscript::ContinuationState::Loop => {
                        new_funcs.push(cont);
                        wait = true;
                    }
                    crate::scripting::angelscript::ContinuationState::Concurrent => {
                        new_funcs.push(cont);
                    }
                    crate::scripting::angelscript::ContinuationState::Completed => {}
                }
            }

            self.yield_func = new_funcs;

            if wait {
                return;
            }

            let inst = self.read_inst(&function);
            macro_rules! command {
                ($cmd_name: ident $(, $param_name: ident : $param_type: ident)*) => {{
                    $(let $param_name = data_read::$param_type(&function.inst, &mut self.context.as_mut().unwrap().pc);)*
                    self.$cmd_name($($param_name ,)*);
                }};

                ($cmd_name: ident : $g_type: ident $(, $param_name: ident : $param_type: ident)*) => {{
                    $(let $param_name = data_read::$param_type(&function.inst, &mut self.context.as_mut().unwrap().pc);)*
                    self.$cmd_name::<$g_type>($($param_name)*);
                }};
            }

            match inst {
                0 => command!(pop, size: u16),
                1 => command!(push, size: u16),
                2 => command!(set4, size: u32),
                3 => self.rd4(),
                4 => command!(rdsf4, index: u16),
                5 => self.wrt4(),
                6 => self.mov4(),
                7 => command!(psf, index: u16),
                8 => command!(movsf4, index: u16),
                9 => self.swap::<u32>(),
                10 => self.store4(&mut reg),
                11 => self.recall4(reg),
                12 => command!(call, function: u32),
                13 => {
                    command!(ret, param_size: u16);
                    return;
                }
                14 => command!(jmp, offset: i32),
                15 => command!(jz, offset: i32),
                16 => command!(jnz, offset: i32),
                17 => self.tz(),
                18 => self.tnz(),
                19 => self.ts_ltz(),
                20 => self.tns_gez(),
                21 => self.tp_gtz(),
                22 => self.tnp_lez(),
                23 => self.add::<i32>(),
                24 => self.sub::<i32>(),
                25 => self.mul::<i32>(),
                26 => self.div::<i32>(0),
                27 => self.xmod::<i32>(0),
                28 => self.neg::<i32>(),
                29 => self.cmp::<i32>(),
                30 => self.inc::<i32>(1),
                31 => self.dec::<i32>(1),
                32 => self.i2f(),
                33 => self.add::<f32>(),
                34 => self.sub::<f32>(),
                35 => self.mul::<f32>(),
                36 => self.div::<f32>(0.),
                37 => self.xmod::<f32>(0.),
                38 => self.neg::<f32>(),
                39 => self.cmp::<f32>(),
                40 => self.inc::<f32>(1.),
                41 => self.dec::<f32>(1.),
                42 => self.f2i(),
                43 => self.bnot(),
                44 => self.band(),
                45 => self.bor(),
                46 => self.bxor(),
                47 => self.bsll(),
                48 => self.bsrl(),
                49 => self.bsra(),
                50 => self.ui2f(),
                51 => self.f2ui(),
                52 => self.cmp::<u32>(),
                53 => self.sb(),
                54 => self.sw(),
                55 => self.ub(),
                56 => self.uw(),
                57 => self.wrt1(),
                58 => self.wrt2(),
                59 => self.inc::<i16>(1),
                60 => self.inc::<i8>(1),
                61 => self.dec::<i16>(1),
                62 => self.dec::<i8>(1),
                63 => self.push_zero(),
                64 => command!(copy, count: u16),
                65 => command!(pga, index: i32),
                66 => command!(set8, data: u64),
                67 => self.wrt8(),
                68 => self.rd8(),
                69 => self.neg::<f64>(),
                70 => self.inc::<f64>(1.),
                71 => self.dec::<f64>(1.),
                72 => self.add::<f64>(),
                73 => self.sub::<f64>(),
                74 => self.mul::<f64>(),
                75 => self.div::<f64>(0.),
                76 => self.xmod::<f64>(0.),
                77 => self.swap::<f64>(),
                78 => self.cmp::<f64>(),
                79 => self.d2i(),
                80 => self.d2ui(),
                81 => self.d2f(),
                82 => self.x2d::<i32>(),
                83 => self.x2d::<u32>(),
                84 => self.x2d::<f32>(),
                85 => self.jmpp(),
                86 => self.sret4(),
                87 => self.sret8(),
                88 => self.rret4(),
                89 => self.rret8(),
                90 => command!(str, index: u16),
                91 => command!(js_jgez, offset: i32),
                92 => command!(jns_jlz, offset: i32),
                93 => command!(jp_jlez, offset: i32),
                94 => command!(jnp_jgz, offset: i32),
                95 => command!(cmpi: i32, rhs: i32),
                96 => command!(cmpi: u32, rhs: u32),
                97 => {
                    command!(callsys, function_index: i32);
                    /*if self.yield_func.is_some() {
                        return;
                    }*/
                    return;
                }
                98 => command!(callbnd, function_index: u32),
                99 => command!(rdga4, index: i32),
                100 => command!(movga4, index: i32),
                101 => command!(addi: i32, rhs: i32),
                102 => command!(subi: i32, rhs: i32),
                103 => command!(cmpi: f32, rhs: f32),
                104 => command!(addi: f32, rhs: f32),
                105 => command!(subi: f32, rhs: f32),
                106 => command!(muli: i32, rhs: i32),
                107 => command!(muli: f32, rhs: f32),
                108 => {
                    // Suspend
                    return;
                }
                109 => command!(alloc, this: i32, index: i32),
                110 => command!(free, obj_type: u32),
                111 => unimplemented!("byte code 111 - loadobj"),
                112 => command!(storeobj, param_index: i16),
                113 => unimplemented!("byte code 113 - getobj"),
                114 => unimplemented!("byte code 114 - refcpy"),
                115 => self.checkref(),
                116 => unimplemented!("byte code 116 - rd1"),
                117 => unimplemented!("byte code 117 - rd2"),
                118 => command!(getobjref, offset: i16),
                119 => unimplemented!("byte code 119 - getref"),
                120 => unimplemented!("byte code 120 - swap48"),
                121 => unimplemented!("byte code 121 - swap84"),
                122 => unimplemented!("byte code 122 - objtype"),
                i => unimplemented!("byte code {}", i),
            }
        }
    }

    fn read_inst(&mut self, function: &ScriptFunction) -> u8 {
        let inst = function.inst[self.context.as_ref().unwrap().pc];
        self.context.as_mut().unwrap().pc += 4;
        inst
    }

    fn pop(&mut self, size: u16) {
        self.sp += size as usize * 4;
    }

    fn push(&mut self, size: u16) {
        self.sp -= size as usize * 4;
    }

    fn set4(&mut self, data: u32) {
        self.sp -= 4;
        unsafe {
            self.write_stack(self.sp, data);
        }
    }

    fn rd4(&mut self) {
        unsafe {
            let pos: u32 = self.read_stack(self.sp);
            let data: u32 = self.read_stack(pos as usize);
            self.write_stack(self.sp, data);
        }
    }

    fn rdsf4(&mut self, index: u16) {
        unsafe {
            let data: u32 = self.read_stack(self.stack.len() - index as usize * 4);
            self.write_stack(self.sp, data);
        }
    }

    fn wrt4(&mut self) {
        unsafe {
            let pos: u32 = self.read_stack(self.sp);
            self.sp += 4;
            let data: u32 = self.read_stack(self.sp);
            self.write_stack(pos as usize, data);
        }
    }

    fn mov4(&mut self) {
        self.wrt4();
        self.sp += 4;
    }

    fn psf(&mut self, index: u16) {
        unsafe {
            let pos = self.fp - index as usize * 4;
            self.sp -= 4;
            self.write_stack(self.sp, pos as u32);
        }
    }

    fn movsf4(&mut self, index: u16) {
        unsafe {
            let pos = self.fp - index as usize * 4;
            let data: u32 = self.read_stack(pos);
            self.write_stack(pos, data);
            self.sp += 4;
        }
    }

    fn swap<T: Copy>(&mut self) {
        unsafe {
            let size = std::mem::size_of::<T>();
            let data: T = self.read_stack(self.sp);
            let data2: T = self.read_stack(self.sp + size);
            self.write_stack(self.sp, data2);
            self.write_stack(self.sp + size, data);
        }
    }

    fn store4(&mut self, reg: &mut u32) {
        unsafe {
            let data = self.read_stack(self.sp);
            *reg = data;
        }
    }

    fn recall4(&mut self, reg: u32) {
        unsafe {
            self.sp -= 4;
            self.write_stack(self.sp, reg);
        }
    }

    fn call(&mut self, function: u32) {
        let module = self.context.as_ref().unwrap().module.clone();
        self.set_function(module, function as usize);
    }

    fn callbnd(&mut self, function: u32) {
        println!("Unimplemented: call: {}", function);
    }

    fn rdga4(&mut self, offset: i32) {
        if offset < 0 {
            let index = -offset - 1;
            let context = self.g.clone();
            let data = context.borrow().get_global(index as usize);
            self.set4(data);
        } else {
            unimplemented!("Global memory not supported yet");
        }
    }

    fn callsys(&mut self, function: i32) {
        let index = -function - 1;
        let context = self.g.clone();
        let context = context.borrow();
        match context.call_function(self, index as usize) {
            super::GlobalFunctionState::Yield(cont) => self.yield_func.push(cont),
            super::GlobalFunctionState::Completed => {}
        }
    }

    fn alloc(&mut self, this: i32, function: i32) {
        println!("Unimplemented: call global2: {} {}", this, function);
    }

    fn storeobj(&mut self, param_index: i16) {
        unsafe {
            self.write_stack(
                (self.fp as isize - param_index as isize * 4) as usize,
                self.robj as u32,
            );
        }
    }

    fn free(&mut self, _obj_type: u32) {
        let obj_ref: u32 = unsafe { self.read_stack(self.sp) };
        let obj_index: u32 = unsafe { self.read_stack(obj_ref as usize) };
        self.sp += 4;
        self.heap[obj_index as usize] = None;
    }

    fn checkref(&mut self) {}

    fn getobjref(&mut self, offset: i16) {
        unsafe {
            let addr = (self.sp as isize + offset as isize * 4) as usize;
            let index: u32 = self.read_stack(addr);
            let objref: u32 = self.read_stack((self.fp as isize - index as isize * 4) as usize);
            self.write_stack(addr, objref);
        }
    }

    fn ret(&mut self, param_size: u16) {
        let func = self.call_stack.pop();
        self.context = func;

        self.sp -= param_size as usize;
    }

    fn jmp(&mut self, offset: i32) {
        self.context.as_mut().unwrap().pc += offset as usize;
    }

    fn jz(&mut self, offset: i32) {
        unsafe {
            let data: i32 = self.read_stack(self.sp);
            self.sp += 4;
            if data == 0 {
                self.jmp(offset);
            }
        }
    }

    fn jnz(&mut self, offset: i32) {
        unsafe {
            let data: i32 = self.read_stack(self.sp);
            self.sp += 4;
            if data != 0 {
                self.jmp(offset);
            }
        }
    }

    fn tz(&mut self) {
        self.unary_op::<i32, _, _>(|a| (a == 0) as i32);
    }

    fn tnz(&mut self) {
        self.unary_op::<i32, _, _>(|a| (a != 0) as i32);
    }

    fn ts_ltz(&mut self) {
        self.unary_op::<i32, _, _>(|a| (a < 0) as i32);
    }

    fn tns_gez(&mut self) {
        self.unary_op::<i32, _, _>(|a| (a >= 0) as i32);
    }

    fn tp_gtz(&mut self) {
        self.unary_op::<i32, _, _>(|a| (a > 0) as i32);
    }

    fn tnp_lez(&mut self) {
        self.unary_op::<i32, _, _>(|a| (a <= 0) as i32);
    }

    fn add<T: Copy + std::ops::Add>(&mut self) {
        self.binary_op::<T, _, _>(|a, b| b + a)
    }

    fn sub<T: Copy + std::ops::Sub>(&mut self) {
        self.binary_op::<T, _, _>(|a, b| b - a)
    }

    fn mul<T: Copy + std::ops::Mul>(&mut self) {
        self.binary_op::<T, _, _>(|a, b| b * a)
    }

    fn div<T: Copy + std::ops::Div + PartialEq>(&mut self, zero: T) {
        unsafe {
            let data1: T = self.read_stack(self.sp);
            if data1 == zero {
                panic!("divided by zero");
            }

            self.sp += 4;
            let data2: T = self.read_stack(self.sp);
            self.write_stack(self.sp, data2 / data1);
        }
    }

    fn xmod<T: Copy + std::ops::Rem + PartialEq>(&mut self, zero: T) {
        unsafe {
            let data1: T = self.read_stack(self.sp);
            if data1 == zero {
                panic!("divided by zero");
            }

            self.sp += 4;
            let data2: T = self.read_stack(self.sp);
            self.write_stack(self.sp, data2 % data1);
        }
    }

    fn neg<T: Copy + std::ops::Neg>(&mut self) {
        self.unary_op::<T, _, _>(|a| -a);
    }

    fn cmp<T: Copy + PartialOrd>(&mut self) {
        self.binary_op::<T, _, _>(|a, b| {
            if b.gt(&a) {
                1
            } else if a.gt(&b) {
                -1
            } else {
                0
            }
        })
    }

    fn inc<T: Copy + std::ops::Add>(&mut self, one: T) {
        unsafe {
            let pos: u32 = self.read_stack(self.sp);
            let data: T = self.read_stack(pos as usize);
            self.write_stack(pos as usize, data + one);
        }
    }

    fn dec<T: Copy + std::ops::Sub>(&mut self, one: T) {
        unsafe {
            let pos: u32 = self.read_stack(self.sp);
            let data: T = self.read_stack(pos as usize);
            self.write_stack(pos as usize, data - one);
        }
    }

    fn i2f(&mut self) {
        self.unary_op::<i32, _, _>(|a| a as f32);
    }

    fn f2i(&mut self) {
        self.unary_op::<f32, _, _>(|a| a as i32);
    }

    fn bnot(&mut self) {
        self.unary_op::<u32, _, _>(|a| !a);
    }

    fn band(&mut self) {
        self.binary_op::<u32, _, _>(|a, b| b & a)
    }

    fn bor(&mut self) {
        self.binary_op::<u32, _, _>(|a, b| b | a)
    }

    fn bxor(&mut self) {
        self.binary_op::<u32, _, _>(|a, b| b ^ a)
    }

    fn bsll(&mut self) {
        self.binary_op::<u32, _, _>(|a, b| b << (a & 0xff))
    }

    fn bsrl(&mut self) {
        self.binary_op::<u32, _, _>(|a, b| b >> (a & 0xff))
    }

    fn bsra(&mut self) {
        self.binary_op::<i32, _, _>(|a, b| b >> (a & 0xff))
    }

    fn ui2f(&mut self) {
        self.unary_op::<u32, _, _>(|a| a as f32);
    }

    fn f2ui(&mut self) {
        self.unary_op::<f32, _, _>(|a| a as u32);
    }

    fn sb(&mut self) {
        self.unary_op::<i32, _, _>(|a| (a as i8) as i32);
    }

    fn sw(&mut self) {
        self.unary_op::<i32, _, _>(|a| (a as i16) as i32);
    }

    fn ub(&mut self) {
        self.unary_op::<u32, _, _>(|a| (a as u8) as u32);
    }

    fn uw(&mut self) {
        self.unary_op::<u32, _, _>(|a| (a as u16) as u32);
    }

    fn wrt1(&mut self) {
        self.binary_op::<u32, _, _>(|a, b| (b & 0xFFFFFF00) + (a & 0xFF));
    }

    fn wrt2(&mut self) {
        self.binary_op::<u32, _, _>(|a, b| (b & 0xFFFF0000) + (a & 0xFFFF));
    }

    fn push_zero(&mut self) {
        self.sp -= 4;
        unsafe {
            self.write_stack(self.sp, 0u32);
        }
    }

    fn copy(&mut self, count: u16) {
        unsafe {
            let dst: u32 = self.read_stack(self.sp);
            self.sp += 4;
            let src: u32 = self.read_stack(self.sp);

            for i in 0..count {
                let data: u32 = self.read_stack(src as usize + i as usize);
                self.write_stack(dst as usize + i as usize, data);
            }
        }
    }

    fn set8(&mut self, data: u64) {
        unsafe {
            self.sp -= 8;
            self.write_stack(self.sp, data);
        }
    }

    fn rd8(&mut self) {
        unsafe {
            let pos: u32 = self.read_stack(self.sp);
            self.sp += 4;
            let data: u64 = self.read_stack(self.sp);
            self.write_stack(pos as usize, data);
        }
    }

    fn wrt8(&mut self) {
        unsafe {
            let pos: u32 = self.read_stack(self.sp);
            self.sp -= 4;
            let data: u64 = self.read_stack(pos as usize);
            self.write_stack(self.sp, data);
        }
    }

    fn d2i(&mut self) {
        unsafe {
            let data: f64 = self.read_stack(self.sp);
            self.sp += 4;
            self.write_stack(self.sp, data as i32);
        }
    }

    fn d2ui(&mut self) {
        unsafe {
            let data: f64 = self.read_stack(self.sp);
            self.sp += 4;
            self.write_stack(self.sp, data as u32);
        }
    }

    fn d2f(&mut self) {
        unsafe {
            let data: f64 = self.read_stack(self.sp);
            self.sp += 4;
            self.write_stack(self.sp, data as f32);
        }
    }

    fn x2d<T: Copy + std::convert::Into<f64>>(&mut self) {
        unsafe {
            let data: i32 = self.read_stack(self.sp);
            self.sp += 8;
            self.sp -= std::mem::size_of::<T>();
            self.write_stack(self.sp, data as f64);
        }
    }

    fn jmpp(&mut self) {
        unsafe {
            let data: i32 = self.read_stack(self.sp);
            self.sp += 4;
            self.context.as_mut().unwrap().pc += (8 * data) as usize;
        }
    }

    fn sret4(&mut self) {
        unsafe {
            let data: u32 = self.read_stack(self.sp);
            self.sp += 4;
            self.r1 = data;
        }
    }

    fn sret8(&mut self) {
        unsafe {
            self.r1 = self.read_stack(self.sp);
            self.sp += 4;
            self.r2 = self.read_stack(self.sp);
            self.sp += 4;
        }
    }

    fn rret4(&mut self) {
        unsafe {
            self.sp -= 4;
            self.write_stack(self.sp, self.r1);
        }
    }

    fn rret8(&mut self) {
        unsafe {
            self.sp -= 4;
            self.write_stack(self.sp, self.r2);
            self.sp -= 4;
            self.write_stack(self.sp, self.r1);
        }
    }

    fn js_jgez(&mut self, offset: i32) {
        self.j(offset, |data| data >= 0);
    }

    fn jns_jlz(&mut self, offset: i32) {
        self.j(offset, |data| data < 0);
    }

    fn jp_jlez(&mut self, offset: i32) {
        self.j(offset, |data| data <= 0);
    }

    fn jnp_jgz(&mut self, offset: i32) {
        self.j(offset, |data| data > 0);
    }

    fn cmpi<T: Copy + PartialOrd>(&mut self, rhs: T) {
        unsafe {
            let data: T = self.read_stack(self.sp);
            self.write_stack(
                self.sp,
                if rhs.gt(&data) {
                    1
                } else if data.gt(&rhs) {
                    -1
                } else {
                    0
                },
            );
        }
    }

    fn addi<T: Copy + std::ops::Add>(&mut self, rhs: T) {
        unsafe {
            let data: T = self.read_stack(self.sp);
            self.write_stack(self.sp, data + rhs);
        }
    }

    fn subi<T: Copy + std::ops::Sub>(&mut self, rhs: T) {
        unsafe {
            let data: T = self.read_stack(self.sp);
            self.write_stack(self.sp, data - rhs);
        }
    }

    fn muli<T: Copy + std::ops::Mul>(&mut self, rhs: T) {
        unsafe {
            let data: T = self.read_stack(self.sp);
            self.write_stack(self.sp, data * rhs);
        }
    }

    fn pga(&mut self, index: i32) {
        let data = if index > 0 {
            let context = self.context.as_ref().unwrap();
            let module = context.module.borrow();
            module.globals[index as usize]
        } else {
            let context = self.g.borrow();
            context.get_global((-index - 1) as usize)
        };

        self.sp -= 4;

        unsafe {
            self.write_stack(self.sp, data);
        }
    }

    fn movga4(&mut self, index: i32) {
        let data: u32 = unsafe { self.read_stack(self.sp) };

        if index > 0 {
            let context = self.context.as_mut().unwrap();
            let mut module = context.module.borrow_mut();
            module.globals[index as usize] = data;
        } else {
            let mut context = self.g.borrow_mut();
            context.set_global((-index - 1) as usize, data);
        };

        self.sp += 4;
    }

    fn str(&mut self, index: u16) {
        let module = self.context.as_ref().unwrap().module.clone();
        let module_ref = module.borrow();
        let string = &module_ref.strings[index as usize];
        unsafe {
            self.sp -= 4;
            self.write_stack(self.sp, index as u32);
            self.sp -= 4;
            self.write_stack(self.sp, string.len() as u32);
        }
    }

    #[inline]
    fn j<F: Fn(i32) -> bool>(&mut self, offset: i32, f: F) {
        unsafe {
            let data: i32 = self.read_stack(self.sp);
            if f(data) {
                self.context.as_mut().unwrap().pc += offset as usize;
            }
        }
    }

    #[inline]
    fn unary_op<T: Copy, U, F: Fn(T) -> U>(&mut self, f: F) {
        unsafe {
            let data: T = self.read_stack(self.sp);
            self.write_stack(self.sp, f(data));
        }
    }

    #[inline]
    fn binary_op<T: Copy, U, F: Fn(T, T) -> U>(&mut self, f: F) {
        unsafe {
            let data: T = self.read_stack(self.sp);
            self.sp += std::mem::size_of::<T>();
            let data2: T = self.read_stack(self.sp);
            self.sp += std::mem::size_of::<T>();
            self.sp -= std::mem::size_of::<U>();
            self.write_stack(self.sp, f(data, data2));
        }
    }

    #[inline]
    unsafe fn write_stack<T>(&mut self, pos: usize, data: T) {
        if std::mem::size_of::<T>() == 8 {
            let data_bytes: &[u8; 8] = std::mem::transmute(&data as *const _ as *const u8);
            for i in 0..8 {
                self.stack[pos + i] = data_bytes[i];
            }
        } else {
            *(&mut self.stack[pos] as *mut u8 as *mut T) = data;
        }
    }

    #[inline]
    unsafe fn read_stack<T: Copy>(&self, pos: usize) -> T {
        if std::mem::size_of::<T>() == 8 {
            let mut data_bytes = [0u8; 8];
            for i in 0..8 {
                data_bytes[i] = self.stack[pos + i];
            }

            *(&data_bytes as *const u8 as *const T)
        } else {
            *(&self.stack[pos] as *const u8 as *const T)
        }
    }

    fn debug_update_module(&mut self) {
        #[cfg(enable_debug)]
        {
            let _ = self.debug_client.notify(Notification::ModuleChanged {
                module: self
                    .context
                    .as_ref()
                    .and_then(|f| Some(f.module.borrow().clone())),
                function: self
                    .context
                    .as_ref()
                    .and_then(|f| Some(f.function_index as u32))
                    .unwrap_or(0),
            });

            let _ = self
                .debug_client
                .notify(Notification::GlobalFunctionsChanged(
                    self.g
                        .borrow()
                        .functions
                        .iter()
                        .map(|f| f.name.clone())
                        .collect(),
                ));
        }
    }

    fn debug_update_context(&mut self) {
        #[cfg(enable_debug)]
        {
            let _ = self
                .debug_client
                .notify(Notification::ObjectsChanged(self.heap.clone()));
            let _ = self.debug_client.notify(Notification::RegisterChanged {
                pc: self.context.as_ref().and_then(|f| Some(f.pc)).unwrap_or(0),
                sp: self.sp,
                fp: self.fp,
                r1: self.r1,
                r2: self.r2,
                object_register: self.robj,
            });

            let _ = self
                .debug_client
                .notify(Notification::StackChanged(self.stack.clone()));
        }
    }

    fn wait_for_action(&mut self) {
        #[cfg(enable_debug)]
        {
            let _ = self.debug_client.call(Request::WaitForAction);
        }
    }
}

pub(crate) mod data_read {
    use byteorder::{LittleEndian, ReadBytesExt};

    pub(crate) fn u16(inst: &[u8], pc: &mut usize) -> u16 {
        *pc += 2;
        (&inst[*pc - 2..*pc]).read_u16::<LittleEndian>().unwrap()
    }

    pub(crate) fn i16(inst: &[u8], pc: &mut usize) -> i16 {
        *pc += 2;
        (&inst[*pc - 2..*pc]).read_i16::<LittleEndian>().unwrap()
    }

    pub(crate) fn i32(inst: &[u8], pc: &mut usize) -> i32 {
        *pc += 4;
        (&inst[*pc - 4..*pc]).read_i32::<LittleEndian>().unwrap()
    }

    pub(crate) fn u32(inst: &[u8], pc: &mut usize) -> u32 {
        *pc += 4;
        (&inst[*pc - 4..*pc]).read_u32::<LittleEndian>().unwrap()
    }

    pub(crate) fn f32(inst: &[u8], pc: &mut usize) -> f32 {
        *pc += 4;
        (&inst[*pc - 4..*pc]).read_f32::<LittleEndian>().unwrap()
    }

    pub(crate) fn u64(inst: &[u8], pc: &mut usize) -> u64 {
        *pc += 8;
        (&inst[*pc - 8..*pc]).read_u64::<LittleEndian>().unwrap()
    }
}
