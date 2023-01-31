//! The EVM Toolkit Disassembler.
//!
//! You can find more information about the command-line tools in
//! [The ETK Book](https://quilt.github.io/etk/).
#![deny(unsafe_code)]
//#![deny(missing_docs)]
#![deny(unreachable_pub)]
#![deny(missing_debug_implementations)]

#[path = "bin/disease/selectors.rs"]
mod selectors;
pub use crate::selectors::DisplayOp;

pub mod blocks;
pub mod sym;
