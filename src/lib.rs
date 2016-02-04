//TODO licence

// TODO use crates.io log instead
#[macro_use]
extern crate log;

// TODO should probably expose data structures, not the modules
pub mod string_buffer;

mod util;
mod ropes;

pub mod rope {
    pub use ::ropes::RopeSlice;
    pub use ::ropes::Rope;
}

pub mod src_rope {
    pub use ::ropes::SrcRopeSlice as RopeSlice;
    pub use ::ropes::SrcRope as Rope;
}
