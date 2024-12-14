use std::{fmt::Debug, str::FromStr};

use nom::InputIter;

#[allow(clippy::missing_errors_doc)]
pub fn try_parse_many<T, E>(input: &str, sep: &str) -> Result<Vec<T>, E>
where
    T: FromStr<Err = E>,
{
    input.split(sep).map(str::parse).collect()
}
#[must_use]
pub fn parse_input_from_str_sep_by<T>(input: &str, sep: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input_from_str_sep_by(input, sep, |x| {
        x.parse().expect("Could not parse item from input")
    })
}

pub fn input_from_str_sep_by<'a, T, F>(input: &'a str, sep: &str, f: F) -> Vec<T>
where
    F: Fn(&'a str) -> T + 'a,
{
    input.trim().split(sep).map(|x| f(x.trim())).collect()
}

struct NumParser<'a> {
    input: &'a str,
}

struct ThingParser<'a, N> {
    input: &'a str,
    parser: Box<dyn nom::Parser<&'a str, N, ()> + 'a>
}

pub fn find_things<'a, N,F>(input: &'a str, parser: F) -> impl Iterator<Item = N> + use<'a, N, F>
where F : nom::Parser<&'a str, N, ()> + 'a
{
    ThingParser {input, parser: Box::new(parser)}
}
impl<N> Iterator for ThingParser<'_,N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Ok((i,n)) = self.parser.parse(self.input) {
                self.input = i;
                return Some(n);
            } else {
                self.input = &self.input[1..];
                if self.input.is_empty() {
                    return None;
                }
            }
        }
    }
}

pub fn parse_numbers<'a>(input: &'a str) -> impl Iterator<Item = u64> + use<'a>{
    NumParser{input}
}

impl Iterator for NumParser<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        //find first digit.
        let ix = self.input.position(|c|char::is_ascii_digit(&c))?;
        let end_ix = ix + (&self.input[ix..]).position(|c| !char::is_ascii_digit(&c)).unwrap_or_else(|| self.input.len() - ix);
        let num = u64::from_str(&self.input[ix..end_ix]).unwrap();
        self.input = &self.input[end_ix..];
        Some(num)
    }
}
