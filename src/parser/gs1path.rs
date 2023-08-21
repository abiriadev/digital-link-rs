use nom::{
	branch::alt,
	combinator::opt,
	sequence::{pair, tuple},
	IResult, Parser,
};

use super::{
	primary_key::{
		cpid, gcn, gdti, giai, ginc, gln, gmn, grai, gsin, gsrn, gsrnp, gtin,
		itip, party_gln, pay_to, sscc,
	},
	qualifier::{cpsn, cpv, glnx, lot, ser, srin, tpx, uic_ext},
};
use crate::model::Gs1Path;

fn gs1path(i: &str) -> IResult<&str, Gs1Path> {
	alt((
		tuple((gtin, opt(cpv), opt(lot), opt(ser))).map(
			|(gtin, cpv, lot, ser)| Gs1Path::Gtin {
				gtin: gtin.to_owned(),
				cpv: cpv.map(ToOwned::to_owned),
				lot: lot.map(ToOwned::to_owned),
				ser: ser.map(ToOwned::to_owned),
			},
		),
		tuple((itip, opt(cpv), opt(lot), opt(ser))).map(
			|(itip, cpv, lot, ser)| Gs1Path::Itip {
				itip: itip.to_owned(),
				cpv: cpv.map(ToOwned::to_owned),
				lot: lot.map(ToOwned::to_owned),
				ser: ser.map(ToOwned::to_owned),
			},
		),
		gmn.map(|gmn| Gs1Path::Gcn(gmn.to_owned())),
		pair(cpid, opt(cpsn)).map(|(cpid, cpsn)| Gs1Path::Cpid {
			cpid: cpid.to_owned(),
			cpsn: cpsn.map(ToOwned::to_owned),
		}),
		pair(gln, opt(glnx)).map(|(gln, glnx)| Gs1Path::Gln {
			gln: gln.to_owned(),
			glnx: glnx.map(ToOwned::to_owned),
		}),
		pay_to.map(|pay_to| Gs1Path::Gcn(pay_to.to_owned())),
		party_gln.map(|party_gln| Gs1Path::Gcn(party_gln.to_owned())),
		pair(gsrnp, opt(srin)).map(|(gsrnp, srin)| Gs1Path::Gsrnp {
			gsrnp: gsrnp.to_owned(),
			srin: srin.map(ToOwned::to_owned),
		}),
		pair(gsrn, opt(srin)).map(|(gsrn, srin)| Gs1Path::Gsrn {
			gsrn: gsrn.to_owned(),
			srin: srin.map(ToOwned::to_owned),
		}),
		gcn.map(|gcn| Gs1Path::Gcn(gcn.to_owned())),
		sscc.map(|sscc| Gs1Path::Gcn(sscc.to_owned())),
		gdti.map(|gdti| Gs1Path::Gcn(gdti.to_owned())),
		ginc.map(|ginc| Gs1Path::Gcn(ginc.to_owned())),
		gsin.map(|gsin| Gs1Path::Gcn(gsin.to_owned())),
		grai.map(|grai| Gs1Path::Gcn(grai.to_owned())),
		giai.map(|giai| Gs1Path::Gcn(giai.to_owned())),
		pair(gtin, tpx).map(|(gtin, tpx)| Gs1Path::Upui {
			gtin: gtin.to_owned(),
			tpx: tpx.to_owned(),
		}),
		pair(party_gln, uic_ext).map(|(party_gln, uic_ext)| Gs1Path::Eoid {
			party_gln: party_gln.to_owned(),
			uic_ext: uic_ext.to_owned(),
		}),
		pair(gln, uic_ext).map(|(gln, uic_ext)| Gs1Path::Fid {
			gln: gln.to_owned(),
			uic_ext: uic_ext.to_owned(),
		}),
		pair(giai, uic_ext).map(|(giai, uic_ext)| Gs1Path::Mid {
			giai: giai.to_owned(),
			uic_ext: uic_ext.to_owned(),
		}),
	))(i)
}
