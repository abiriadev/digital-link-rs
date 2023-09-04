use assert_matches::assert_matches;
use digital_link_rs::{DigitalLink, Gs1Path};

#[test]
fn parse() {
	let url =
		r"https://id.gs1.org/01/09520123456788/10/ABC1/21/12345?17=180426";

	let res = DigitalLink::try_from_str(url);

	assert_matches!(res, Ok(_));
	let res = res.unwrap();

	let Gs1Path::Gtin { gtin, lot, ser, .. } = res.gs1_path else {
		panic!();
	};

	assert_eq!(gtin, "09520123456788");

	assert_eq!(lot, Some("ABC1".to_owned()));

	assert_eq!(ser, Some("12345".to_owned()));

	assert_eq!(
		res.data_attributes.expiry_date,
		Some("180426".to_owned())
	);

	// assert_eq!(res)
}
