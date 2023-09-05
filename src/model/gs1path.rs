use std::fmt::{self, Display, Formatter};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[non_exhaustive]
#[derive(Debug)]
#[wasm_bindgen]
pub enum Gs1Path {
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

impl Gs1Path {
	fn to_code(&self) -> &'static str {
		match self {
			Gs1Path::Gtin { .. } => "01",
			Gs1Path::Itip { .. } => "8006",
			Gs1Path::Gmn(_) => "8013",
			Gs1Path::Cpid { .. } => "8010",
			Gs1Path::Gln { .. } => "414",
			Gs1Path::PayTo(_) => "415",
			Gs1Path::PartyGln(_) => "417",
			Gs1Path::Gsrnp { .. } => "8017",
			Gs1Path::Gsrn { .. } => "8018",
			Gs1Path::Gcn(_) => "255",
			Gs1Path::Sscc(_) => "00",
			Gs1Path::Gdti(_) => "253",
			Gs1Path::Ginc(_) => "401",
			Gs1Path::Gsin(_) => "402",
			Gs1Path::Grai(_) => "8003",
			Gs1Path::Giai(_) => "8004",
			Gs1Path::Upui { .. } => "01",
			Gs1Path::Eoid { .. } => "417",
			Gs1Path::Fid { .. } => "414",
			Gs1Path::Mid { .. } => "8004",
		}
	}

	fn stringify_primary(&self) -> String {
		format!("/{}/{}", self.to_code(), match self {
			Gs1Path::Gtin { gtin, .. } => gtin,
			Gs1Path::Itip { itip, .. } => itip,
			Gs1Path::Gmn(gmn) => gmn,
			Gs1Path::Cpid { cpid, .. } => cpid,
			Gs1Path::Gln { gln, .. } => gln,
			Gs1Path::PayTo(pay_to) => pay_to,
			Gs1Path::PartyGln(party_gln) => party_gln,
			Gs1Path::Gsrnp { gsrnp, .. } => gsrnp,
			Gs1Path::Gsrn { gsrn, .. } => gsrn,
			Gs1Path::Gcn(gcn) => gcn,
			Gs1Path::Sscc(sscc) => sscc,
			Gs1Path::Gdti(gdti) => gdti,
			Gs1Path::Ginc(ginc) => ginc,
			Gs1Path::Gsin(gsin) => gsin,
			Gs1Path::Grai(grai) => grai,
			Gs1Path::Giai(giai) => giai,
			Gs1Path::Upui { gtin, .. } => gtin,
			Gs1Path::Eoid { party_gln, .. } => party_gln,
			Gs1Path::Fid { gln, .. } => gln,
			Gs1Path::Mid { giai, .. } => giai,
		})
	}

	fn stringify_cpv(cpv: &Option<String>) -> String {
		cpv.clone().map_or_else(
			|| "".to_owned(),
			|cpv| format!("/22/{cpv}"),
		)
	}

	fn stringify_lot(lot: &Option<String>) -> String {
		lot.clone().map_or_else(
			|| "".to_owned(),
			|lot| format!("/10/{lot}"),
		)
	}

	fn stringify_ser(ser: &Option<String>) -> String {
		ser.clone().map_or_else(
			|| "".to_owned(),
			|ser| format!("/21/{ser}"),
		)
	}

	fn stringify_cpsn(cpsn: &Option<String>) -> String {
		cpsn.clone().map_or_else(
			|| "".to_owned(),
			|cpsn| format!("/8011/{cpsn}"),
		)
	}

	fn stringify_glnx(glnx: &Option<String>) -> String {
		glnx.clone().map_or_else(
			|| "".to_owned(),
			|glnx| format!("/254/{glnx}"),
		)
	}

	fn stringify_refno(refno: &Option<String>) -> String {
		refno.clone().map_or_else(
			|| "".to_owned(),
			|refno| format!("/8020/{refno}"),
		)
	}

	fn stringify_srin(srin: &Option<String>) -> String {
		srin.clone().map_or_else(
			|| "".to_owned(),
			|srin| format!("/8019/{srin}"),
		)
	}

	fn stringify_tpx(tpx: &String) -> String { format!("/235/{tpx}") }

	fn stringify_uic_ext(uic_ext: &String) -> String {
		format!("/7040/{uic_ext}")
	}

	fn stringify_qualifiers(&self) -> String {
		match self {
			Gs1Path::Gtin { cpv, lot, ser, .. } => format!(
				"{}{}{}",
				Self::stringify_cpv(cpv),
				Self::stringify_lot(lot),
				Self::stringify_ser(ser)
			),
			Gs1Path::Itip { cpv, lot, ser, .. } => format!(
				"{}{}{}",
				Self::stringify_cpv(cpv),
				Self::stringify_lot(lot),
				Self::stringify_ser(ser)
			),
			Gs1Path::Cpid { cpsn, .. } => Self::stringify_cpsn(cpsn),
			Gs1Path::Gln { glnx, .. } => Self::stringify_glnx(glnx),
			Gs1Path::Gsrnp { srin, .. } => Self::stringify_srin(srin),
			Gs1Path::Gsrn { srin, .. } => Self::stringify_srin(srin),
			Gs1Path::Upui { tpx, .. } => Self::stringify_tpx(tpx),
			Gs1Path::Eoid { uic_ext, .. } => Self::stringify_uic_ext(uic_ext),
			Gs1Path::Fid { uic_ext, .. } => Self::stringify_uic_ext(uic_ext),
			Gs1Path::Mid { uic_ext, .. } => Self::stringify_uic_ext(uic_ext),

			Gs1Path::Gmn(_)
			| Gs1Path::PayTo(_)
			| Gs1Path::PartyGln(_)
			| Gs1Path::Gcn(_)
			| Gs1Path::Sscc(_)
			| Gs1Path::Gdti(_)
			| Gs1Path::Ginc(_)
			| Gs1Path::Gsin(_)
			| Gs1Path::Grai(_)
			| Gs1Path::Giai(_) => "".to_owned(),
		}
	}

	fn stringify(&self) -> String {
		format!(
			"{}{}",
			self.stringify_primary(),
			self.stringify_qualifiers()
		)
	}
}

impl Display for Gs1Path {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.stringify())
	}
}
