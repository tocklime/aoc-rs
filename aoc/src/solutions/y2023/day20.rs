use std::{
    collections::{HashMap, VecDeque},
    future::pending,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::{success, value},
    multi::separated_list1,
    sequence::{tuple, preceded},
    Parser,
};
use nom_supreme::ParserExt;
use utils::{collections::VecLookup, nom::IResult};

aoc_harness::aoc_main!(2023 day 20, part1 [p1::<1000>], part2 [p2], example part1 EG => 32000000, example part1 EG2 => 11687500);

#[derive(Clone, PartialEq, Eq, Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}
impl ModuleType {
    fn parse(input: &str) -> IResult<Self> {
        alt((
            value(ModuleType::FlipFlop(false), tag("%")),
            value(ModuleType::Conjunction(HashMap::new()), tag("&")),
            success(ModuleType::Broadcast),
        ))(input)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Module {
    name: String,
    typ: ModuleType,
    targets: Vec<String>,
}
impl Module {
    fn parse(input: &str) -> IResult<Self> {
        let (input, (typ, name, targets)) = tuple((
            ModuleType::parse,
            alpha1.terminated(tag(" -> ")),
            separated_list1(tag(", "), alpha1),
        ))(input)?;
        let name = name.to_string();
        let targets = targets
            .into_iter()
            .map(std::borrow::ToOwned::to_owned)
            .collect();
        Ok((input, Self { name, typ, targets }))
    }
    fn add_input(&mut self, name: &str) {
        if let ModuleType::Conjunction(inputs) = &mut self.typ {
            inputs.insert(name.to_string(), false);
        }
    }
    fn handle_pulse(&mut self, source: &str, value: bool) -> Option<bool> {
        match &mut self.typ {
            ModuleType::Broadcast => Some(value),
            ModuleType::FlipFlop(b) => {
                if value {
                    None
                } else {
                    *b = !*b;
                    Some(*b)
                }
            }
            ModuleType::Conjunction(inps) => {
                if let Some(x) = inps.get_mut(source) {
                    *x = value;
                    Some(!inps.values().all(|x| *x))
                } else {
                    panic!("Can't find input {source} for {}", &self.name);
                }
            }
        }
    }
}
fn p1<const N: usize>(input: &str) -> u32 {
    let (_, modules) = separated_list1(newline, Module::parse)
        .terminated(newline)
        .all_consuming()
        .complete()
        .parse(input)
        .expect("parse");
    let mut lookup: HashMap<String, Module> =
        modules.into_iter().map(|x| (x.name.clone(), x)).collect();
    let mut input_maps: HashMap<String, Vec<String>> = HashMap::new();
    for m in lookup.values() {
        for t in &m.targets {
            input_maps
                .entry(t.to_owned())
                .or_default()
                .push(m.name.to_owned());
        }
    }
    for (target, sources) in input_maps {
        if let Some(t) = lookup.get_mut(&target) {
            for s in sources {
                t.add_input(&s);
            }
        }
    }
    let mut counts = [0; 2];
    for _ in 0..N {
        let mut pending_signals = VecDeque::new();
        pending_signals.push_back(("button".to_owned(), false, "broadcaster".to_owned()));
        while let Some((source, val, name)) = pending_signals.pop_front() {
            // println!("{source} -{}-> {name}", if val { "high"} else {"low"});
            counts[usize::from(val)] += 1;
            if let Some(me) = lookup.get_mut(&name) {
                if let Some(value) = me.handle_pulse(&source, val) {
                    for out in &me.targets {
                        pending_signals.push_back((me.name.clone(), value, out.clone()));
                    }
                }
            }
        }
    }
    counts[0] * counts[1]
}
fn p2(input: &str) -> usize {
    let (_, modules) = separated_list1(newline, Module::parse)
        .terminated(newline)
        .all_consuming()
        .complete()
        .parse(input)
        .expect("parse");
    let mut lookup: HashMap<String, Module> =
        modules.into_iter().map(|x| (x.name.clone(), x)).collect();
    let mut input_maps: HashMap<String, Vec<String>> = HashMap::new();
    // println!("digraph {{");
    for m in lookup.values() {
        // let ty = match m.typ {
        //     ModuleType::Broadcast => "",
        //     ModuleType::FlipFlop(_) => "%",
        //     ModuleType::Conjunction(_) => "&",
        // };
        // println!("{} [label = \"{}: {}\"]", &m.name, &m.name, ty);
        for (ix, t) in m.targets.iter().enumerate() {
            // println!("{} -> {t} [label = \"{ix}\"]", &m.name);
            input_maps
                .entry(t.to_owned())
                .or_default()
                .push(m.name.to_owned());
        }
    }
    // println!("}}");
    for (target, sources) in input_maps {
        if let Some(t) = lookup.get_mut(&target) {
            for s in sources {
                t.add_input(&s);
            }
        }
    }
    let mut hit_count = 0;
    let mut last_hit = 0;
    //vj= 3732 + 3733n
    //xk= 3792 + 3793n
    //gr= 3946 + 3947n
    //fb= 4056 + 4057n
    //find n,m s.t.
    // vj == xk:
    // 3732 + 3733n = 3792 + 3793m
    // ____   3733n = 60 + 3793m

    

    let start = std::time::Instant::now();

    for press_count in 1.. {
        let mut pending_signals = VecDeque::new();
        pending_signals.push_back(("button".to_owned(), false, "fb".to_owned()));
        while let Some((source, val, name)) = pending_signals.pop_front() {
            if &name == "gh" && val {
                println!("hit {hit_count} at {press_count} {} since last time", press_count - last_hit);
                hit_count +=1;
                last_hit = press_count;
                if hit_count > 5 {
                    let elapsed = start.elapsed();
                    println!("{press_count} presses in {elapsed:?}");
                    return 0;
                }
            }
            if let Some(me) = lookup.get_mut(&name) {
                if let Some(value) = me.handle_pulse(&source, val) {
                    for out in &me.targets {
                        pending_signals.push_back((me.name.clone(), value, out.clone()));
                    }
                }
            }
        }
    }
    unreachable!()
}

const EG: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
const EG2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
