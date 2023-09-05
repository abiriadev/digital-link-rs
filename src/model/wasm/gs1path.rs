use std::fmt::Debug;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::Gs1Path;

#[derive(Debug, Clone, Default)]
#[wasm_bindgen(getter_with_clone)]
pub struct Gs1PathWasm {
	pub gtin: Option<Gtin>,
	pub itip: Option<Itip>,
	pub gmn: Option<Gmn>,
	pub cpid: Option<Cpid>,
	pub gln: Option<Gln>,
	pub pay_to: Option<PayTo>,
	pub party_gln: Option<PartyGln>,
	pub gsrnp: Option<Gsrnp>,
	pub gsrn: Option<Gsrn>,
	pub gcn: Option<Gcn>,
	pub sscc: Option<Sscc>,
	pub gdti: Option<Gdti>,
	pub ginc: Option<Ginc>,
	pub gsin: Option<Gsin>,
	pub grai: Option<Grai>,
	pub giai: Option<Giai>,
	pub upui: Option<Upui>,
	pub eoid: Option<Eoid>,
	pub fid: Option<Fid>,
	pub mid: Option<Mid>,
}

impl From<Gs1Path> for Gs1PathWasm {
	fn from(value: Gs1Path) -> Self {
		let mut gs1path_w = Self::default();

		match value {
			Gs1Path::Gtin {
				gtin,
				cpv,
				lot,
				ser,
			} =>
				gs1path_w.gtin = Some(Gtin {
					gtin,
					cpv,
					lot,
					ser,
				}),
			Gs1Path::Itip {
				itip,
				cpv,
				lot,
				ser,
			} =>
				gs1path_w.itip = Some(Itip {
					itip,
					cpv,
					lot,
					ser,
				}),
			Gs1Path::Gmn(v) => gs1path_w.gmn = Some(Gmn(v)),
			Gs1Path::Cpid { cpid, cpsn } =>
				gs1path_w.cpid = Some(Cpid { cpid, cpsn }),
			Gs1Path::Gln { gln, glnx } =>
				gs1path_w.gln = Some(Gln { gln, glnx }),
			Gs1Path::PayTo(v) => gs1path_w.pay_to = Some(PayTo(v)),
			Gs1Path::PartyGln(v) => gs1path_w.party_gln = Some(PartyGln(v)),
			Gs1Path::Gsrnp { gsrnp, srin } =>
				gs1path_w.gsrnp = Some(Gsrnp { gsrnp, srin }),
			Gs1Path::Gsrn { gsrn, srin } =>
				gs1path_w.gsrn = Some(Gsrn { gsrn, srin }),
			Gs1Path::Gcn(v) => gs1path_w.gcn = Some(Gcn(v)),
			Gs1Path::Sscc(v) => gs1path_w.sscc = Some(Sscc(v)),
			Gs1Path::Gdti(v) => gs1path_w.gdti = Some(Gdti(v)),
			Gs1Path::Ginc(v) => gs1path_w.ginc = Some(Ginc(v)),
			Gs1Path::Gsin(v) => gs1path_w.gsin = Some(Gsin(v)),
			Gs1Path::Grai(v) => gs1path_w.grai = Some(Grai(v)),
			Gs1Path::Giai(v) => gs1path_w.giai = Some(Giai(v)),
			Gs1Path::Upui { gtin, tpx } =>
				gs1path_w.upui = Some(Upui { gtin, tpx }),
			Gs1Path::Eoid { party_gln, uic_ext } =>
				gs1path_w.eoid = Some(Eoid { party_gln, uic_ext }),
			Gs1Path::Fid { gln, uic_ext } =>
				gs1path_w.fid = Some(Fid { gln, uic_ext }),
			Gs1Path::Mid { giai, uic_ext } =>
				gs1path_w.mid = Some(Mid { giai, uic_ext }),
		}

		gs1path_w
	}
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Gtin {
	pub gtin: String,
	pub cpv: Option<String>,
	pub lot: Option<String>,
	pub ser: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Itip {
	pub itip: String,
	pub cpv: Option<String>,
	pub lot: Option<String>,
	pub ser: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Gmn(String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Cpid {
	pub cpid: String,
	pub cpsn: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Gln {
	pub gln: String,
	pub glnx: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct PayTo(String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct PartyGln(String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Gsrnp {
	pub gsrnp: String,
	pub srin: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Gsrn {
	pub gsrn: String,
	pub srin: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Gcn(String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Sscc(String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Gdti(String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Ginc(String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Gsin(String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Grai(String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Giai(String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Upui {
	pub gtin: String,
	pub tpx: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Eoid {
	pub party_gln: String,
	pub uic_ext: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Fid {
	pub gln: String,
	pub uic_ext: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Mid {
	pub giai: String,
	pub uic_ext: String,
}