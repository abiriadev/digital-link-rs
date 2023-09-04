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

macro_rules! attr {
	(match ($attr: ident, $key: expr, $value: expr $(,)?) {
		$(
			$code: pat => ($name: ident, $parser: expr $(,)?)
		),* $(,)?
	}) => {
		match ($key) {
			$(
				$code => {
					if let Some((_, value)) =
						nom::combinator::all_consuming(
							nom::combinator::recognize($parser)
						)
							.parse($value)
							.ok()
					{
						$attr.$name = Some(value.to_owned());
					}
				},
			)*
			_ => todo!()
		}
	};
}
