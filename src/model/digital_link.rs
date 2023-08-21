use nom::combinator::all_consuming;
use url::Url;

use super::{error::Error, gs1path, Gs1Path};

pub struct DigitalLink {
	gs1_path: Gs1Path,
	data_attributes: DataAttributes,
}

impl DigitalLink {
	pub fn try_from_str(s: &str) -> Result<Self, Error> {
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

#[non_exhaustive]
pub struct DataAttributes {
	net_weight_vmti: Option<String>,
}
