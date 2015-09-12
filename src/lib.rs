//TODO licence

#![allow(raw_pointer_derive)]

// TODO use crates.io log instead
#[macro_use]
extern crate log;

// TODO should probably expose data structures, not the modules
pub mod rope;
pub mod src_rope;
pub mod string_buffer;
mod util;