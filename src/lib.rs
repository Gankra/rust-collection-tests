#![feature(macro_rules)]

extern crate test;

#[macro_escape]
pub mod utils;
pub mod collection;
pub mod map;
pub mod priorityqueue;