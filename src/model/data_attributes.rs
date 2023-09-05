use wasm_bindgen::prelude::wasm_bindgen;

/// Maps of all possible attributes of a Digital Link
#[derive(Default, Debug, Clone)]
#[non_exhaustive]
#[wasm_bindgen(getter_with_clone, inspectable)]
pub struct DataAttributes {
	pub net_weight_vmti: Option<String>,
	pub length_vmti: Option<String>,
	pub width_vmti: Option<String>,
	pub depth_vmti: Option<String>,
	pub area_vmti: Option<String>,
	pub net_volume_vmti: Option<String>,
	pub mass_per_unit_area_vmti: Option<String>,
	pub gross_weight: Option<String>,
	pub logistic_length: Option<String>,
	pub logistic_width: Option<String>,
	pub logistic_depth: Option<String>,
	pub logistic_area: Option<String>,
	pub logistic_volume: Option<String>,
	pub processor: Option<String>,
	pub lot: Option<String>,
	pub expiry_date: Option<String>,
	pub expiry_time: Option<String>,
	pub content: Option<String>,
	pub prod_date: Option<String>,
	pub due_date: Option<String>,
	pub pack_date: Option<String>,
	pub best_before_date: Option<String>,
	pub sell_by_date: Option<String>,
	pub first_freeze_date: Option<String>,
	pub harvest_date: Option<String>,
	pub price_per_unit: Option<String>,
	pub variant: Option<String>,
	pub var_count: Option<String>,
	pub count: Option<String>,
	pub amount_pay_per_unit: Option<String>,
	pub additional_id: Option<String>,
	pub cust_part_no: Option<String>,
	pub mto_variant: Option<String>,
	pub pcn: Option<String>,
	pub secondary_serial: Option<String>,
	pub ref_to_source: Option<String>,
	pub amount: Option<String>,
	pub amount_iso: Option<String>,
	pub price: Option<String>,
	pub price_iso: Option<String>,
	pub percent_off: Option<String>,
	pub order_number: Option<String>,
	pub route: Option<String>,
	pub ship_to_loc: Option<String>,
	pub bill_to: Option<String>,
	pub purchase_from: Option<String>,
	pub ship_for_loc: Option<String>,
	pub prod_serv_loc: Option<String>,
	pub ship_to_post: Option<String>,
	pub ship_to_post_iso: Option<String>,
	pub origin: Option<String>,
	pub country_process: Option<String>,
	pub country_full_process: Option<String>,
	pub country_initial_process: Option<String>,
	pub country_disassembly: Option<String>,
	pub origin_subdivision: Option<String>,
	pub nhrn_pzn: Option<String>,
	pub nhrn_cip: Option<String>,
	pub nhrn_cn: Option<String>,
	pub nhrn_drn: Option<String>,
	pub nhrn_aim: Option<String>,
	pub nhrn_us_fda: Option<String>,
	pub nsn: Option<String>,
	pub meat_cut: Option<String>,
	pub active_potency: Option<String>,
	pub catch_area: Option<String>,
	pub fishing_gear_type: Option<String>,
	pub prod_method: Option<String>,
	pub refurb_lot: Option<String>,
	pub func_stat: Option<String>,
	pub rev_stat: Option<String>,
	pub giai_assembly: Option<String>,
	pub dimensions: Option<String>,
	pub cmt_no: Option<String>,
	pub iban: Option<String>,
	pub prod_time: Option<String>,
	pub version: Option<String>,
	pub ref_no: Option<String>,
	pub coupon_idna: Option<String>,
	pub points: Option<String>,
	pub itip_content: Option<String>,
	pub certification_ref: Option<String>,
	pub aquatic_species: Option<String>,
	pub optical_sensor: Option<String>,
	pub paperless_coupon_idna: Option<String>,
	pub internal: Option<String>,
	pub mutual: Option<String>,
	pub extension: Option<String>,
	pub ship_to_comp: Option<String>,
	pub ship_to_name: Option<String>,
	pub ship_to_add1: Option<String>,
	pub ship_to_add2: Option<String>,
	pub ship_to_sub: Option<String>,
	pub ship_to_locality: Option<String>,
	pub ship_to_reg: Option<String>,
	pub ship_to_country: Option<String>,
	pub ship_to_phone: Option<String>,
	pub rtn_to_comp: Option<String>,
	pub rtn_to_name: Option<String>,
	pub rtn_to_add1: Option<String>,
	pub rtn_to_add2: Option<String>,
	pub rtn_to_sub: Option<String>,
	pub rtn_to_loc: Option<String>,
	pub rtn_to_reg: Option<String>,
	pub rtn_to_country: Option<String>,
	pub rtn_to_post: Option<String>,
	pub rtn_to_phone: Option<String>,
	pub srv_description: Option<String>,
	pub dangerous_goods: Option<String>,
	pub auth_to_leave: Option<String>,
	pub sig_required: Option<String>,
	pub not_before_del_date: Option<String>,
	pub not_after_del_date: Option<String>,
	pub release_date: Option<String>,
	pub gtin: Option<String>,
	pub itip: Option<String>,
	pub gmn: Option<String>,
	pub cpid: Option<String>,
	pub gln: Option<String>,
	pub pay_to: Option<String>,
	pub party_gln: Option<String>,
	pub gsrnp: Option<String>,
	pub gsrn: Option<String>,
	pub gcn: Option<String>,
	pub sscc: Option<String>,
	pub gdti: Option<String>,
	pub ginc: Option<String>,
	pub gsin: Option<String>,
	pub grai: Option<String>,
	pub giai: Option<String>,
}
