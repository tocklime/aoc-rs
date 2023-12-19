use std::{cmp::Ordering, collections::HashMap};

use nom::{
    character::complete::{self, alpha1, newline, one_of},
    multi::{many1, separated_list1},
    sequence::tuple,
    Parser,
};
use nom_supreme::{multi::collect_separated_terminated, tag::complete::tag, ParserExt};
use num_enum::IntoPrimitive;
use utils::{nom::IResult, span::Span};

aoc_harness::aoc_main!(2023 day 19, part1 [p1] => 399_284, part2 [p2] => 121_964_982_771_486, example both EG => (19114, 167_409_079_868_000));

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
    constraints: [Vec<(Ordering, bool, u16)>; 4],
}

impl PartConstraints {
    fn count_quality_options(cs: &Vec<(Ordering, bool, u16)>) -> u64 {
        let mut span = Span::new(1,4001u16);
        for &(ord, invert, cut) in cs {
            let as_span = match (ord,invert) {
                (Ordering::Less, true) => Span::new(cut, 4001),
                (Ordering::Less, false) => Span::new(1, cut),
                (Ordering::Greater, true) => Span::new(1, cut+1),
                (Ordering::Greater, false) => Span::new(cut+1, 4001),
                _ => panic!()
            };
            match span.intersection(&as_span) {
                Some(x) => span = x,
                None => return 0
            }
        }
        span.size() as u64
    }
    fn count(&self) -> u64 {
        self.constraints
            .iter()
            .map(PartConstraints::count_quality_options)
            .product()
    }
    fn add(&mut self, q: Quality, ord: Ordering, invert: bool, val: u16) {
        self.constraints[u8::from(q) as usize].push((ord, invert, val));
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, IntoPrimitive)]
#[repr(u8)]
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
        (part.values[u8::from(self.quality) as usize].cmp(&self.value) == self.check)
            .then_some(&self.target)
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
    values: [u16; 4],
}

impl Part {
    fn parse(input: &str) -> IResult<Self> {
        let (input, values): (_, Vec<u16>) = collect_separated_terminated(
            Quality::parse.precedes(tag("=")).precedes(complete::u16),
            tag(","),
            tag("}\n"),
        )
        .preceded_by(tag("{"))
        .parse(input)?;
        let values: [u16; 4] = std::array::from_fn(|n| values[n]);
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
            total += p.values.iter().copied().map(u32::from).sum::<u32>();
        }
    }
    total
}
fn p2(input: &str) -> u64 {
    let (input, prob) = X::parse(input).unwrap();
    assert_eq!(input, "");
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
