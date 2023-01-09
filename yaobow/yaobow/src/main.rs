use std::{cell::RefCell, io::BufRead, rc::Rc};

use fileformats::dff::{list_chunks, read_dff};
use openpal4::{
    application::OpenPal4Application,
    scripting::{global_context::ScriptGlobalContext, module::ScriptModule, vm::ScriptVm},
};

mod openpal4;

pub fn main() {
    /*let mut app = OpenPal4Application::create("OpenPAL4");
    app.initialize();
    app.run();*/

    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    let data = std::fs::read("F:\\PAL4\\gamedata\\PALActor\\101\\101.dff").unwrap();
    let chunks = read_dff(&data).unwrap();
    println!("{}", serde_json::to_string(&chunks).unwrap());

    /*let content = std::fs::read("F:\\PAL4\\gamedata\\script\\script.csb").unwrap();

    let module = ScriptModule::load_from_buffer(&content).unwrap();
    println!("{}", serde_json::to_string(&module).unwrap());

    let context = Rc::new(RefCell::new(ScriptGlobalContext::new()));
    let mut vm = ScriptVm::new(context);
    let module = Rc::new(RefCell::new(module));
    vm.set_module(module);
    vm.set_function(0);
    vm.execute();*/
}