use std::borrow::{Borrow, Cow};

use crate::DataAttributes;

macro_rules! v {
	($v:tt) => {
		Some($v.into_owned())
	};
}

pub fn parse_data_attribute(
	data_attributes: &mut DataAttributes,
	(key, value): (Cow<'_, str>, Cow<'_, str>),
) {
	match key.borrow() {
		"3100" | "3101" | "3102" | "3103" | "3104" | "3105" | "3200"
		| "3201" | "3202" | "3203" | "3204" | "3205" | "3560" | "3561"
		| "3562" | "3563" | "3564" | "3565" | "3570" | "3571" | "3572"
		| "3573" | "3574" | "3575" => {
			// netWeightVMTIValue = 6DIGIT
			data_attributes.net_weight_vmti = v!(value);
		},
		"3110" | "3111" | "3112" | "3113" | "3114" | "3115" | "3210"
		| "3211" | "3212" | "3213" | "3214" | "3215" | "3220" | "3221"
		| "3222" | "3223" | "3224" | "3225" | "3230" | "3231" | "3232"
		| "3233" | "3234" | "3235" => {
			// lengthVMTIValue = 6DIGIT
			data_attributes.length_vmti = v!(value);
		},
		"3120" | "3121" | "3122" | "3123" | "3124" | "3125" | "3240"
		| "3241" | "3242" | "3243" | "3244" | "3245" | "3250" | "3251"
		| "3252" | "3253" | "3254" | "3255" | "3260" | "3261" | "3262"
		| "3263" | "3264" | "3265" => {
			// widthVMTIValue = 6DIGIT
			data_attributes.width_vmti = v!(value);
		},
		"3130" | "3131" | "3132" | "3133" | "3134" | "3135" | "3270"
		| "3271" | "3272" | "3273" | "3274" | "3275" | "3280" | "3281"
		| "3282" | "3283" | "3284" | "3285" | "3290" | "3291" | "3292"
		| "3293" | "3294" | "3295" => {
			// depthVMTIValue = 6DIGIT
			data_attributes.depth_vmti = v!(value);
		},
		"3140" | "3141" | "3142" | "3143" | "3144" | "3145" | "3500"
		| "3501" | "3502" | "3503" | "3504" | "3505" | "3510" | "3511"
		| "3512" | "3513" | "3514" | "3515" | "3520" | "3521" | "3522"
		| "3523" | "3524" | "3525" => {
			// areaVMTIValue = 6DIGIT
			data_attributes.area_vmti = v!(value);
		},
		"3150" | "3151" | "3152" | "3153" | "3154" | "3155" | "3160"
		| "3161" | "3162" | "3163" | "3164" | "3165" | "3600" | "3601"
		| "3602" | "3603" | "3604" | "3605" | "3610" | "3611" | "3612"
		| "3613" | "3614" | "3615" | "3640" | "3641" | "3642" | "3643"
		| "3644" | "3645" | "3650" | "3651" | "3652" | "3653" | "3654"
		| "3655" | "3660" | "3661" | "3662" | "3663" | "3664" | "3665" => {
			// netVolumeVMTIValue = 6DIGIT
			data_attributes.net_volume_vmti = v!(value);
		},
		"3370" | "3371" | "3372" | "3373" | "3374" | "3375" => {
			// massPerUnitAreaVMTIValue = 6DIGIT
			data_attributes.mass_per_unit_area_vmti = v!(value);
		},
		"3300" | "3301" | "3302" | "3303" | "3304" | "3305" | "3400"
		| "3401" | "3402" | "3403" | "3404" | "3405" => {
			// grossWeightValue = 6DIGIT
			data_attributes.gross_weight = v!(value);
		},
		"3310" | "3311" | "3312" | "3313" | "3314" | "3315" | "3410"
		| "3411" | "3412" | "3413" | "3414" | "3415" | "3420" | "3421"
		| "3422" | "3423" | "3424" | "3425" | "3430" | "3431" | "3432"
		| "3433" | "3434" | "3435" => {
			// logisticLengthValue = 6DIGIT
			data_attributes.logistic_length = v!(value);
		},
		"3320" | "3321" | "3322" | "3323" | "3324" | "3325" | "3440"
		| "3441" | "3442" | "3443" | "3444" | "3445" | "3450" | "3451"
		| "3452" | "3453" | "3454" | "3455" | "3460" | "3461" | "3462"
		| "3463" | "3464" | "3465" => {
			// logisticWidthValue = 6DIGIT
			data_attributes.logistic_width = v!(value);
		},
		"3330" | "3331" | "3332" | "3333" | "3334" | "3335" | "3470"
		| "3471" | "3472" | "3473" | "3474" | "3475" | "3480" | "3481"
		| "3482" | "3483" | "3484" | "3485" | "3490" | "3491" | "3492"
		| "3493" | "3494" | "3495" => {
			// logisticDepthValue = 6DIGIT
			data_attributes.logistic_depth = v!(value);
		},
		"3340" | "3341" | "3342" | "3343" | "3344" | "3345" | "3530"
		| "3531" | "3532" | "3533" | "3534" | "3535" | "3540" | "3541"
		| "3542" | "3543" | "3544" | "3545" | "3550" | "3551" | "3552"
		| "3553" | "3554" | "3555" => {
			// logisticAreaValue = 6DIGIT
			data_attributes.logistic_area = v!(value);
		},
		"3350" | "3351" | "3352" | "3353" | "3354" | "3355" | "3360"
		| "3361" | "3362" | "3363" | "3364" | "3365" | "3620" | "3621"
		| "3622" | "3623" | "3624" | "3625" | "3630" | "3631" | "3632"
		| "3633" | "3634" | "3635" | "3670" | "3671" | "3672" | "3673"
		| "3674" | "3675" | "3680" | "3681" | "3682" | "3683" | "3684"
		| "3685" | "3690" | "3691" | "3692" | "3693" | "3694" | "3695" => {
			// logisticVolumeValue = 6DIGIT
			data_attributes.logistic_volume = v!(value);
		},
		"7030" | "7031" | "7032" | "7033" | "7034" | "7035" | "7036"
		| "7037" | "7038" | "7039" => {
			// processorValue = 3DIGIT 1*27XCHAR
			data_attributes.processor = v!(value);
		},
		"02" => {
			// contentParameter = 14DIGIT
			data_attributes.content = v!(value);
		},
		"11" => {
			// prodDateParameter = 6DIGIT
			data_attributes.prod_date = v!(value);
		},
		"12" => {
			// dueDateParameter = 6DIGIT
			data_attributes.due_date = v!(value);
		},
		"13" => {
			// packDateParameter = 6DIGIT
			data_attributes.pack_date = v!(value);
		},
		"15" => {
			// bestBeforeDateParameter = 6DIGIT
			data_attributes.best_before_date = v!(value);
		},
		"16" => {
			// sellByDateParameter = 6DIGIT
			data_attributes.sell_by_date = v!(value);
		},
		"7006" => {
			// firstFreezeDateParameter = 6DIGIT
			data_attributes.first_freeze_date = v!(value);
		},
		"7007" => {
			// harvestDateParameter = 6DIGIT [6DIGIT]
			data_attributes.harvest_date = v!(value);
		},
		"8005" => {
			// pricePerUnitParameter = 6DIGIT
			data_attributes.price_per_unit = v!(value);
		},
		"20" => {
			// variantParameter = 2DIGIT
			data_attributes.variant = v!(value);
		},
		"30" => {
			// varCountParameter = 1*8DIGIT
			data_attributes.var_count = v!(value);
		},
		"37" => {
			// countParameter = 1*8DIGIT
			data_attributes.count = v!(value);
		},
		"90" => {
			// mutualParameter = 1*30XCHAR
			data_attributes.mutual = v!(value);
		},
		"240" => {
			// additionalIdParameter = 1*30XCHAR
			data_attributes.additional_id = v!(value);
		},
		"241" => {
			// custPartNoParameter = 1*30XCHAR
			data_attributes.cust_part_no = v!(value);
		},
		"242" => {
			// mtoVariantParameter = 1*6DIGIT
			data_attributes.mto_variant = v!(value);
		},
		"243" => {
			// pcnParameter = 1*20XCHAR
			data_attributes.pcn = v!(value);
		},
		"250" => {
			// secondarySerialParameter = 1*30XCHAR
			data_attributes.secondary_serial = v!(value);
		},
		"251" => {
			// refToSourceParameter = 1*30XCHAR
			data_attributes.ref_to_source = v!(value);
		},
		"3900" | "3901" | "3902" | "3903" | "3904" | "3905" => {
			// amountValue = 1*15DIGIT
			data_attributes.amount = v!(value);
		},
		"3910" | "3911" | "3912" | "3913" | "3914" | "3915" => {
			// amountISOValue = 3DIGIT 1*15DIGIT
			data_attributes.amount_iso = v!(value);
		},
		"3920" | "3921" | "3922" | "3923" | "3924" | "3925" => {
			// priceValue = 1*15DIGIT
			data_attributes.price = v!(value);
		},
		"3930" | "3931" | "3932" | "3933" | "3934" | "3935" => {
			// priceISOValue = 3DIGIT 1*15DIGIT
			data_attributes.price_iso = v!(value);
		},
		"3940" | "3941" | "3942" | "3943" | "3944" | "3945" => {
			// percentOffValue = 4DIGIT
			data_attributes.percent_off = v!(value);
		},
		"400" => {
			// orderNumberParameter = 1*30XCHAR
			data_attributes.order_number = v!(value);
		},
		"403" => {
			// routeParameter = 1*30XCHAR
			data_attributes.route = v!(value);
		},
		"410" => {
			// shipToLocParameter = 13DIGIT
			data_attributes.ship_to_loc = v!(value);
		},
		"411" => {
			// billToParameter = 13DIGIT
			data_attributes.bill_to = v!(value);
		},
		"412" => {
			// purchaseFromParameter = 13DIGIT
			data_attributes.purchase_from = v!(value);
		},
		"413" => {
			// shipForLocParameter = 13DIGIT
			data_attributes.ship_for_loc = v!(value);
		},
		"416" => {
			// prodServLocParameter = 13DIGIT
			data_attributes.prod_serv_loc = v!(value);
		},
		"420" => {
			// shipToPostParameter = 1*20XCHAR
			data_attributes.ship_to_post = v!(value);
		},
		"421" => {
			// shipToPostISOParameter = 3DIGIT 1*9XCHAR
			data_attributes.ship_to_post_iso = v!(value);
		},
		"422" => {
			// originParameter = 3DIGIT
			data_attributes.origin = v!(value);
		},
		"424" => {
			// countryProcessParameter = 3DIGIT
			data_attributes.country_process = v!(value);
		},
		"426" => {
			// countryFullProcessParameter = 3DIGIT
			data_attributes.country_full_process = v!(value);
		},
		"423" => {
			// countryInitialProcessParameter = 3DIGIT 1*12DIGIT
			data_attributes.country_initial_process = v!(value);
		},
		"425" => {
			// countryDisassemblyParameter = 3DIGIT 1*12DIGIT
			data_attributes.country_disassembly = v!(value);
		},
		"427" => {
			// originSubdivisionParameter = 1*3XCHAR
			data_attributes.origin_subdivision = v!(value);
		},
		"710" => {
			// nhrnPZNParameter = 1*20XCHAR
			data_attributes.nhrn_pzn = v!(value);
		},
		"711" => {
			// nhrnCIPParameter = 1*20XCHAR
			data_attributes.nhrn_cip = v!(value);
		},
		"712" => {
			// nhrnCNParameter = 1*20XCHAR
			data_attributes.nhrn_cn = v!(value);
		},
		"713" => {
			// nhrnDRNParameter = 1*20XCHAR
			data_attributes.nhrn_drn = v!(value);
		},
		"714" => {
			// nhrnAIMParameter = 1*20XCHAR
			data_attributes.nhrn_aim = v!(value);
		},
		"715" => {
			// nhrnUS-FDAParameter = 1*20XCHAR
			data_attributes.nhrn_us_fda = v!(value);
		},
		"7001" => {
			// nsnParameter = 13DIGIT
			data_attributes.nsn = v!(value);
		},
		"7002" => {
			// meatCutParameter = 1*30XCHAR
			data_attributes.meat_cut = v!(value);
		},
		"7004" => {
			// activePotencyParameter = 1*4DIGIT
			data_attributes.active_potency = v!(value);
		},
		"7005" => {
			// catchAreaParameter = 1*12XCHAR
			data_attributes.catch_area = v!(value);
		},
		"7008" => {
			// aquaticSpeciesParameter = 1*3XCHAR
			data_attributes.aquatic_species = v!(value);
		},
		"7009" => {
			// fishingGearTypeParameter = 1*10XCHAR
			data_attributes.fishing_gear_type = v!(value);
		},
		"7010" => {
			// prodMethodParameter = 1*2XCHAR
			data_attributes.prod_method = v!(value);
		},
		"7020" => {
			// refurbLotParameter = 1*20XCHAR
			data_attributes.refurb_lot = v!(value);
		},
		"7021" => {
			// funcStatParameter = 1*20XCHAR
			data_attributes.func_stat = v!(value);
		},
		"7022" => {
			// revStatParameter = 1*20XCHAR
			data_attributes.rev_stat = v!(value);
		},
		"7023" => {
			// giaiAssemblyParameter = 1*30XCHAR
			data_attributes.giai_assembly = v!(value);
		},
		"7230" | "7231" | "7232" | "7233" | "7234" | "7235" | "7236"
		| "7237" | "7238" | "7239" => {
			// certificationRefValue = 2XCHAR 1*28XCHAR
			data_attributes.certification_ref = v!(value);
		},
		"8001" => {
			// dimensionsParameter = 14DIGIT
			data_attributes.dimensions = v!(value);
		},
		"8002" => {
			// cmtNoParameter = 1*20XCHAR
			data_attributes.cmt_no = v!(value);
		},
		"8007" => {
			// ibanParameter = 1*34XCHAR
			data_attributes.iban = v!(value);
		},
		"8008" => {
			// prodTimeParameter = 8DIGIT [2DIGIT] [2DIGIT]
			data_attributes.prod_time = v!(value);
		},
		"8009" => {
			// opticalSensorParameter = 1*50XCHAR
			data_attributes.optical_sensor = v!(value);
		},
		"8012" => {
			// versionParameter = 1*20XCHAR
			data_attributes.version = v!(value);
		},
		"8020" => {
			// refNoParameter = 1*25XCHAR
			data_attributes.ref_no = v!(value);
		},
		"8026" => {
			// itipContentParameter = 14DIGIT 2DIGIT 2DIGIT
			data_attributes.itip_content = v!(value);
		},
		"8110" => {
			// couponIDNAParameter = 1*70XCHAR
			data_attributes.coupon_idna = v!(value);
		},
		"8111" => {
			// pointsParameter = 4DIGIT
			data_attributes.points = v!(value);
		},
		"8112" => {
			// paperlessCouponIDNAParameter = 1*70XCHAR
			data_attributes.paperless_coupon_idna = v!(value);
		},
		"4300" => {
			// shipToCompParameter = 1*35XCHAR
			data_attributes.ship_to_comp = v!(value);
		},
		"4301" => {
			// shipToNameParameter = 1*35XCHAR
			data_attributes.ship_to_name = v!(value);
		},
		"4302" => {
			// shipToAdd1Parameter = 1*70XCHAR
			data_attributes.ship_to_add1 = v!(value);
		},
		"4303" => {
			// shipToAdd2Parameter = 1*70XCHAR
			data_attributes.ship_to_add2 = v!(value);
		},
		"4304" => {
			// shipToSubParameter = 1*70XCHAR
			data_attributes.ship_to_sub = v!(value);
		},
		"4305" => {
			// shipToLocalityParameter = 1*70XCHAR
			data_attributes.ship_to_locality = v!(value);
		},
		"4306" => {
			// shipToRegParameter = 1*70XCHAR
			data_attributes.ship_to_reg = v!(value);
		},
		"4307" => {
			// shipToCountryParameter = 2XCHAR
			data_attributes.ship_to_country = v!(value);
		},
		"4308" => {
			// shipToPhoneParameter = 1*30XCHAR
			data_attributes.ship_to_phone = v!(value);
		},
		"4310" => {
			// rtnToCompParameter = 1*35XCHAR
			data_attributes.rtn_to_comp = v!(value);
		},
		"4311" => {
			// rtnToNameParameter = 1*35XCHAR
			data_attributes.rtn_to_name = v!(value);
		},
		"4312" => {
			// rtnToAdd1Parameter = 1*70XCHAR
			data_attributes.rtn_to_add1 = v!(value);
		},
		"4313" => {
			// rtnToAdd2Parameter = 1*70XCHAR
			data_attributes.rtn_to_add2 = v!(value);
		},
		"4314" => {
			// rtnToSubParameter = 1*70XCHAR
			data_attributes.rtn_to_sub = v!(value);
		},
		"4315" => {
			// rtnToLocParameter = 1*70XCHAR
			data_attributes.rtn_to_loc = v!(value);
		},
		"4316" => {
			// rtnToRegParameter = 1*70XCHAR
			data_attributes.rtn_to_reg = v!(value);
		},
		"4317" => {
			// rtnToCountryParameter = 2XCHAR
			data_attributes.rtn_to_country = v!(value);
		},
		"4318" => {
			// rtnToPostParameter = 1*20XCHAR
			data_attributes.rtn_to_post = v!(value);
		},
		"4319" => {
			// rtnToPhoneParameter = 1*30XCHAR
			data_attributes.rtn_to_phone = v!(value);
		},
		"4320" => {
			// srvDescriptionParameter = 1*35XCHAR
			data_attributes.srv_description = v!(value);
		},
		"4321" => {
			// dangerousGoodsParameter = BOOLEAN
			data_attributes.dangerous_goods = v!(value);
		},
		"4322" => {
			// authToLeaveParameter = BOOLEAN
			data_attributes.auth_to_leave = v!(value);
		},
		"4323" => {
			// sigRequiredParameter = BOOLEAN
			data_attributes.sig_required = v!(value);
		},
		"4324" => {
			// notBeforeDelDateParameter = 10DIGIT
			data_attributes.not_before_del_date = v!(value);
		},
		"4325" => {
			// notAfterDelDateParameter = 10DIGIT
			data_attributes.not_after_del_date = v!(value);
		},
		"4326" => {
			// releaseDateParameter = 6DIGIT
			data_attributes.release_date = v!(value);
		},
		"3950" | "3951" | "3952" | "3953" => {
			// amountPayPerUnitValue = 6DIGIT
			data_attributes.amount_pay_per_unit = v!(value);
		},
		"01" => {
			// gtinParameter = gtin-value
			data_attributes.gtin = v!(value);
		},
		"8006" => {
			// itipParameter = itip-value
			data_attributes.itip = v!(value);
		},
		"8013" => {
			// gmnParameter = gmn-value
			data_attributes.gmn = v!(value);
		},
		"8010" => {
			// cpidParameter = cpid-value
			data_attributes.cpid = v!(value);
		},
		"414" => {
			// glnParameter = gln-value
			data_attributes.gln = v!(value);
		},
		"415" => {
			// payToParameter = payTo-value
			data_attributes.pay_to = v!(value);
		},
		"417" => {
			// partyGlnParameter = partyGln-value
			data_attributes.party_gln = v!(value);
		},
		"8017" => {
			// gsrnpParameter = gsrnp-value
			data_attributes.gsrnp = v!(value);
		},
		"8018" => {
			// gsrnParameter = gsrn-value
			data_attributes.gsrn = v!(value);
		},
		"255" => {
			// gcnParameter = gcn-value
			data_attributes.gcn = v!(value);
		},
		"00" => {
			// ssccParameter = sscc-value
			data_attributes.sscc = v!(value);
		},
		"253" => {
			// gdtiParameter = gdti-value
			data_attributes.gdti = v!(value);
		},
		"401" => {
			// gincParameter = ginc-value
			data_attributes.ginc = v!(value);
		},
		"402" => {
			// gsinParameter = gsin-value
			data_attributes.gsin = v!(value);
		},
		"8003" => {
			// graiParameter = grai-value
			data_attributes.grai = v!(value);
		},
		"8004" => {
			// giaiParameter = giai-value
			data_attributes.giai = v!(value);
		},
		"96" | "97" | "98" | "99" => {
			// internalValue = 1*90XCHAR
			data_attributes.internal = v!(value);
		},
	}
}
