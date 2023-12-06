use std::collections::HashMap;


use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, anychar, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

aoc_harness::aoc_main!(2022 day 21, part1 [p1] => 194_058_098_264_286, part2 [p2] => 3_592_056_845_086, example both EG => (152,301));

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
    Lit(u64),
    Op(char, &'a str, &'a str),
}
struct Monkeys<'a>(HashMap<&'a str, Action<'a>>);

impl<'a> Monkeys<'a> {
    fn from_str(s: &'a str) -> Self {
        let (_, lines) = separated_list1(
            newline,
            separated_pair(take(4_usize), tag(": "), parse_action),
        )(s)
        .unwrap();
        Self(lines.into_iter().collect())
    }
    fn eval<const NO_HUMN: bool>(&self, start: &str) -> Option<u64> {
        if NO_HUMN && start == "humn" {
            None
        } else {
            Some(match &self.0[start] {
                Action::Lit(x) => *x,
                Action::Op('+', a, b) => self.eval::<NO_HUMN>(a)? + self.eval::<NO_HUMN>(b)?,
                Action::Op('*', a, b) => self.eval::<NO_HUMN>(a)? * self.eval::<NO_HUMN>(b)?,
                Action::Op('-', a, b) => self.eval::<NO_HUMN>(a)? - self.eval::<NO_HUMN>(b)?,
                Action::Op('/', a, b) => self.eval::<NO_HUMN>(a)? / self.eval::<NO_HUMN>(b)?,
                _ => panic!(),
            })
        }
    }

    fn solve_humn(&self, start: &str, target: u64) -> u64 {
        if start == "humn" {
            target
        } else {
            match &self.0[start] {
                Action::Op(op, a, b) => {
                    match (self.eval::<true>(a), self.eval::<true>(b)) {
                        (Some(a_val), None) => {
                            //target = a_val - XXXX
                            let target_sub = match op {
                                '+' => target - a_val,
                                '-' => a_val - target,
                                '*' => target / a_val,
                                '/' => a_val / target,
                                _ => panic!(),
                            };
                            self.solve_humn(b, target_sub)
                        }
                        (None, Some(b_val)) => {
                            //target = XXXX - b_val
                            let target_sub = match op {
                                '+' => target - b_val,
                                '-' => b_val + target,
                                '*' => target / b_val,
                                '/' => b_val * target,
                                _ => panic!(),
                            };
                            self.solve_humn(a, target_sub)
                        }
                        _ => panic!(),
                    }
                }
                Action::Lit(_) => panic!(),
            }
        }
    }
}
fn parse_action(input: &str) -> IResult<&str, Action> {
    alt((
        map(complete::u64, Action::Lit),
        map(
            tuple((take(4_usize), tag(" "), anychar, tag(" "), take(4_usize))),
            |(a, _, o, _, b)| Action::Op(o, a, b),
        ),
    ))(input)
}
fn p1(input: &str) -> u64 {
    Monkeys::from_str(input).eval::<false>("root").unwrap()
}
fn p2(input: &str) -> u64 {
    let monkeys = Monkeys::from_str(input);
    let Action::Op(_, l, r) = monkeys.0["root"] else {panic!()};
    match (monkeys.eval::<true>(l), monkeys.eval::<true>(r)) {
        (None, Some(b)) => monkeys.solve_humn(l, b),
        (Some(a), None) => monkeys.solve_humn(r, a),
        _ => panic!(),
    }
}
