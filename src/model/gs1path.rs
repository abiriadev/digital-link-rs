#[non_exhaustive]
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
