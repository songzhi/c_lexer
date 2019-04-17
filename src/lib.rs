#![feature(proc_macro_hygiene)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

#[macro_use]
extern crate phf;

mod token;
#[macro_use]
mod macros;
mod identifier;
