use nom::{Parser, character::complete::multispace0, error::ParseError, sequence::delimited};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

pub type NomError<'a> = nom::error::Error<&'a str>;
pub type IResult<'a, T> = nom::IResult<&'a str, T, NomError<'a>>;

pub fn parse_all<'a, F, O, E>(input: &'a str, mut inner: F) -> O
where
    F: Parser<&'a str, Output = O, Error = E>,
    E: std::fmt::Debug,
{
    let x = &mut inner;
    let (x, o) = x.parse_complete(input).expect("Parse OK");
    assert_eq!(x, "");
    o
}
