#![feature(proc_macro_hygiene)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

#[macro_use]
extern crate phf;

pub mod token;
#[macro_use]
mod macros;
mod identifier;
mod number;
mod string;
mod state;
mod equivalence;
pub mod error;
/// Module for efficient string representation
pub mod internship {
    extern crate internship;
    pub use internship::*;
}