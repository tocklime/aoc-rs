use std::{cmp::Ordering, collections::HashMap};

use nom::{
    character::complete::{self, alpha1, newline, one_of},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    Parser,
};
use nom_supreme::{multi::collect_separated_terminated, tag::complete::tag, ParserExt};
use petgraph::Graph;
use rustc_hash::FxHashMap;
use utils::nom::IResult;

aoc_harness::aoc_main!(2023 day 19, part1 [p1], part2 [p2], example both EG => (19114, 167_409_079_868_000));

#[derive(Debug)]
struct X {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}
impl X {
    fn parse(input: &str) -> IResult<Self> {
        let (input, (workflows, _, parts)) = tuple((
            separated_list1(newline, Workflow::parse),
            tag("\n\n"),
            many1(Part::parse),
        ))(input)?;
        let workflows = workflows.into_iter().map(|x| (x.name.clone(), x)).collect();
        Ok((input, Self { workflows, parts }))
    }
    fn explore(&self) -> Vec<PartConstraints> {
        let mut ans = Vec::new();
        let pc = PartConstraints::default();
        let mut stack = vec![("in", pc)];
        while let Some((pos, constraints)) = stack.pop() {
            if pos == "A" {
                ans.push(constraints);
            } else if pos == "R" {
                //...
            } else {
                let wf = &self.workflows[pos];
                let mut elses: Vec<(Quality, Ordering, u16)> = Vec::new();
                for r in &wf.rules {
                    let mut new_cons = constraints.clone();
                    for e in &elses {
                        new_cons.add(e.0, e.1, true, e.2);
                    }
                    new_cons.add(r.quality, r.check, false, r.value);
                    stack.push((&r.target, new_cons));
                    elses.push((r.quality, r.check, r.value));
                }
                let mut new_cons = constraints.clone();
                for e in &elses {
                    new_cons.add(e.0, e.1, true, e.2);
                }
                stack.push((&wf.default, new_cons));
            }
        }
        ans
    }
    fn count_paths<'a>(&'a self, start: &'a str, cache: &mut FxHashMap<&'a str, usize>) -> usize {
        if let Some(&n) = cache.get(&start) {
            return n;
        }
        println!("Routes from {start}...");
        let count = if start == "A" {
            1
        } else {
            self.workflows
                .values()
                .map(|w: &Workflow| {
                    // dbg!(w);
                    w.rules
                        .iter()
                        .map(|r| self.count_paths(&r.target, cache))
                        .sum::<usize>()
                        + self.count_paths(&w.default, cache)
                })
                .sum()
        };
        cache.insert(start, count);
        count
    }
}
#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<CheckRule>,
    default: String,
}
impl Workflow {
    fn parse(input: &str) -> IResult<Self> {
        let (input, (name, _, rules, _, default, _)) = tuple((
            alpha1.map(|x: &str| x.to_owned()),
            tag("{"),
            separated_list1(tag(","), CheckRule::parse),
            tag(","),
            alpha1.map(|x: &str| x.to_owned()),
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
    fn sort(&self, part: &Part) -> &str {
        self.rules
            .iter()
            .find_map(|r| r.matches(part))
            .unwrap_or(&self.default)
    }
}

#[derive(Debug, Default, Clone)]
struct PartConstraints {
    constraints: HashMap<Quality, Vec<(Ordering, bool, u16)>>,
}

impl PartConstraints {
    fn count(&self) -> u64 {
        [Quality::Aerodynamic,Quality::ExtremelyCool,Quality::Musical,Quality::Shiny]
        .map(|q| {
            self.constraints.get(&q).map(|v| {
                (1..=4000)
                    .filter(|n| {
                        v.iter()
                            .all(|(ord, invert, val)| (n.cmp(val) == *ord) != *invert)
                    })
                    .count() as u64
            }).unwrap_or(4000)
        }).iter().product()
    }
    fn add(&mut self, q: Quality, ord: Ordering, invert: bool, val: u16) {
        self.constraints
            .entry(q)
            .or_default()
            .push((ord, invert, val));
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Quality {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}
impl Quality {
    fn parse(input: &str) -> IResult<Self> {
        one_of("xmas")
            .map(|x| match x {
                'x' => Quality::ExtremelyCool,
                'm' => Quality::Musical,
                'a' => Quality::Aerodynamic,
                's' => Quality::Shiny,
                _ => panic!(),
            })
            .parse(input)
    }
    fn to_char(&self) -> char {
        match self {
            Quality::ExtremelyCool => 'x',
            Quality::Musical => 'm',
            Quality::Aerodynamic => 'a',
            Quality::Shiny => 's',
        }
    }
}

#[derive(Debug)]
enum RuleTypes {
    Default(String),
    Checked(CheckRule),
}

#[derive(Debug)]
struct CheckRule {
    quality: Quality,
    check: Ordering,
    value: u16,
    target: String,
}

impl CheckRule {
    fn matches(&self, part: &Part) -> Option<&str> {
        (part.values[&self.quality].cmp(&self.value) == self.check).then_some(&self.target)
    }
    fn parse(input: &str) -> IResult<Self> {
        let (input, (quality, check, value, _, target)) = tuple((
            Quality::parse,
            one_of("<>").map(|x| {
                if x == '<' {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }),
            complete::u16,
            tag(":"),
            alpha1.map(|x: &str| x.to_owned()),
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
    values: HashMap<Quality, u16>,
}

impl Part {
    fn parse(input: &str) -> IResult<Self> {
        let (input, values) = collect_separated_terminated(
            separated_pair(Quality::parse, tag("="), complete::u16),
            tag(","),
            tag("}\n"),
        )
        .preceded_by(tag("{"))
        .parse(input)?;
        Ok((input, Self { values }))
    }
}

fn p1(input: &str) -> u32 {
    let (input, prob) = X::parse(input).unwrap();
    assert_eq!(input, "");
    let mut total = 0;
    for p in prob.parts {
        let mut flow = "in";
        while flow != "A" && flow != "R" {
            flow = prob.workflows[flow].sort(&p);
        }
        if flow == "A" {
            total += p.values.values().copied().map(u32::from).sum::<u32>();
        }
    }
    total
}
fn shw(x: Ordering) -> char {
    match x {
        Ordering::Less => '<',
        Ordering::Equal => '=',
        Ordering::Greater => '>',
    }
}
fn p2(input: &str) -> u64 {
    let (input, prob) = X::parse(input).unwrap();
    assert_eq!(input, "");
    // println!("digraph {{");
    // for w in prob.workflows.values() {
    //     for r in &w.rules {
    //         println!(
    //             "{} -> {} [label = \"{}{}{}\"]",
    //             w.name,
    //             r.target,
    //             r.quality.to_char(),
    //             shw(r.check),
    //             r.value
    //         );
    //     }
    //     println!("{} -> {}", w.name, w.default);
    // }
    // println!("}}");

    //what are all possible A-routes?
    let a_routes = prob.explore();
    // dbg!(&a_routes);
    a_routes.iter().map(PartConstraints::count).sum()
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
