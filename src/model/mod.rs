pub use gs1path::Gs1Path;
use thiserror::Error;
use url::ParseError;

use crate::parser::gs1path;

mod digital_link;
mod gs1path;

#[non_exhaustive]
pub struct DataAttributes {
	net_weight_vmti: Option<String>,
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("url parsing failed: {0}")]
	UrlParseError(#[from] ParseError),
	#[error("gs1 path parsing failed: {0}")]
	Gs1PathParseError(#[from] nom::Err<String>),
}
