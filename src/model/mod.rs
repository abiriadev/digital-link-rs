pub use data_attributes::DataAttributes;
pub use digital_link::{DigitalLink, DigitalLinkWasm};
pub use gs1path::Gs1Path;

use crate::parser::gs1path;

mod data_attributes;
mod digital_link;
mod error;
mod gs1path;
mod wasm;
