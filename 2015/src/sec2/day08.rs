use nom::bytes::complete::tag;
use nom::IResult;
use nom::multi::many0;
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::character::complete::none_of;

fn quote(i: &str) -> IResult<&str, char> {
    let (i, _) = tag("\\\"")(i)?;
    Ok((i, '"'))
}

fn backslash(i: &str) -> IResult<&str, char> {
    let (i, _) = tag("\\\\")(i)?;
    Ok((i, '\\'))
}

fn hex(i: &str) -> IResult<&str, char> {
    let (i, _) = tag("\\x")(i)?;
    let (i, hex) = take(2_usize)(i)?;
    let val = u8::from_str_radix(hex, 16).expect("Bad hex digits");
    Ok((i, char::from(val)))
}

fn character(i: &str) -> IResult<&str, char> {
    alt((quote, hex, backslash, none_of("\"")))(i)
}

pub fn interpret_line(i: &str) -> IResult<&str, String> {
    let (i, _) = tag("\"")(i)?;
    let (i, s) = many0(character)(i)?;
    let (i, _) = tag("\"")(i)?;
    Ok((i, s.into_iter().collect()))
}

#[aoc(day8, part1)]
pub fn p1(input: &str) -> usize {
    input.lines().map(|l| {
        let code = l.len();
        let (_, mem) = interpret_line(l).unwrap();
        code - mem.chars().count()
    }).sum()
}


#[aoc(day8, part2)]
pub fn p2(input: &str) -> usize {
    input.lines().map(|l| {
        let num_quotes = l.chars().filter(|&c| c == '\"').count();
        let num_backslash = l.chars().filter(|&c| c == '\\').count();
        2 + num_quotes + num_backslash
    }).sum()
}

