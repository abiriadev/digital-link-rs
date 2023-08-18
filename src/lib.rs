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
