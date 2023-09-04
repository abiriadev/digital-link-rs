use std::fmt::Display;

use nom::combinator::all_consuming;
use url::Url;

use super::{data_attributes::DataAttributes, error::Error, gs1path, Gs1Path};
use crate::parser::parse_data_attribute;

pub struct DigitalLink {
	gs1_path: Gs1Path,
	data_attributes: DataAttributes,
	base_url: String,
}

impl DigitalLink {
	pub fn try_from_str(s: &str) -> Result<DigitalLink, Error> {
		let url = Url::parse(s);

		let Ok(url) = url else { return Err(Error::UrlParseError(url.unwrap_err())); };

		let res = all_consuming(gs1path)(url.path());

		match res {
			Ok((_, gs1_path)) => {
				let mut data_attributes = DataAttributes::default();

				for query_pair in url.query_pairs() {
					parse_data_attribute(&mut data_attributes, query_pair);
				}

				Ok(Self {
					gs1_path,
					data_attributes,
					base_url: url.host_str().unwrap().to_owned(),
				})
			},
			Err(err) => Err(Error::Gs1PathParseError(
				err.map(|e| e.to_string()),
			)),
		}
	}
}

impl Display for DigitalLink {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"http://{}{}",
			self.base_url, self.gs1_path
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn serialized_result_should_be_same_as_input() {
		let url =
			"http://digital-link.aipim.io/01/09520123456788/10/ABC1/21/12345";

		let dl = DigitalLink::try_from_str(url).unwrap();

		assert_eq!(url, dl.to_string());
	}
}
