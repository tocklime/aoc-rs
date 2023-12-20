use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::{success, value},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};
use nom_supreme::ParserExt;
use utils::nom::IResult;

aoc_harness::aoc_main!(2023 day 20,
    generator System::from_str,
    part1 [p1] => 730_797_576,
    part2 [p2] => 226_732_077_152_351,
    example part1 EG => 32_000_000,
    example part1 EG2 => 11_687_500);

#[derive(Clone, PartialEq, Eq, Debug)]
enum ModuleType<ID: PartialEq + Eq + std::hash::Hash> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<ID, bool>),
}
impl<'a> ModuleType<&'a str> {
    fn parse(input: &str) -> IResult<Self> {
        alt((
            value(ModuleType::FlipFlop(false), tag("%")),
            value(ModuleType::Conjunction(HashMap::new()), tag("&")),
            success(ModuleType::Broadcast),
        ))(input)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Module<ID : PartialEq + Eq + std::hash::Hash> {
    name: ID,
    typ: ModuleType<ID>,
    targets: Vec<ID>,
}
impl<'a> Module<&'a str> {
    fn parse(input: &'a str) -> IResult<Self> {
        let (input, (typ, name, targets)) = tuple((
            ModuleType::parse,
            alpha1.terminated(tag(" -> ")),
            separated_list1(tag(", "), alpha1),
        ))(input)?;
        Ok((input, Self { name, typ, targets }))
    }
}
impl<ID : PartialEq + Eq + std::hash::Hash + std::fmt::Display> Module<ID> {
    fn add_input(&mut self, name: ID) {
        if let ModuleType::Conjunction(inputs) = &mut self.typ {
            inputs.insert(name, false);
        }
    }
    fn handle_pulse(&mut self, source: Option<ID>, value: bool) -> Option<bool> {
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
                let source = source.expect("No injecting to conjunctions");
                if let Some(x) = inps.get_mut(&source) {
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
struct System<ID : PartialEq + Eq + std::hash::Hash> {
    modules: HashMap<ID, Module<ID>>,
    input_maps: HashMap<ID, Vec<ID>>,
    pending_signals: VecDeque<(Option<ID>, bool, ID)>,
    signal_counts: [usize; 2],
}
impl<'a> System<&'a str> {
    fn from_str(input: &'a str) -> Self {
        let (_, modules) = separated_list1(newline, Module::parse)
            .terminated(newline)
            .all_consuming()
            .complete()
            .parse(input)
            .expect("parse");
        let mut modules: HashMap<&str, Module<&'a str>> = modules.into_iter().map(|x| (x.name, x)).collect();
        let mut input_maps: HashMap<&str, Vec<&str>> = HashMap::new();
        for m in modules.values() {
            for t in &m.targets {
                input_maps.entry(t).or_default().push(m.name);
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
}
impl<ID : PartialEq + Eq + std::hash::Hash+ Clone + std::fmt::Display> System<ID> {
    fn inject_signal(&mut self, target: ID, value: bool) {
        let t = self.modules.get_mut(&target).expect("valid module");
        assert!(
            !matches!(t.typ, ModuleType::Conjunction(_)),
            "Cannot inject to Conjunction modules"
        );
        self.pending_signals.push_back((None, value, target));
    }
    fn quiet(&self) -> bool {
        self.pending_signals.is_empty()
    }
    fn next_signal_is(&self, target: ID, value: bool) -> bool {
        self.pending_signals
            .front()
            .map(|s| s.1 == value && s.2 == target)
            .unwrap_or_default()
    }
    fn step(&mut self) {
        let (source, value, target) = self.pending_signals.pop_front().expect("pending signal");
        self.signal_counts[usize::from(value)] += 1;
        if let Some(me) = self.modules.get_mut(&target) {
            if let Some(value) = me.handle_pulse(source, value) {
                for out in &me.targets {
                    self.pending_signals
                        .push_back((Some(me.name.clone()), value, out.clone()));
                }
            }
        }
    }
}
fn p1(system: &System<&str>) -> usize {
    let mut sys = system.clone();
    for _ in 0..1000 {
        sys.inject_signal("broadcaster", false);
        while !sys.quiet() {
            sys.step();
        }
    }
    sys.signal_counts.iter().copied().product()
}
fn p2(system: &System<&str>) -> usize {
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
