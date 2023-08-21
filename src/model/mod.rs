pub use gs1path::Gs1Path;
use thiserror::Error;
use url::{ParseError, Url};

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
	ParseError(#[from] ParseError),
	#[error("validation failed: {0}")]
	ValidationError(ValidationError),
}

#[derive(Error, Debug)]
pub enum ValidationError {
	#[error("can't find primary identification key")]
	PrimaryIdentificationKeyNotFound,
	#[error("can't find primary identification key value")]
	PrimaryIdentificationKeyValueNotFound,
	#[error("primary identification key is invalid")]
	InvalidPrimaryIdentificationKey,
}

impl DigitalLink {
	fn try_from_str(s: &str) -> Result<Self, Error> {
		let url = Url::parse(s);

		let Ok(url) = url else { return Err(Error::ParseError(url.unwrap_err())); };

		let mut seg_it = url
			.path_segments()
			.ok_or(Error::ValidationError(
				ValidationError::PrimaryIdentificationKeyNotFound,
			))?;

		let primary_id = seg_it
			.next()
			.ok_or(Error::ValidationError(
				ValidationError::PrimaryIdentificationKeyNotFound,
			))?;

		let primary_id_value = seg_it
			.next()
			.ok_or(Error::ValidationError(
				ValidationError::PrimaryIdentificationKeyValueNotFound,
			))?;

		let gs1_path = match primary_id {
			"01" => Gs1Path::Gtin {
				gtin: primary_id_value.to_owned(),
				cpv: None,
				lot: None,
				ser: None,
			},
			_ => unimplemented!(),
		};

		Ok(Self {
			gs1_path,
			data_attributes: DataAttributes {
				net_weight_vmti: None,
			},
		})
	}
}
