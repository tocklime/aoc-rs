use nom::{
    IResult,
    bytes::complete::{tag,take},
    character::complete::{digit1, none_of},
    multi::{many0, many1},
    branch::alt,
    combinator::all_consuming,
    sequence::tuple
};

enum Seg {
    Literal(String),
    Repeated(usize,usize,Vec<Seg>),
}

impl Seg {
    fn shallow_depth(&self) -> usize {
        match self {
            Self::Literal(s) => s.len(),
            Self::Repeated(x,l, _) => x * l,
        }
    }
    fn full_depth(&self) -> usize {
        match self {
            Self::Literal(s) => s.len(),
            Self::Repeated(x,_,v) => x * v.iter().map(|x| x.full_depth()).sum::<usize>()
        }
    }
    fn expand_marker(i: &str) -> IResult<&str, Self> {
        let (i,(a,b)) = tuple((tag("("),digit1,tag("x"),digit1,tag(")")))(i)
            .map(|(i,(_,a,_,b,_))| (i,(a,b)))?;
        let repeat_len = a.parse::<usize>().unwrap();
        let repeat_count = b.parse::<usize>().unwrap();
        let (i, repeated) = take(repeat_len)(i)?;
        let (_, p) = Self::line(repeated)?;
        Ok((i, Self::Repeated(repeat_count,repeat_len,p)))
    }

    fn literal(i: &str) -> IResult<&str, Self> {
        let (i, v) = many1(none_of("("))(i)?;
        Ok((i, Self::Literal(v.into_iter().collect())))
    }

    fn line(i: &str) -> IResult<&str, Vec<Self>> {
        let (i, segs) = all_consuming(many0(alt((Self::expand_marker, Self::literal))))(i)?;
        Ok((i, segs.into_iter().collect()))
    }

    fn parse(i: &str) -> Vec<Self> {
        Self::line(i).unwrap().1
    }
}


#[aoc(day9, part1)]
fn p1(input: &str) -> usize {
    Seg::parse(input).into_iter().map(|x| x.shallow_depth()).sum()
}
#[aoc(day9, part2)]
fn p2(input: &str) -> usize {
    Seg::parse(input).into_iter().map(|x| x.full_depth()).sum()
}

