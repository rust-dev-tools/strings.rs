//TODO licence

#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(rustc_private)]
#![feature(core)]
#![feature(str_char)]
#![feature(unicode)]

// TODO use crates.io log instead
#[macro_use]
extern crate log;

// TODO should probably expose data structures, not the modules
pub mod rope;
pub mod src_rope;
pub mod string_buffer;
