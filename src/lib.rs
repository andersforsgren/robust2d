#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate matches;
extern crate num;

pub mod ffi;
pub mod arithmetic;
pub mod primitives;
