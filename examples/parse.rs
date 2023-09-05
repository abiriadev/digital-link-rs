use digital_link_rs::{self, DigitalLink, Gs1Path};

fn main() {
	let digital_link = DigitalLink::try_from_str(
        r"https://id.gs1.org/00/195201234567891232?02=09520123456788&37=25&10=ABC12&17=180426"
    ).expect("parsing error");

	if let Gs1Path::Sscc(sscc) = digital_link.gs1_path {
		println!("sscc: {sscc}");
	}

	if let Some(date) = digital_link.data_attributes.expiry_date {
		println!("expiry date: {date}");
	}
}
