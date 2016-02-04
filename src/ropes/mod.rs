#[macro_use]
mod macros;

mod rope;
mod src_rope;

pub use self::rope::Rope;
pub use self::rope::RopeSlice;

pub use self::src_rope::Rope as SrcRope;
pub use self::src_rope::RopeSlice as SrcRopeSlice;
