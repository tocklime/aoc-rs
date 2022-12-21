use std::collections::HashMap;

use aoc_harness::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, anychar, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

aoc_main!(2022 day 21, part1 [p1], part2 [p2], example both EG => (152,301));

const EG: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
#[derive(Debug)]
enum Action<'a> {
    Lit(i64),
    Op(char, &'a str, &'a str),
}
fn parse_action<'a>(input: &'a str) -> IResult<&'a str, Action<'a>> {
    alt((
        map(complete::i64, Action::Lit),
        map(
            tuple((take(4_usize), tag(" "), anychar, tag(" "), take(4_usize))),
            |(a, _, o, _, b)| Action::Op(o, a, b),
        ),
    ))(input)
}
fn eval(map: &HashMap<&str, Action>, start: &str) -> i64 {
    match map.get(start) {
        Some(Action::Lit(x)) => *x,
        Some(Action::Op('+', a, b)) => eval(map, a) + eval(map, b),
        Some(Action::Op('*', a, b)) => eval(map, a) * eval(map, b),
        Some(Action::Op('-', a, b)) => eval(map, a) - eval(map, b),
        Some(Action::Op('/', a, b)) => eval(map, a) / eval(map, b),
        _ => panic!(),
    }
}
fn eval2(map: &HashMap<&str, Action>, start: &str) -> Option<i64> {
    if start == "humn" {
        None
    } else {
        Some(match map.get(start) {
            Some(Action::Lit(x)) => *x,
            Some(Action::Op('+', a, b)) => eval2(map, a)? + eval2(map, b)?,
            Some(Action::Op('*', a, b)) => eval2(map, a)? * eval2(map, b)?,
            Some(Action::Op('-', a, b)) => eval2(map, a)? - eval2(map, b)?,
            Some(Action::Op('/', a, b)) => eval2(map, a)? / eval2(map, b)?,
            _ => panic!(),
        })
    }
}
fn p1(input: &str) -> i64 {
    let (_, lines) = separated_list1(
        newline,
        separated_pair(take(4_usize), tag(": "), parse_action),
    )(input)
    .unwrap();
    let map: HashMap<&str, Action> = lines.into_iter().collect();
    eval(&map, "root")
}
fn figure_num_to_make(map: &HashMap<&str, Action>, who: &str, target: i64) -> i64 {
    println!("Need to make {} say {}...", who, target);
    if who == "humn" {
        target
    } else {
        match map.get(who) {
            Some(Action::Lit(x)) => *x,
            Some(Action::Op(op, a, b)) => {
                println!("  their op is {} {} {}", a, op, b);
                let a_val = eval2(map, a);
                let b_val = eval2(map, b);
                println!("  a_val is {:?}, b_val is {:?}", a_val, b_val);
                let val = [a_val, b_val].iter().find_map(|x| *x).unwrap();
                let target_sub = match (a_val.is_some(), op) {
                    (_, '+') => target - val,
                    (false, '-') => target + val,
                    (true, '-') => val - target,
                    (_, '*') => target / val,
                    (false, '/') => target * val,
                    (true, '/') => val / target,
                    _ => panic!(),
                };
                println!("  target is {}", target_sub);
                if a_val.is_none() {
                    figure_num_to_make(map, a, target_sub)
                } else {
                    figure_num_to_make(map, b, target_sub)
                }
            }
            _ => panic!(),
        }
    }
}
fn p2(input: &str) -> i64 {
    let (_, lines) = separated_list1(
        newline,
        separated_pair(take(4_usize), tag(": "), parse_action),
    )(input)
    .unwrap();
    let map: HashMap<&str, Action> = lines.into_iter().collect();
    let Action::Op(_, l, r) = map["root"] else {panic!()};
    let (a, b) = (eval2(&map, l), eval2(&map, r));
    dbg!(a, b);
    if a.is_none() {
        figure_num_to_make(&map, l, b.unwrap())
    } else {
        figure_num_to_make(&map, r, a.unwrap())
    }
}
