use std::{collections::HashMap, str::FromStr};

use aoc_harness::*;
use utils::numset::NumSet;

aoc_main!(2021 day 8, generator lines::<Line>, part1 [p1] => 473, part2 [brute_force,intelligence] => 1097568,
  example part1 EG => 26, example part2 EG0 => 5353, example part2 EG => 61229
);
type NSet = NumSet<usize>;

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
        let wires = s
            .next()
            .unwrap()
            .split(' ')
            .map(str_to_numset)
            .collect();
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
    pub fn decode(&self) -> usize {
        let lu : HashMap<NSet,usize> = DIGIT_MAP
            .iter()
            .map(|x| (str_to_numset(x.1), x.0))
            .collect::<HashMap<_, _>>();
        //1 is the only one with 2 lit
        let one = *self.wires.iter().find(|x| x.len() == 2).unwrap();
        //4 is the only one with 4 lit
        let four = *self.wires.iter().find(|x| x.len() == 4).unwrap();
        //7 is the only one with 3 lit
        let seven = *self.wires.iter().find(|x| x.len() == 3).unwrap();
        //8 is the only one with 7 lit
        let eight = *self.wires.iter().find(|x| x.len() == 7).unwrap();
        //a is the one lit in 7 but not in 1.
        let map_a  = (seven - one).iter().exactly_one().unwrap();
        //6 is the only one with 6 lit s.t. 6 | 1 has 7 lit
        let six = *self.wires.iter().filter(|x| x.len() == 6 && (**x | one).len() == 7).exactly_one().unwrap();
        //c is the one lit in 8 but not in 6.
        let map_c = (eight - six).iter().exactly_one().unwrap();
        //5 is the only one with 5 lit s.t. 6 - 5 has 1 lit (and that lit one is e)
        let five = *self.wires.iter().filter(|&&x| x.len() == 5 && (six-x).len() == 1).exactly_one().unwrap();
        let map_e = (six - five).iter().exactly_one().unwrap();
        //0 is the only one with length 6 with e lit.
        let zero = *self.wires.iter().filter(|&&x| x.len() == 6 && x.contains(map_e as usize) && x != six).exactly_one().unwrap();
        //9 is the other len 6.
        // let nine = *self.wires.iter().filter(|&&x| x.len() == 6 && x != zero && x != six).exactly_one().unwrap();
        //2 has len 5, and e lit.
        let two = *self.wires.iter().filter(|x| x.len() == 5 && x.contains(map_e as usize)).exactly_one().unwrap();
        //3 has len 5, and isn't 2 or 5.
        let three = *self.wires.iter().filter(|&&x| x.len() == 5 && x != two && x != five).exactly_one().unwrap();
        //d is 8 - 0.
        let map_d = (eight - zero).iter().exactly_one().unwrap();
        let map_b = (five - three).iter().exactly_one().unwrap();
        let map_f = (three - two).iter().exactly_one().unwrap();
        let map_g = (five - seven - four).iter().exactly_one().unwrap();
        let mut ans = [0; 7];
        ans[map_a] = 0_u8;
        ans[map_b] = 1;
        ans[map_c] = 2;
        ans[map_d] = 3;
        ans[map_e] = 4;
        ans[map_f] = 5;
        ans[map_g] = 6;

        let out = self
            .lights
            .iter()
            .map(|&x| convert(x, &ans))
            .map(|x| lu[&x])
            .collect_vec();
        out[0] * 1000 + out[1] * 100 + out[2] * 10 + out[3]
    }
    pub fn find(&self) -> usize {
        let lu : HashMap<NSet,usize> = DIGIT_MAP
            .iter()
            .map(|x| (str_to_numset(x.1), x.0))
            .collect::<HashMap<_, _>>();
        let ans = "abcdefg"
            .chars()
            .permutations(7)
            .map(|x| x.into_iter().map(char_to_num).collect_vec())
            .find(|p| {
                [&self.wires, &self.lights].iter().all(|i| {
                    i.iter().all(|w| {
                        let co = convert(*w, p);
                        lu.contains_key(&co)
                    })
                })
            })
            .unwrap();
        let out = self
            .lights
            .iter()
            .map(|&x| convert(x, &ans))
            .map(|x| lu[&x])
            .collect_vec();
        out[0] * 1000 + out[1] * 100 + out[2] * 10 + out[3]
    }
}
fn convert(input: NSet, mapping: &[u8]) -> NSet {
    // dbg!(input, mapping);
    input
        .iter()
        .map(|x| mapping[x as usize])
        .collect()
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
