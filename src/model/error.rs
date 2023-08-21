use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum Error {
	#[error("url parsing failed: {0}")]
	UrlParseError(#[from] ParseError),
	#[error("gs1 path parsing failed: {0}")]
	Gs1PathParseError(#[from] nom::Err<String>),
}
