#![doc = include_str!("../README.md")]

pub mod copy;
pub mod discard;
pub mod read;

pub use self::copy::*;
pub use self::discard::*;
pub use self::read::*;
