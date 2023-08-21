use nom::{
	branch::alt, bytes::complete::tag, character::complete::one_of,
	combinator::recognize, IResult,
};

pub fn digit(i: &str) -> IResult<&str, char> { one_of("0123456789")(i) }

pub fn alpha(i: &str) -> IResult<&str, char> {
	one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")(i)
}

pub fn alphanumeric(i: &str) -> IResult<&str, char> { alt((digit, alpha))(i) }

pub fn xchar(i: &str) -> IResult<&str, &str> {
	alt((
		recognize(alphanumeric),
		tag(r#"""#),
		tag("-"),
		tag("."),
		tag("_"),
		tag("%21"),
		tag("%25"),
		tag("%26"),
		tag("%2B"),
		tag("%2C"),
		tag("%2F"),
		tag("%2A"),
		tag("%28"),
		tag("%29"),
		tag("%27"),
		tag("%3B"),
		tag("%3A"),
		tag("%3C"),
		tag("%3E"),
		tag("%3D"),
		tag("%3F"),
	))(i)
}

#[cfg(test)]
mod tests {
	use insta::assert_debug_snapshot;

	use super::*;

	#[test]
	fn colon_should_be_xsymbol() {
		assert_eq!(xchar(":"), Ok(("", ":")));
	}

	#[test]
	fn x_should_be_xsymbol() {
		assert_eq!(xchar("x"), Ok(("", "x")));
	}

	#[test]
	fn hash_should_not_be_xsymbol() {
		assert_debug_snapshot!(xchar("#"));
	}
}
