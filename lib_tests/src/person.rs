use std::convert::TryInto;

use protobuf_gen::ProtobufGen;

use crate::city::City;

#[derive(Debug, Clone, ProtobufGen, Arbitrary, PartialEq)]
#[protobuf_gen(proxy_mod = "crate::proxy")]
pub struct Dummy {
    pub id: u32,
}

#[derive(Debug, Default, Clone, ProtobufGen, Arbitrary, PartialEq)]
#[protobuf_gen(proxy_mod = "crate::proxy")]
pub struct Designer {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, ProtobufGen, Arbitrary, PartialEq)]
#[protobuf_gen(proxy_mod = "crate::proxy")]
pub enum Job {
    None,
    Programmer { skill: String, grade: u8 },
    Designer { designer: Designer },
}

impl Default for Job {
    fn default() -> Self {
        Job::None
    }
}

#[derive(Debug, Clone, ProtobufGen, Arbitrary, PartialEq)]
#[protobuf_gen(proxy_mod = "crate::proxy")]
pub enum AreaCode {
    Seoul,
    Seongnam,
    Jinhae,
}

#[derive(Debug, Default, Clone, Arbitrary, PartialEq)]
pub struct NumberBuffer(Vec<u8>);

impl Into<Vec<u8>> for NumberBuffer {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for NumberBuffer {
    fn from(vs: Vec<u8>) -> Self {
        NumberBuffer(vs)
    }
}

#[derive(Debug, Clone, ProtobufGen, Arbitrary, PartialEq)]
#[protobuf_gen(proxy_mod = "crate::proxy")]
pub struct Person {
    pub(crate) _inner: i32,
    pub id: u8,
    #[protobuf_gen(substitute = "bytes")]
    pub number: NumberBuffer,
    pub hobbies: Vec<u32>,
    pub job: Job,
    pub city: City,
    pub area_code: AreaCode,
}
