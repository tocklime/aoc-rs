use std::{
    collections::{HashMap, VecDeque},
    future::pending,
};

use clap::builder::StyledStr;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::{success, value},
    multi::separated_list1,
    sequence::{preceded, tuple},
    Parser,
};
use nom_supreme::ParserExt;
use utils::{collections::VecLookup, iter::borrow_mut_twice, nom::IResult};

aoc_harness::aoc_main!(2023 day 20,
    generator System::from_str,
    part1 [p1] => 730_797_576,
    part2 [p2] => 226_732_077_152_351,
    example part1 EG => 32_000_000,
    example part1 EG2 => 11_687_500);

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
#[derive(Default, Clone, Debug)]
struct System {
    modules: HashMap<String, Module>,
    input_maps: HashMap<String, Vec<String>>,
    pending_signals: VecDeque<(String, bool, String)>,
    signal_counts: [usize; 2],
}
impl System {
    fn from_str(input: &str) -> Self {
        let (_, modules) = separated_list1(newline, Module::parse)
            .terminated(newline)
            .all_consuming()
            .complete()
            .parse(input)
            .expect("parse");
        let mut modules: HashMap<String, Module> =
            modules.into_iter().map(|x| (x.name.clone(), x)).collect();
        let mut input_maps: HashMap<String, Vec<String>> = HashMap::new();
        for m in modules.values() {
            for t in &m.targets {
                input_maps
                    .entry(t.to_owned())
                    .or_default()
                    .push(m.name.to_owned());
            }
        }
        for (target, sources) in &input_maps {
            if let Some(t) = modules.get_mut(target) {
                for s in sources {
                    t.add_input(s);
                }
            }
        }
        Self {
            modules,
            input_maps,
            ..Default::default()
        }
    }
    fn inject_signal(&mut self, target: &str, value: bool) {
        let t = self.modules.get_mut(target).expect("valid module");
        assert!(
            !matches!(t.typ, ModuleType::Conjunction(_)),
            "Cannot inject to Conjunction modules"
        );
        self.pending_signals
            .push_back(("injected".to_string(), value, target.to_owned()));
    }
    fn quiet(&self) -> bool {
        self.pending_signals.is_empty()
    }
    fn next_signal_is(&self, target: &str, value: bool) -> bool {
        self.pending_signals
            .front()
            .map(|s| s.1 == value && s.2 == target)
            .unwrap_or_default()
    }
    fn step(&mut self) {
        let (source, value, target) = self.pending_signals.pop_front().expect("pending signal");
        self.signal_counts[usize::from(value)] += 1;
        if let Some(me) = self.modules.get_mut(&target) {
            if let Some(value) = me.handle_pulse(&source, value) {
                for out in &me.targets {
                    self.pending_signals
                        .push_back((me.name.clone(), value, out.clone()));
                }
            }
        }
    }
}
fn p1(system: &System) -> usize {
    let mut sys = system.clone();
    for _ in 0..1000 {
        sys.inject_signal("broadcaster", false);
        while !sys.quiet() {
            sys.step();
        }
    }
    sys.signal_counts.iter().copied().product()
}
fn p2(system: &System) -> usize {
    let broadcast_outputs = &system.modules["broadcaster"].targets;
    let rx_input = &system.input_maps["rx"][0];
    broadcast_outputs
        .iter()
        .map(|x| {
            assert!(matches!(system.modules[x].typ, ModuleType::FlipFlop(_)));
            let mut sys = system.clone();
            (1..)
                .find(|_| {
                    sys.inject_signal(x, false);
                    while !sys.quiet() {
                        sys.step();
                        if sys.next_signal_is(rx_input, true) {
                            return true;
                        }
                    }
                    false
                })
                .unwrap()
        })
        .product()
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
