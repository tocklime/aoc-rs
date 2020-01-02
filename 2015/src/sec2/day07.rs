#![allow(clippy::implicit_hasher)]
use nom::IResult;
use nom::character::complete::{alpha1, digit1};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::combinator::all_consuming;
use nom::sequence::tuple;
use std::collections::HashMap;

fn item(i: &str) -> IResult<&str, Item> {
    alt((item_const, item_name))(i)
}

fn item_const(i: &str) -> IResult<&str, Item> {
    digit1(i).map(|(i, a)| (i, Item::Const(a.parse().unwrap())))
}

fn item_name(i: &str) -> IResult<&str, Item> {
    alpha1(i).map(|(i, a)| (i, Item::Ref(a)))
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
    tuple((item, tag(" OR "), item))(i)
        .map(|(i, (a, _, b))| (i, LineOp::Or(a, b)))
}

fn lhs_and(i: &str) -> IResult<&str, LineOp> {
    tuple((item, tag(" AND "), item))(i)
        .map(|(i, (a, _, b))| (i, LineOp::And(a, b)))
}

fn lhs_const(i: &str) -> IResult<&str, LineOp> {
    item(i).map(|(i, a)| (i, LineOp::Const(a)))
}

fn lhs(i: &str) -> IResult<&str, LineOp> {
    alt((lhs_and, lhs_or, lhs_not, lhs_lshift, lhs_rshift, lhs_const))(i)
}

fn line(i: &str) -> IResult<&str, Line> {
    all_consuming(tuple((lhs, tag(" -> "), alpha1)))(i)
        .map(|(i, (lhs, _, rhs))| (i, Line { lhs, rhs }))
}

#[derive(Debug)]
pub enum Item<'a> {
    Const(u16),
    Ref(&'a str),
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

pub fn gen(input: &str) -> Vec<Line> {
    input.lines().map(|x|
        {
            line(x).unwrap().1
        }
    ).collect()
}

pub fn find_value<'a>(by_name: &'a HashMap<&'a str, Line<'a>>, known_values: &mut HashMap<&'a str, u16>, target: &'a Item) -> u16 {
    match target {
        Item::Const(v) => *v,
        Item::Ref(target) => {
            if let Some(x) = known_values.get(target) {
                *x
            }else {
                let l = by_name.get(target).expect("Unknown wire");
                let val = match &l.lhs {
                    LineOp::Const(a) => find_value(by_name,known_values,&a),
                    LineOp::Not(a ) => !find_value(by_name,known_values,&a),
                    LineOp::And(a,b) => find_value(by_name,known_values,&a) & find_value(by_name,known_values,&b),
                    LineOp::Or(a,b) => find_value(by_name,known_values,&a) | find_value(by_name,known_values,&b),
                    LineOp::LShift(a,b) => find_value(by_name,known_values,&a) << find_value(by_name,known_values,&b),
                    LineOp::RShift(a,b) => find_value(by_name,known_values,&a) >> find_value(by_name,known_values,&b),
                };
                known_values.insert(target, val);
                val
            }
        }
    }

}

#[aoc(day7, part1)]
pub fn p1(input: &str) -> u16 {
    let parsed = gen(input);
    let by_name = parsed.into_iter().map(|l| (l.rhs,l)).collect();
    find_value(&by_name, &mut HashMap::new(), &Item::Ref("a"))
}

#[aoc(day7, part2)]
pub fn p2(input: &str) -> u16 {
    let parsed = gen(input);
    let by_name = parsed.into_iter().map(|l| (l.rhs,l)).collect();
    let a_val = find_value(&by_name, &mut HashMap::new(), &Item::Ref("a"));
    let mut values = HashMap::new();
    values.insert("b",a_val);
    find_value(&by_name, &mut values, &Item::Ref("a"))
}
