#![allow(unused)]

use nom::{
	bytes::complete::tag,
	combinator::{opt, recognize},
	multi::{count, many_m_n},
	sequence::{pair, preceded},
	IResult,
};

use super::primitive::{digit, xchar};

macro_rules! comp {
	($name:ident, $code:literal, $value:expr) => {
		fn $name(i: &str) -> IResult<&str, &str> {
			recognize(pair(
				preceded(tag("/"), tag($code)),
				preceded(tag("/"), $value),
			))(i)
		}
	};
}

comp! { gtin,	"01",	count(digit, 14) }
comp! { itip,	"8006",	count(digit, 18) }
comp! { gmn,	"8013",	many_m_n(1, 25, xchar) }
comp! { cpid,	"8010",	many_m_n(1, 30, xchar) }
comp! { gln,	"414",	count(digit, 13) }
comp! { pay_to,	"415",	count(digit, 13) }
comp! { party_gln,	"417",	count(digit, 13) }
comp! { gsrnp,	"8017",	count(digit, 18) }
comp! { gsrn,	"8018",	count(digit, 18) }
comp! { gcn,	"255",	many_m_n(13, 25, digit) }
comp! { sscc,	"00",	count(digit, 18) }
comp! { gdti,	"253",	recognize(pair(count(digit, 13), many_m_n(1, 17, xchar))) }
comp! { ginc,	"401",	many_m_n(1, 30, xchar) }
comp! { gsin,	"402",	count(digit, 17) }
comp! { grai,	"8003",	many_m_n(14, 30, xchar) }
comp! { giai,	"8004",	many_m_n(1, 30, xchar) }
