use thiserror::Error;
use url::{ParseError, Url};

#[non_exhaustive]
enum Gs1Path {
	Gtin {
		gtin: String,
		cpv: Option<String>,
		lot: Option<String>,
		ser: Option<String>,
	},
	Itip {
		itip: String,
		cpv: Option<String>,
		lot: Option<String>,
		ser: Option<String>,
	},
	Gmn(String),
	Cpid {
		cpid: String,
		cpsn: Option<String>,
	},
	Gln {
		gln: String,
		glnx: Option<String>,
	},
	PayTo(String),
	PartyGln(String),
	Gsrnp {
		gsrnp: String,
		srin: Option<String>,
	},
	Gsrn {
		gsrn: String,
		srin: Option<String>,
	},
	Gcn(String),
	Sscc(String),
	Gdti(String),
	Ginc(String),
	Gsin(String),
	Grai(String),
	Giai(String),
	Upui {
		gtin: String,
		tpx: String,
	},
	Eoid {
		party_gln: String,
		uic_ext: String,
	},
	Fid {
		gln: String,
		uic_ext: String,
	},
	Mid {
		giai: String,
		uic_ext: String,
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

#[derive(Error, Debug)]
enum Error {
	#[error("url parsing failed: {0}")]
	ParseError(#[from] ParseError),
	#[error("validation failed: {0}")]
	ValidationError(ValidationError),
}

#[derive(Error, Debug)]
enum ValidationError {
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
