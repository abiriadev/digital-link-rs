use nom::{branch::alt, character::complete::one_of, IResult};

pub fn digit(i: &str) -> IResult<&str, char> { one_of("0123456789")(i) }

pub fn alpha(i: &str) -> IResult<&str, char> {
	one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")(i)
}

pub fn alphanumeric(i: &str) -> IResult<&str, char> { alt((digit, alpha))(i) }

pub fn xchar(i: &str) -> IResult<&str, char> {
	alt((
		alphanumeric,
		one_of(r#""-._!%&+./*[]';:<>=?"#),
	))(i)
}

#[cfg(test)]
mod tests {
	use insta::assert_debug_snapshot;

	use super::*;

	#[test]
	fn colon_should_be_xsymbol() {
		assert_eq!(xchar(":"), Ok(("", ':')));
	}

	#[test]
	fn x_should_be_xsymbol() {
		assert_eq!(xchar("x"), Ok(("", 'x')));
	}

	#[test]
	fn hash_should_not_be_xsymbol() {
		assert_debug_snapshot!(xchar("#"));
	}
}
