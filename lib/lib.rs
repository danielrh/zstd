#![feature(libc)]
#![feature(offset_to)]
#![feature(const_slice_as_ptr)]
#![feature(used)]
#![feature(extern_types)]
extern crate libc;
pub mod decompress;
pub mod compress;
pub mod common;
