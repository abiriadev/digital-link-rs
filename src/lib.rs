//! # Digital Link for Rust
//!
//! WIP implementation of GS 1 Digital Link standard for Rust
//! ```
#![doc = include_str!("../examples/parse.rs")]
//! ```

mod model;
mod parser;

pub use model::{DataAttributes, DigitalLink, Gs1Path};
