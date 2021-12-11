use std::str::FromStr;

use aoc_harness::*;
use utils::numset::NumSet;

aoc_main!(2021 day 8, generator lines::<Line>, part1 [p1] => 473, part2 [p2] => 1_097_568,
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

#[allow(clippy::cast_possible_truncation)]
fn char_to_num(c: char) -> u8 {
    ((c as u32) as u8) - b'a'
}
fn str_to_numset(input: &str) -> NSet {
    let mut ans = NumSet::new();
    for c in input.chars() {
        ans.insert(char_to_num(c).into());
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
        let mut ans = [0; 1 << 7];
        let one = *self.wires.iter().find(|x| x.len() == 2).unwrap();
        let four = *self.wires.iter().find(|x| x.len() == 4).unwrap();
        for &x in &self.wires {
            ans[x.inner() as usize] = match x.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 if (one.is_subset(&x)) => 3,
                5 if (four | x).len() == 7 => 2,
                5 => 5,
                6 if (!one.is_subset(&x)) => 6,
                6 if (four.is_subset(&x)) => 9,
                6 => 0,
                7 => 8,
                _ => unreachable!(),
            };
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
}
fn p1(input: &[Line]) -> usize {
    input
        .iter()
        .map(|l| {
            l.lights
                .iter()
                .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn p2(input: &[Line]) -> usize {
    input.iter().map(Line::decode).sum()
}
