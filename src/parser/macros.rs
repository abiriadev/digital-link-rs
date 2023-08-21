macro_rules! comp {
	($name:ident, $code:literal, $value:expr) => {
		pub fn $name(i: &str) -> nom::IResult<&str, &str> {
			nom::sequence::preceded(
				nom::sequence::tuple((
					nom::bytes::complete::tag("/"),
					nom::bytes::complete::tag($code),
					nom::bytes::complete::tag("/"),
				)),
				nom::combinator::recognize($value),
			)(i)
		}
	};
}
