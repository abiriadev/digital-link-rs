use nom::{character::complete::one_of, IResult};

fn xchar(i: &str) -> IResult<&str, char> {
	one_of(
		r#"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"-._!%&+./*[]';:<>=?"#,
	)(i)
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
