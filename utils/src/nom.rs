use nom::{character::complete::multispace0, error::ParseError, sequence::delimited, Parser};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub type NomError<'a> = nom_supreme::error::ErrorTree<&'a str>;
pub type IResult<'a, T> = nom::IResult<&'a str, T, NomError<'a>>;

pub fn parse_all<'a, F, O>(input: &'a str, inner: F) -> O
where
    F: Parser<&'a str, O, NomError<'a>>,
{
    let parsed: Result<O, NomError<'a>> =
        nom_supreme::final_parser::final_parser::<_, _, _, NomError<'a>>(inner)(input);
    parsed.unwrap()
}
