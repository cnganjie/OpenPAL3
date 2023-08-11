use std::{borrow::Cow, str::Utf8Error};

use binrw::{binrw, BinRead, BinWrite};

#[binrw]
#[brw(little)]
#[derive(Clone)]
pub struct SizedString {
    #[bw(calc(string.len() as u32))]
    size: u32,

    #[br(count = size)]
    string: Vec<u8>,
}

impl SizedString {
    pub fn data(&self) -> &[u8] {
        &self.string
    }

    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        let slice = if self.string.last() == Some(&0) {
            &self.string[..self.string.len() - 1]
        } else {
            &self.string
        };

        std::str::from_utf8(slice)
    }
}

impl<T: AsRef<str>> From<T> for SizedString {
    fn from(value: T) -> Self {
        Self {
            string: value.as_ref().as_bytes().to_vec(),
        }
    }
}

impl From<SizedString> for String {
    fn from(value: SizedString) -> Self {
        String::from_utf8_lossy(&value.string).to_string()
    }
}

impl std::fmt::Debug for SizedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SizedString(\"{}\")",
            String::from_utf8_lossy(&self.string)
        )
    }
}

impl PartialEq<&str> for SizedString {
    fn eq(&self, other: &&str) -> bool {
        String::from_utf8_lossy(&self.string) == *other
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Pal4NodeSection {
    version1: u32,
    version2: u32,

    #[br(if(version1 == 0 || (version1 < 2 && version2 < 2)))]
    pub root: Option<Pal4Node>,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Pal4Node {
    pub name: SizedString,
    pub property_count: u32,

    #[br(count = property_count)]
    pub properties: Vec<Pal4NodeProperty>,

    pub children_count: u32,

    #[br(count = children_count)]
    pub children: Vec<Box<Pal4Node>>,
}

impl Pal4Node {
    pub fn get_child_by_name(&self, name: &str) -> Option<&Pal4Node> {
        self.children.iter().find(|c| c.name == name).map(|c| &**c)
    }

    pub fn get_property_by_name(&self, name: &str) -> Option<&Pal4NodeProperty> {
        self.properties
            .iter()
            .find(|p| *p.name() == name)
            .map(|p| p)
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub enum Pal4NodeProperty {
    #[br(magic(2u32))]
    Float(Pal4NodePropertyValue<f32>),

    #[br(magic(3u32))]
    String(Pal4NodePropertyValue<SizedString>),
}

impl Pal4NodeProperty {
    pub fn name(&self) -> &SizedString {
        match self {
            Self::Float(v) => &v.name,
            Self::String(v) => &v.name,
        }
    }

    pub fn f32(&self) -> Option<f32> {
        if let Self::Float(v) = self {
            Some(v.value)
        } else {
            None
        }
    }

    pub fn string(&self) -> Option<Cow<str>> {
        if let Self::String(v) = self {
            Some(String::from_utf8_lossy(v.value.data()))
        } else {
            None
        }
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Pal4NodePropertyValue<
    T: for<'a> BinRead<Args<'a> = ()> + for<'a> BinWrite<Args<'a> = ()>,
> {
    pub name: SizedString,
    pub value: T,
}