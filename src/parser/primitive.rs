use insta::{assert_debug_snapshot, assert_yaml_snapshot};
use nom::{
	character::complete::one_of,
	error::{Error, ErrorKind, ParseError},
	Err, IResult,
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
	assert_debug_snapshot!(xchar("#"));
}
