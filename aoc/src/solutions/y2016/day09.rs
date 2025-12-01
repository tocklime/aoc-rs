aoc_harness::aoc_main!(2016 day 9, part1 [p1] => 107_035, part2 [p2] => 11_451_628_995,
    example part1 "ADVENT" => 6,
    example part1 "A(1x5)BC" => 7,
);

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{digit1, none_of},
    combinator::all_consuming,
    multi::{many0, many1},
    IResult, Parser,
};

enum Seg {
    Literal(String),
    Repeated(usize, usize, Vec<Seg>),
}

impl Seg {
    fn shallow_depth(&self) -> usize {
        match self {
            Self::Literal(s) => s.len(),
            Self::Repeated(x, l, _) => x * l,
        }
    }
    fn full_depth(&self) -> usize {
        match self {
            Self::Literal(s) => s.len(),
            Self::Repeated(x, _, v) => x * v.iter().map(Self::full_depth).sum::<usize>(),
        }
    }
    fn expand_marker(i: &str) -> IResult<&str, Self> {
        let (i, (a, b)) = (tag("("), digit1, tag("x"), digit1, tag(")")).parse(i)
            .map(|(i, (_, a, _, b, _))| (i, (a, b)))?;
        let repeat_len = a.parse::<usize>().unwrap();
        let repeat_count = b.parse::<usize>().unwrap();
        let (i, repeated) = take(repeat_len)(i)?;
        let (_, p) = Self::line(repeated)?;
        Ok((i, Self::Repeated(repeat_count, repeat_len, p)))
    }

    fn literal(i: &str) -> IResult<&str, Self> {
        let (i, v) = many1(none_of("(")).parse(i)?;
        Ok((i, Self::Literal(v.into_iter().collect())))
    }

    fn line(i: &str) -> IResult<&str, Vec<Self>> {
        let (i, segs) = all_consuming(many0(alt((Self::expand_marker, Self::literal)))).parse(i)?;
        Ok((i, segs.into_iter().collect()))
    }

    fn parse(i: &str) -> Vec<Self> {
        Self::line(i).unwrap().1
    }
}

fn p1(input: &str) -> usize {
    Seg::parse(input.trim())
        .into_iter()
        .map(|x| x.shallow_depth())
        .sum()
}

fn p2(input: &str) -> usize {
    Seg::parse(input.trim())
        .into_iter()
        .map(|x| x.full_depth())
        .sum()
}
