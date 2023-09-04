use nom::{
	branch::alt, bytes::complete::tag, character::complete::one_of,
	combinator::recognize, IResult,
};

#[allow(unused)]
pub fn bool(inp: &str) -> IResult<&str, &str> { alt((tag("0"), tag("1")))(inp) }

pub fn digit(i: &str) -> IResult<&str, &str> {
	recognize(one_of("0123456789"))(i)
}

pub fn upper_alpha(i: &str) -> IResult<&str, &str> {
	recognize(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))(i)
}

pub fn lower_alpha(i: &str) -> IResult<&str, &str> {
	recognize(one_of("abcdefghijklmnopqrstuvwxyz"))(i)
}

pub fn alpha(i: &str) -> IResult<&str, &str> {
	alt((upper_alpha, lower_alpha))(i)
}

pub fn alphanumeric(i: &str) -> IResult<&str, &str> { alt((digit, alpha))(i) }

pub fn xchar(i: &str) -> IResult<&str, &str> {
	alt((
		alphanumeric,
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

pub fn ychar(i: &str) -> IResult<&str, &str> {
	alt((
		digit,
		upper_alpha,
		tag("-"),
		tag("%23"),
		tag("%2F"),
	))(i)
}

#[cfg(test)]
mod tests {
	use insta::assert_debug_snapshot;

	use super::*;

	#[test]
	fn colon_should_not_be_xchar() {
		assert_debug_snapshot!(xchar(":"));
	}

	#[test]
	fn escaped_colon_should_be_xchar() {
		assert_eq!(xchar("%3A"), Ok(("", "%3A")));
	}

	#[test]
	fn question_mark_should_not_be_xchar() {
		assert_debug_snapshot!(xchar("?"));
	}

	#[test]
	fn escaped_question_mark_should_be_xchar() {
		assert_eq!(xchar("%3F"), Ok(("", "%3F")));
	}

	#[test]
	fn escaped_question_mark_should_be_ychar() {
		assert_debug_snapshot!(ychar("%3F"));
	}

	#[test]
	fn x_should_be_xchar() {
		assert_eq!(xchar("x"), Ok(("", "x")));
	}

	#[test]
	fn hash_should_not_be_xchar() {
		assert_debug_snapshot!(xchar("#"));
	}

	#[test]
	fn escaped_hash_should_not_be_xchar() {
		assert_debug_snapshot!(xchar("%23"));
	}

	#[test]
	fn hash_should_not_be_ychar() {
		assert_debug_snapshot!(ychar("#"));
	}

	#[test]
	fn escaped_hash_should_not_be_ychar() {
		assert_eq!(ychar("%23"), Ok(("", "%23")));
	}
}
