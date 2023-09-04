use std::borrow::{Borrow, Cow};

use nom::{
	combinator::opt,
	multi::{count, many_m_n},
	Parser,
};

use super::{
	primary_key::{
		cpid_value, gcn_value, gdti_value, giai_value, ginc_value, gln_value,
		gmn_value, grai_value, gsin_value, gsrn_value, gsrnp_value, gtin_value,
		itip_value, party_gln_value, pay_to_value, sscc_value,
	},
	primitive::{bool, digit, xchar},
};
use crate::DataAttributes;

pub fn parse_data_attribute(
	data_attributes: &mut DataAttributes,
	(key, value): (Cow<'_, str>, Cow<'_, str>),
) {
	attr! {
		match (
			data_attributes,
			key.borrow(),
			value.borrow(),
		) {
			"3100" | "3101" | "3102" | "3103" | "3104" | "3105" | "3200"
			| "3201" | "3202" | "3203" | "3204" | "3205" | "3560" | "3561"
			| "3562" | "3563" | "3564" | "3565" | "3570" | "3571" | "3572"
			| "3573" | "3574" | "3575" => (net_weight_vmti, digit!(6)),
			"3110" | "3111" | "3112" | "3113" | "3114" | "3115" | "3210"
			| "3211" | "3212" | "3213" | "3214" | "3215" | "3220" | "3221"
			| "3222" | "3223" | "3224" | "3225" | "3230" | "3231" | "3232"
			| "3233" | "3234" | "3235" => (length_vmti, digit!(6)),
			"3120" | "3121" | "3122" | "3123" | "3124" | "3125" | "3240"
			| "3241" | "3242" | "3243" | "3244" | "3245" | "3250" | "3251"
			| "3252" | "3253" | "3254" | "3255" | "3260" | "3261" | "3262"
			| "3263" | "3264" | "3265" => (width_vmti, digit!(6)),
			"3130" | "3131" | "3132" | "3133" | "3134" | "3135" | "3270"
			| "3271" | "3272" | "3273" | "3274" | "3275" | "3280" | "3281"
			| "3282" | "3283" | "3284" | "3285" | "3290" | "3291" | "3292"
			| "3293" | "3294" | "3295" => (depth_vmti, digit!(6)),
			"3140" | "3141" | "3142" | "3143" | "3144" | "3145" | "3500"
			| "3501" | "3502" | "3503" | "3504" | "3505" | "3510" | "3511"
			| "3512" | "3513" | "3514" | "3515" | "3520" | "3521" | "3522"
			| "3523" | "3524" | "3525" => (area_vmti, digit!(6)),
			"3150" | "3151" | "3152" | "3153" | "3154" | "3155" | "3160"
			| "3161" | "3162" | "3163" | "3164" | "3165" | "3600" | "3601"
			| "3602" | "3603" | "3604" | "3605" | "3610" | "3611" | "3612"
			| "3613" | "3614" | "3615" | "3640" | "3641" | "3642" | "3643"
			| "3644" | "3645" | "3650" | "3651" | "3652" | "3653" | "3654"
			| "3655" | "3660" | "3661" | "3662" | "3663" | "3664" | "3665" =>
				(net_volume_vmti, digit!(6)),
			"3370" | "3371" | "3372" | "3373" | "3374" | "3375" =>
				(mass_per_unit_area_vmti, digit!(6)),
			"3300" | "3301" | "3302" | "3303" | "3304" | "3305" | "3400"
			| "3401" | "3402" | "3403" | "3404" | "3405" => (gross_weight, digit!(6)),
			"3310" | "3311" | "3312" | "3313" | "3314" | "3315" | "3410"
			| "3411" | "3412" | "3413" | "3414" | "3415" | "3420" | "3421"
			| "3422" | "3423" | "3424" | "3425" | "3430" | "3431" | "3432"
			| "3433" | "3434" | "3435" => (logistic_length, digit!(6)),
			"3320" | "3321" | "3322" | "3323" | "3324" | "3325" | "3440"
			| "3441" | "3442" | "3443" | "3444" | "3445" | "3450" | "3451"
			| "3452" | "3453" | "3454" | "3455" | "3460" | "3461" | "3462"
			| "3463" | "3464" | "3465" => (logistic_width, digit!(6)),
			"3330" | "3331" | "3332" | "3333" | "3334" | "3335" | "3470"
			| "3471" | "3472" | "3473" | "3474" | "3475" | "3480" | "3481"
			| "3482" | "3483" | "3484" | "3485" | "3490" | "3491" | "3492"
			| "3493" | "3494" | "3495" => (logistic_depth, digit!(6)),
			"3340" | "3341" | "3342" | "3343" | "3344" | "3345" | "3530"
			| "3531" | "3532" | "3533" | "3534" | "3535" | "3540" | "3541"
			| "3542" | "3543" | "3544" | "3545" | "3550" | "3551" | "3552"
			| "3553" | "3554" | "3555" => (logistic_area, digit!(6)),
			"3350" | "3351" | "3352" | "3353" | "3354" | "3355" | "3360"
			| "3361" | "3362" | "3363" | "3364" | "3365" | "3620" | "3621"
			| "3622" | "3623" | "3624" | "3625" | "3630" | "3631" | "3632"
			| "3633" | "3634" | "3635" | "3670" | "3671" | "3672" | "3673"
			| "3674" | "3675" | "3680" | "3681" | "3682" | "3683" | "3684"
			| "3685" | "3690" | "3691" | "3692" | "3693" | "3694" | "3695" =>
				(logistic_volume, digit!(6)),
			"7030" | "7031" | "7032" | "7033" | "7034" | "7035" | "7036"
			| "7037" | "7038" | "7039" => (processor, digit!(3).and(xchar!(1, 27))),
			"02" => (content, digit!(14)),
			"11" => (prod_date, digit!(6)),
			"12" => (due_date, digit!(6)),
			"13" => (pack_date, digit!(6)),
			"15" => (best_before_date, digit!(6)),
			"16" => (sell_by_date, digit!(6)),
			"7006" => (first_freeze_date, digit!(6)),
			"7007" => (
				harvest_date,
				digit!(6).and(opt(digit!(6))),
			),
			"8005" => (price_per_unit, digit!(6)),
			"20" => (variant, digit!(2)),
			"30" => (var_count, digit!(1, 8)),
			"37" => (count, digit!(1, 8)),
			"90" => (mutual, xchar!(1, 30)),
			"240" => (additional_id, xchar!(1, 30)),
			"241" => (cust_part_no, xchar!(1, 30)),
			"242" => (mto_variant, digit!(1, 6)),
			"243" => (pcn, xchar!(1, 20)),
			"250" => (secondary_serial, xchar!(1, 30)),
			"251" => (ref_to_source, xchar!(1, 30)),
			"3900" | "3901" | "3902" | "3903" | "3904" | "3905" =>
				(amount, digit!(1, 15)),
			"3910" | "3911" | "3912" | "3913" | "3914" | "3915" =>
				(amount_iso, digit!(3).and(digit!(1, 15))),
			"3920" | "3921" | "3922" | "3923" | "3924" | "3925" =>
				(price, digit!(1, 15)),
			"3930" | "3931" | "3932" | "3933" | "3934" | "3935" =>
				(price_iso, digit!(3).and(digit!(1, 15))),
			"3940" | "3941" | "3942" | "3943" | "3944" | "3945" =>
				(percent_off, digit!(4)),
			"400" => (order_number, xchar!(1, 30)),
			"403" => (route, xchar!(1, 30)),
			"410" => (ship_to_loc, digit!(13)),
			"411" => (bill_to, digit!(13)),
			"412" => (purchase_from, digit!(13)),
			"413" => (ship_for_loc, digit!(13)),
			"416" => (prod_serv_loc, digit!(13)),
			"420" => (ship_to_post, xchar!(1, 20)),
			"421" => (
				ship_to_post_iso,
				digit!(3).and(xchar!(1, 9)),
			),
			"422" => (origin, digit!(3)),
			"424" => (country_process, digit!(3)),
			"426" => (country_full_process, digit!(3)),
			"423" => (
				country_initial_process,
				digit!(3).and(digit!(1, 12)),
			),
			"425" => (
				country_disassembly,
				digit!(3).and(digit!(1, 12)),
			),
			"427" => (origin_subdivision, xchar!(1, 3)),
			"710" => (nhrn_pzn, xchar!(1, 20)),
			"711" => (nhrn_cip, xchar!(1, 20)),
			"712" => (nhrn_cn, xchar!(1, 20)),
			"713" => (nhrn_drn, xchar!(1, 20)),
			"714" => (nhrn_aim, xchar!(1, 20)),
			"715" => (nhrn_us_fda, xchar!(1, 20)),
			"7001" => (nsn, digit!(13)),
			"7002" => (meat_cut, xchar!(1, 30)),
			"7004" => (active_potency, digit!(1, 4)),
			"7005" => (catch_area, xchar!(1, 12)),
			"7008" => (aquatic_species, xchar!(1, 3)),
			"7009" => (fishing_gear_type, xchar!(1, 10)),
			"7010" => (prod_method, xchar!(1, 2)),
			"7020" => (refurb_lot, xchar!(1, 20)),
			"7021" => (func_stat, xchar!(1, 20)),
			"7022" => (rev_stat, xchar!(1, 20)),
			"7023" => (giai_assembly, xchar!(1, 30)),
			"7230" | "7231" | "7232" | "7233" | "7234" | "7235" | "7236"
			| "7237" | "7238" | "7239" => (
				certification_ref,
				xchar!(2).and(xchar!(1, 28)),
			),
			"8001" => (dimensions, digit!(14)),
			"8002" => (cmt_no, xchar!(1, 20)),
			"8007" => (iban, xchar!(1, 34)),
			"8008" => (
				prod_time,
				digit!(8)
					.and(opt(digit!(2)))
					.and(opt(digit!(2))),
			),
			"8009" => (optical_sensor, xchar!(1, 50)),
			"8012" => (version, xchar!(1, 20)),
			"8020" => (ref_no, xchar!(1, 25)),
			"8026" => (itip_content, digit!(18)),
			"8110" => (coupon_idna, xchar!(1, 70)),
			"8111" => (points, digit!(4)),
			"8112" => (paperless_coupon_idna, xchar!(1, 70)),
			"4300" => (ship_to_comp, xchar!(1, 35)),
			"4301" => (ship_to_name, xchar!(1, 35)),
			"4302" => (ship_to_add1, xchar!(1, 70)),
			"4303" => (ship_to_add2, xchar!(1, 70)),
			"4304" => (ship_to_sub, xchar!(1, 70)),
			"4305" => (ship_to_locality, xchar!(1, 70)),
			"4306" => (ship_to_reg, xchar!(1, 70)),
			"4307" => (ship_to_country, xchar!(2)),
			"4308" => (ship_to_phone, xchar!(1, 30)),
			"4310" => (rtn_to_comp, xchar!(1, 35)),
			"4311" => (rtn_to_name, xchar!(1, 35)),
			"4312" => (rtn_to_add1, xchar!(1, 70)),
			"4313" => (rtn_to_add2, xchar!(1, 70)),
			"4314" => (rtn_to_sub, xchar!(1, 70)),
			"4315" => (rtn_to_loc, xchar!(1, 70)),
			"4316" => (rtn_to_reg, xchar!(1, 70)),
			"4317" => (rtn_to_country, xchar!(2)),
			"4318" => (rtn_to_post, xchar!(1, 20)),
			"4319" => (rtn_to_phone, xchar!(1, 30)),
			"4320" => (srv_description, xchar!(1, 35)),
			"4321" => (dangerous_goods, bool),
			"4322" => (auth_to_leave, bool),
			"4323" => (sig_required, bool),
			"4324" => (not_before_del_date, digit!(10)),
			"4325" => (not_after_del_date, digit!(10)),
			"4326" => (release_date, digit!(6)),
			"3950" | "3951" | "3952" | "3953" => (amount_pay_per_unit, digit!(6)),
			"01" => (gtin, gtin_value),
			"8006" => (itip, itip_value),
			"8013" => (gmn, gmn_value),
			"8010" => (cpid, cpid_value),
			"414" => (gln, gln_value),
			"415" => (pay_to, pay_to_value),
			"417" => (party_gln, party_gln_value),
			"8017" => (gsrnp, gsrnp_value),
			"8018" => (gsrn, gsrn_value),
			"255" => (gcn, gcn_value),
			"00" => (sscc, sscc_value),
			"253" => (gdti, gdti_value),
			"401" => (ginc, ginc_value),
			"402" => (gsin, gsin_value),
			"8003" => (grai, grai_value),
			"8004" => (giai, giai_value),
			"96" | "97" | "98" | "99" => (internal, xchar!(1, 90)),
		}
	};
}
