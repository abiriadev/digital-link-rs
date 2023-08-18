use url::{ParseError, Url};

#[non_exhaustive]
enum Gs1Path {
	GtinPath {
		gtin: String,
		cpv: Option<String>,
		lot: Option<String>,
		ser: Option<String>,
	},
}

#[non_exhaustive]
struct DataAttributes {
	net_weight_vmti: Option<String>,
}

struct DigitalLink {
	gs1_path: Gs1Path,
	data_attributes: DataAttributes,
}

enum Error {
	ParseError(ParseError),
	ValidationError(ValidationError),
}

enum ValidationError {
	PrimaryIdentificationKeyNotFound,
	PrimaryIdentificationKeyValueNotFound,
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
			"01" => Gs1Path::GtinPath {
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
