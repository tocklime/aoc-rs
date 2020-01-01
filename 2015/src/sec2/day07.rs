use nom::IResult;
use nom::character::complete::{alpha1, digit1};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::combinator::all_consuming;
use nom::sequence::tuple;
/*
NOT dq -> dr
kg OR kf -> kh
ep OR eo -> eq
44430 -> b
*/
fn item(i: &str) -> IResult<&str, Item> {
    alt((item_const,item_name))(i)
}
fn item_const(i: &str) -> IResult<&str, Item> {
    digit1(i).map(|(i,a)| (i,Item::Const(a.parse().unwrap())))
}
fn item_name(i: &str) -> IResult<&str, Item> {
    alpha1(i).map(|(i,a)| (i,Item::Ref(a)))
}

fn lhs_not(i: &str) -> IResult<&str, LineOp> {
    let (i, _) = tag("NOT ")(i)?;
    let (i, item) = item(i)?;
    Ok((i, LineOp::Not(item)))
}

fn lhs_lshift(i: &str) -> IResult<&str, LineOp> {
    tuple((item, tag(" LSHIFT "), item))(i)
        .map(|(i, (a, _, b))| (i, LineOp::LShift(a, b)))
}

fn lhs_rshift(i: &str) -> IResult<&str, LineOp> {
    tuple((item, tag(" RSHIFT "), item))(i)
        .map(|(i, (a, _, b))| (i, LineOp::RShift(a, b)))
}

fn lhs_or(i: &str) -> IResult<&str, LineOp> {
    tuple((item,tag(" OR "),item))(i)
        .map(|(i,(a,_,b))| (i, LineOp::Or(a, b)))
}

fn lhs_and(i: &str) -> IResult<&str, LineOp> {
    tuple((item,tag(" AND "),item))(i)
        .map(|(i,(a,_,b))| (i, LineOp::And(a, b)))
}
fn lhs_const(i: &str) -> IResult<&str, LineOp> {
    item(i).map(|(i,a)| (i, LineOp::Const(a)))
}

fn lhs(i: &str) -> IResult<&str, LineOp> {
    alt((lhs_and, lhs_or, lhs_not, lhs_lshift, lhs_rshift,lhs_const))(i)
}

fn line(i: &str) -> IResult<&str, Line> {
    all_consuming(tuple((lhs, tag(" -> "), alpha1)))(i)
        .map(|(i, (lhs, _, rhs))| (i, Line { lhs, rhs }))
}

#[derive(Debug)]
pub enum Item<'a> {
    Const(u16),
    Ref(&'a str)
}
#[derive(Debug)]
pub enum LineOp<'a> {
    Const(Item<'a>),
    Not(Item<'a>),
    Or(Item<'a>, Item<'a>),
    And(Item<'a>, Item<'a>),
    LShift(Item<'a>, Item<'a>),
    RShift(Item<'a>, Item<'a>),
}

#[derive(Debug)]
pub struct Line<'a> {
    lhs: LineOp<'a>,
    rhs: &'a str,
}

impl<'a> LineOp<'a> {
    fn items(&self) -> Vec<&Item<'a>> {
        match self {
            LineOp::Const(a) => vec![a],
            LineOp::Not(a) => vec![a],
            LineOp::Or(a,b) => vec![a,b],
            LineOp::And(a,b) => vec![a,b],
            LineOp::LShift(a,b) => vec![a,b],
            LineOp::RShift(a,b) => vec![a,b],
        }
    }
}

pub fn gen(input: &str) -> Vec<Line> {
    input.lines().map(|x|
        {
            line(x).unwrap().1
        }
    ).collect()
}

#[aoc(day7, part1)]
pub fn p1(input: &str) -> usize {
    let parsed = gen(input);
    println!("{:?}", parsed);
    parsed.len()
}