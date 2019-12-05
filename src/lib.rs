//! Read and parse Id PAK data from files or any other source.

mod fileentry;
mod header;
mod pak;

pub mod errors;
pub use crate::pak::{open, IdPak, IdPakLoadResult, IdPakReader};
