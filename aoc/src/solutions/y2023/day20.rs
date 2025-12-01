use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap, VecDeque},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::{all_consuming, success, value},
    multi::separated_list1,
    sequence::terminated,
    Parser,
};
use utils::{collections::VecLookup, nom::IResult};

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
    Conjunction(VecLookup<bool>),
}
impl ModuleType {
    fn parse(input: &str) -> IResult<Self> {
        alt((
            value(ModuleType::FlipFlop(false), tag("%")),
            value(ModuleType::Conjunction(VecLookup::default()), tag("&")),
            success(ModuleType::Broadcast),
        ))
        .parse(input)
    }
}
thread_local! {
    static CONVERT : RefCell<(usize,HashMap<String,usize>)> = RefCell::new((0,HashMap::new()));
}

fn convert(a: &str) -> usize {
    CONVERT.with_borrow_mut(|(next, map)| match map.entry(a.to_string()) {
        Entry::Occupied(x) => *x.get(),
        Entry::Vacant(v) => {
            let a = *next;
            v.insert(a);
            *next += 1;
            a
        }
    })
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Module {
    name: usize,
    typ: ModuleType,
    targets: Vec<usize>,
}
impl Module {
    fn parse(input: &str) -> IResult<Self> {
        let (input, (typ, name, targets)) = (
            ModuleType::parse,
            terminated(alpha1, tag(" -> ")),
            separated_list1(tag(", "), alpha1),
        )
            .parse(input)?;
        let name = convert(name);
        // if name == 0 {
        //     dbg!(input, &targets);
        // }
        let targets = targets.into_iter().map(convert).collect();
        // if name == 0 {
        //     dbg!(&targets);
        // }
        Ok((input, Self { name, typ, targets }))
    }
}
impl Module {
    fn add_input(&mut self, name: usize) {
        if let ModuleType::Conjunction(inputs) = &mut self.typ {
            inputs.insert(name, false);
        }
    }
    fn handle_pulse(&mut self, source: Option<usize>, value: bool) -> Option<bool> {
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
    modules: VecLookup<Module>,
    input_maps: HashMap<usize, Vec<usize>>,
    pending_signals: VecDeque<(Option<usize>, bool, usize)>,
    signal_counts: [usize; 2],
}
impl System {
    fn from_str(input: &str) -> Self {
        CONVERT.with_borrow_mut(|x| {
            x.0 = 0;
            x.1.clear();
        });
        let (_, modules) =
            all_consuming(terminated(separated_list1(newline, Module::parse), newline))
                .parse(input)
                .expect("parse");
        let mut modules: VecLookup<Module> = modules.into_iter().map(|x| (x.name, x)).collect();
        // modules.sort_by_key(|x| x.name);
        let mut input_maps: HashMap<usize, Vec<usize>> = HashMap::new();
        for (_, m) in modules.iter() {
            for t in &m.targets {
                input_maps.entry(*t).or_default().push(m.name);
            }
        }
        for (target, sources) in &input_maps {
            if let Some(t) = modules.get_mut(*target) {
                for s in sources {
                    t.add_input(*s);
                }
            }
        }
        // CONVERT.with_borrow(|x| {
        //     dbg!(x);
        // });
        Self {
            modules,
            input_maps,
            ..Default::default()
        }
    }
    fn inject_signal(&mut self, target: usize, value: bool) {
        if self.modules.get(target).is_none() {
            // CONVERT.with_borrow(|x| {
            //     dbg!(x);
            // });
            // dbg!(target, &self.modules[0], self.modules.len());
            for (ix, m) in self.modules.iter().enumerate() {
                assert_eq!(ix, m.1.name);
            }
        }
        let t = self.modules.get_mut(target).expect("valid module");
        assert!(
            !matches!(t.typ, ModuleType::Conjunction(_)),
            "Cannot inject to Conjunction modules"
        );
        self.pending_signals.push_back((None, value, target));
    }
    fn quiet(&self) -> bool {
        self.pending_signals.is_empty()
    }
    fn next_signal_is(&self, target: usize, value: bool) -> bool {
        self.pending_signals
            .front()
            .map(|s| s.1 == value && s.2 == target)
            .unwrap_or_default()
    }
    fn next_signal(&mut self) -> Option<(Option<usize>, bool, usize)> {
        self.pending_signals.pop_front()
    }
    fn step(&mut self) {
        let (source, value, target) = self.next_signal().unwrap();
        self.signal_counts[usize::from(value)] += 1;
        if let Some(me) = self.modules.get_mut(target) {
            if let Some(value) = me.handle_pulse(source, value) {
                for out in &me.targets {
                    self.pending_signals.push_back((Some(me.name), value, *out));
                }
            }
        }
    }
}

fn p1(system: &System) -> usize {
    let mut sys = system.clone();
    let start = convert("broadcaster");
    for _ in 0..1000 {
        sys.inject_signal(start, false);
        while !sys.quiet() {
            sys.step();
        }
    }
    sys.signal_counts.iter().copied().product()
}

fn p2(system: &System) -> usize {
    let start = convert("broadcaster");
    let end = convert("rx");
    let broadcast_outputs = &system.modules[start].targets;
    let rx_input = system.input_maps[&end][0];
    broadcast_outputs
        .iter()
        .map(|x| {
            assert!(matches!(system.modules[*x].typ, ModuleType::FlipFlop(_)));
            let mut sys = system.clone();
            (1..)
                .find(|_| {
                    sys.inject_signal(*x, false);
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
