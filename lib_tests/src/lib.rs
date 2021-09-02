#[macro_use]
extern crate proptest_derive;

pub mod city;
pub mod person;
pub mod stable_vec;

pub mod proxy {
    include!("../proxy/yellow_book.rs");
}
