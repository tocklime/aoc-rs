use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc_harness::*;

aoc_main!(2021 day 8, generator lines::<Line>, part1 [p1], part2 [p2],
  example part1 EG => 26, example part2 EG0 => 5353, example part2 EG => 61229
);

const EG0: &str =
    "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
const EG: &str = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

#[derive(Debug)]
struct Digit {
    on: String,
}
impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let on = s.chars().sorted().collect();
        Ok(Self { on })
    }
}

#[derive(Debug)]
struct Line {
    wires: Vec<Digit>,
    lights: Vec<Digit>,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" | ");
        let wires = input::<Digit, ' '>(s.next().unwrap());
        let lights = input::<Digit, ' '>(s.exactly_one().unwrap());
        Ok(Self { wires, lights })
    }
}
impl Line {
    pub fn decode(&self) -> usize {
        let LIGHT_MAP = vec![
            ('a', "0235789"),
            ('b', "045689"),
            ('c', "01234789"),
            ('d', "2345689"),
            ('e', "0268"),
            ('f', "013456789"),
            ('g', "0235689"),
        ];
        let lu = DIGIT_MAP
            .iter()
            .map(|x| (x.1, x.0))
            .collect::<HashMap<_, _>>();
        let mut poss: HashMap<char, Vec<char>> = HashMap::new();
        let one = self.wires.iter().find(|x| x.on.len() == 2).unwrap();
        poss.entry('c')
            .or_insert_with(Vec::new)
            .extend(one.on.chars());
        poss.entry('f')
            .or_insert_with(Vec::new)
            .extend(one.on.chars());
        // let four = self.wires.iter().find(|x| x.on.len() == )

        dbg!(poss);
        0
    }
    pub fn find(&self) -> usize {
        let lu = DIGIT_MAP
            .iter()
            .map(|x| (x.1.to_owned(), x.0))
            .collect::<HashMap<_, _>>();
        let ans = "abcdefg"
            .chars()
            .permutations(7)
            .find(|p| {
                let conv = |c: &str| {
                    c.chars()
                        .map(|x| p[(x as u32 - 'a' as u32) as usize])
                        .sorted()
                        .collect::<String>()
                };
                self.wires.iter().all(|w| {
                    let co = conv(&w.on);
                    lu.contains_key(&co)
                })
                // && self.lights.iter().all(|w| lu.contains_key(&conv(&w.on)))
            })
            .unwrap();
        let out = self.lights.iter().map(|x| x.on.chars()
            .map(|x| ans[(x as u32 - 'a' as u32) as usize])
            .sorted().collect::<String>())
            .map(|x| lu[&x])
            .collect_vec();
        out[0] * 1000 + out[1] * 100 + out[2] * 10 + out[3]
    }
}

const DIGIT_MAP: [(usize, &str); 10] = [
    (0, "abcefg"),
    (1, "cf"),
    (2, "acdeg"),
    (3, "acdfg"),
    (4, "bcdf"),
    (5, "abdfg"),
    (6, "abdefg"),
    (7, "acf"),
    (8, "abcdefg"),
    (9, "abcdfg"),
];
fn p1(input: &[Line]) -> usize {
    input
        .iter()
        .map(|l| {
            l.lights
                .iter()
                .filter(|x| [2, 4, 3, 7].contains(&x.on.len()))
                .count()
        })
        .sum()
}

fn p2(input: &[Line]) -> usize {
    input.iter().map(|l| l.find()).sum()
}
