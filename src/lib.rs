#![feature(alloc)]

#![no_std]

extern crate byteorder;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate alloc;

mod rlp;
mod elastic_array;
mod hexutil;
