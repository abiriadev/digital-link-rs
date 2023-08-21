pub use gs1path::Gs1Path;
use nom::combinator::all_consuming;
use thiserror::Error;
use url::{ParseError, Url};

use crate::parser::gs1path;

mod gs1path;

#[non_exhaustive]
pub struct DataAttributes {
	net_weight_vmti: Option<String>,
}

pub struct DigitalLink {
	gs1_path: Gs1Path,
	data_attributes: DataAttributes,
}

#[derive(Error, Debug)]
pub enum Error {
	#[error("url parsing failed: {0}")]
	UrlParseError(#[from] ParseError),
	#[error("gs1 path parsing failed: {0}")]
	Gs1PathParseError(#[from] nom::Err<String>),
}

impl DigitalLink {
	fn try_from_str(s: &str) -> Result<Self, Error> {
		let url = Url::parse(s);

		let Ok(url) = Url::parse(s) else { return Err(Error::UrlParseError(url.unwrap_err())); };

		let res = all_consuming(gs1path)(url.path());

		match res {
			Ok((_, gs1_path)) => Ok(Self {
				gs1_path,
				data_attributes: DataAttributes {
					net_weight_vmti: None,
				},
			}),
			Err(err) => Err(Error::Gs1PathParseError(
				err.map(|e| e.to_string()),
			)),
		}
	}
}
