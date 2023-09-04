use nom::{
	multi::{count, many_m_n},
	sequence::pair,
};

use super::primitive::{digit, xchar};

comp! { cpv,		"22",	many_m_n(1, 20, xchar) }
comp! { lot,		"10",	many_m_n(1, 20, xchar) }
comp! { ser,		"21",	many_m_n(1, 20, xchar) }
comp! { cpsn,		"8011",	many_m_n(1, 12, digit) }
comp! { glnx,		"254",	many_m_n(1, 20, xchar) }
comp! { refno,		"8020",	many_m_n(1, 25, xchar) }
comp! { srin,		"8019",	many_m_n(1, 10, digit) }
comp! { tpx,		"235",	many_m_n(1, 28, xchar) }
comp! { uic_ext,	"7040",	pair(digit, count(xchar, 3)) }
