macro_rules! comp {
	($name:ident, $code:literal, $value:expr) => {
		fn $name(i: &str) -> nom::IResult<&str, &str> {
			nom::combinator::recognize(nom::sequence::pair(
				nom::sequence::preceded(
					nom::bytes::complete::tag("/"),
					nom::bytes::complete::tag($code),
				),
				nom::sequence::preceded(nom::bytes::complete::tag("/"), $value),
			))(i)
		}
	};
}
