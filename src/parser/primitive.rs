use nom::{
	character::complete::one_of,
	error::{Error, ErrorKind, ParseError},
	IResult,
};

fn xchar(i: &str) -> IResult<&str, char> {
	one_of(
		r#"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"-._!%&+./*[]';:<>=?"#,
	)(i)
}

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
	assert_eq!(
		xchar("#"),
		Err(nom::Err::Error(Error::from_error_kind(
			"#",
			ErrorKind::OneOf
		)))
	);
}
