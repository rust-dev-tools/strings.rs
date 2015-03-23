//TODO licence

#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(unicode)]
#![feature(rustc_private)]
#![feature(collections)]
#![feature(core)]

// TODO use crates.io log instead
#[macro_use]
extern crate log;

// TODO should probably expose data structures, not the modules
pub mod rope;
pub mod src_rope;
pub mod string_buffer;
