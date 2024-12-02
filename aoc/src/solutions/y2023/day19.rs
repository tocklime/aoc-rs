use std::{cmp::Ordering, collections::HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, one_of},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::tuple,
    Parser,
};
use nom_supreme::{multi::collect_separated_terminated, ParserExt};
use utils::{nom::IResult, span::Span};

aoc_harness::aoc_main!(2023 day 19,
    generator X::parse_all,
    part1 [p1] => 399_284,
    part2 [p2] => 121_964_982_771_486,
    example both EG => (19114, 167_409_079_868_000));

#[derive(Debug)]
struct X<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}
impl<'a> X<'a> {
    fn parse_all(input: &'a str) -> Self {
        X::parse
            .all_consuming()
            .complete()
            .parse(input)
            .expect("parse")
            .1
    }
    fn parse(input: &'a str) -> IResult<'a, Self> {
        let (input, (workflows, _, parts)) = tuple((
            separated_list1(newline, Workflow::parse),
            tag("\n\n"),
            many1(Part::parse),
        ))(input)?;
        let workflows = workflows.into_iter().map(|x| (x.name, x)).collect();
        Ok((input, Self { workflows, parts }))
    }
    fn explore(&self) -> Vec<PartConstraints> {
        let mut ans = Vec::new();
        let pc = PartConstraints::new();
        let mut stack: Vec<(&str, PartConstraints)> = vec![("in", pc)];
        while let Some((pos, constraints)) = stack.pop() {
            match pos {
                "A" => ans.push(constraints),
                "R" => (),
                pos => {
                    let wf = &self.workflows[pos];
                    let final_constraint = wf.rules.iter().try_fold(constraints, |to_here, r| {
                        if let Some(x) = to_here.add(r.quality, r.check, false, r.value) {
                            stack.push((r.target, x));
                        }
                        to_here.add(r.quality, r.check, true, r.value)
                    });
                    if let Some(x) = final_constraint {
                        stack.push((wf.default, x));
                    }
                }
            }
        }
        ans
    }
}
#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    default: &'a str,
}
impl<'a> Workflow<'a> {
    fn parse(input: &'a str) -> IResult<'a, Self> {
        let (input, (name, _, rules, _, default, _)) = tuple((
            alpha1,
            tag("{"),
            separated_list1(tag(","), Rule::parse),
            tag(","),
            alpha1,
            tag("}"),
        ))(input)?;
        Ok((
            input,
            Self {
                name,
                rules,
                default,
            },
        ))
    }
    fn process(&self, part: &Part) -> &str {
        self.rules
            .iter()
            .find_map(|r| r.matches(part))
            .unwrap_or(self.default)
    }
}

#[derive(Debug, Clone)]
struct PartConstraints {
    constraints: [Span<u16>; 4],
}

impl PartConstraints {
    fn new() -> Self {
        Self {
            constraints: [Span::new(1, 4001); 4],
        }
    }
    fn to_span(ord: Ordering, invert: bool, cut: u16) -> Span<u16> {
        match (ord, invert) {
            (Ordering::Less, true) => Span::new(cut, 4001),
            (Ordering::Less, false) => Span::new(1, cut),
            (Ordering::Greater, true) => Span::new(1, cut + 1),
            (Ordering::Greater, false) => Span::new(cut + 1, 4001),
            _ => panic!(),
        }
    }
    fn count(&self) -> u64 {
        self.constraints.map(|x| x.size() as u64).iter().product()
    }
    fn add(&self, q: usize, ord: Ordering, invert: bool, val: u16) -> Option<Self> {
        let as_span = Self::to_span(ord, invert, val);
        let new_q = self.constraints[q].intersection(&as_span)?;
        let mut a = self.clone();
        a.constraints[q] = new_q;
        Some(a)
    }
}

fn parse_quality(input: &str) -> IResult<usize> {
    one_of("xmas")
        .map(|x| "xmas".chars().position(|t| t == x).unwrap())
        .parse(input)
}

#[derive(Debug)]
struct Rule<'a> {
    quality: usize,
    check: Ordering,
    value: u16,
    target: &'a str,
}

impl<'a> Rule<'a> {
    fn matches(&self, part: &Part) -> Option<&str> {
        (part.values[self.quality].cmp(&self.value) == self.check).then_some(self.target)
    }
    fn parse(input: &'a str) -> IResult<'a, Self> {
        let (input, (quality, check, value, _, target)) = tuple((
            parse_quality,
            alt((
                value(Ordering::Less, tag("<")),
                value(Ordering::Greater, tag(">")),
            )),
            complete::u16,
            tag(":"),
            alpha1,
        ))(input)?;
        Ok((
            input,
            Self {
                quality,
                check,
                value,
                target,
            },
        ))
    }
}

#[derive(Debug)]
struct Part {
    values: [u16; 4],
}

impl Part {
    fn parse(input: &str) -> IResult<Self> {
        let (input, values): (_, Vec<u16>) = collect_separated_terminated(
            parse_quality.precedes(tag("=")).precedes(complete::u16),
            tag(","),
            tag("}\n"),
        )
        .preceded_by(tag("{"))
        .parse(input)?;
        let values: [u16; 4] = std::array::from_fn(|n| values[n]);
        Ok((input, Self { values }))
    }
}

fn p1(prob: &X) -> u32 {
    prob.parts
        .iter()
        .map(|p| {
            let mut flow = "in";
            loop {
                match prob.workflows[flow].process(p) {
                    "A" => break p.values.iter().copied().map(u32::from).sum::<u32>(),
                    "R" => break 0,
                    x => flow = x,
                }
            }
        })
        .sum()
}
fn p2(prob: &X) -> u64 {
    prob.explore().iter().map(PartConstraints::count).sum()
}

const EG: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
