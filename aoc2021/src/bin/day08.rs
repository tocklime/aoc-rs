use std::{collections::HashMap, str::FromStr};

use aoc_harness::*;
use utils::numset::NumSet;

aoc_main!(2021 day 8, generator lines::<Line>, part1 [p1] => 473, part2 [brute_force,intelligence] => 1097568,
  example part1 EG => 26, example part2 EG0 => 5353, example part2 EG => 61229
);
type NSet = NumSet<u8>;

const EG0: &str = "
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
";
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
struct Line {
    wires: Vec<NSet>,
    lights: Vec<NSet>,
}

fn char_to_num(c: char) -> u8 {
    (c as u32 - 'a' as u32) as u8
}
fn str_to_numset(input: &str) -> NSet {
    let mut ans = NumSet::new();
    for c in input.chars() {
        ans.insert(char_to_num(c).into())
    }
    ans
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" | ");
        let wires = s.next().unwrap().split(' ').map(str_to_numset).collect();
        let lights = s
            .exactly_one()
            .unwrap()
            .split(' ')
            .map(str_to_numset)
            .collect();
        Ok(Self { wires, lights })
    }
}
impl Line {
    pub fn deduce_map(&self) -> [usize; 1 << 7] {
        let mut remaining = self.wires.clone();
        let mut ns = [NSet::from(0); 10];
        remaining.retain(|&x| match x.len() {
            2 => {
                ns[1] = x;
                false
            }
            4 => {
                ns[4] = x;
                false
            }
            _ => true,
        });
        for x in remaining {
            let n = match x.len() {
                3 => 7,
                7 => 8,
                5 if (ns[1].is_subset(&x)) => 3,
                5 if (ns[4] | x).len() == 7 => 2,
                5 => 5,
                6 if (!ns[1].is_subset(&x)) => 6,
                6 if (ns[4].is_subset(&x)) => 9,
                6 => 0,
                _ => unreachable!(),
            };
            ns[n] = x;
        }
        let mut ans = [0; 1 << 7];
        for (n, &s) in ns.iter().enumerate() {
            ans[s.inner() as usize] = n;
        }
        ans
    }
    pub fn decode(&self) -> usize {
        let m = self.deduce_map();

        self.lights
            .iter()
            .map(|&x| m[x.inner() as usize])
            .fold(0, |acc, x| acc * 10 + x)
    }
    pub fn find(&self) -> usize {
        let lu: HashMap<NSet, usize> = DIGS
            .iter()
            .enumerate()
            .map(|(d, n)| (NumSet::from(*n), d))
            .collect::<HashMap<_, _>>();
        let ans = "abcdefg"
            .chars()
            .permutations(7)
            .map(|x| x.into_iter().map(char_to_num).collect_vec())
            .find(|p| {
                self.wires.iter().all(|w| {
                    let co = convert(*w, p);
                    lu.contains_key(&co)
                })
            })
            .unwrap();
        self.lights
            .iter()
            .map(|&x| convert(x, &ans))
            .map(|x| lu[&x])
            .fold(0, |acc, x| acc * 10 + x)
    }
}
fn convert(input: NSet, mapping: &[u8]) -> NSet {
    // dbg!(input, mapping);
    input.iter().map(|x| mapping[x as usize]).collect()
}

const DIGS: [u8; 10] = [
    0b01110111, 0b00100100, 0b01011101, 0b01101101, 0b00101110, 0b01101011, 0b01111011, 0b00100101,
    0b01111111, 0b01101111,
];
fn p1(input: &[Line]) -> usize {
    input
        .iter()
        .map(|l| {
            l.lights
                .iter()
                .filter(|x| [2, 4, 3, 7].contains(&x.len()))
                .count()
        })
        .sum()
}

fn brute_force(input: &[Line]) -> usize {
    input.iter().map(|l| l.find()).sum()
}
fn intelligence(input: &[Line]) -> usize {
    input.iter().map(|l| l.decode()).sum()
}
