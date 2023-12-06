use itertools::chain;
use rayon::prelude::*;
use std::{fmt::Display, ops::Add, str::FromStr};

use aoc_harness::*;

aoc_harness::aoc_main!(2021 day 18, generator lines::<Snail>, part1 [p1] => 4641, part2 [p2,p2_parallel] => 4624, example part1 EG => 4140, example part2 EG => 3993);

const EG: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

#[derive(Debug, Clone)]
enum Snail {
    Leaf(usize),
    Pair(Box<(Snail, Snail)>),
}
fn parse_snail(s: &str) -> (Snail, &str) {
    match s.chars().next().unwrap() {
        '[' => {
            let (a, s) = parse_snail(&s[1..]);
            assert_eq!(s.as_bytes().first(), Some(&b','));
            let (b, s) = parse_snail(&s[1..]);
            assert_eq!(s.as_bytes().first(), Some(&b']'));
            let ans = Snail::new_pair(a, b);
            (ans, &s[1..])
        }
        c => (
            Snail::Leaf(usize::try_from(c.to_digit(16).unwrap()).unwrap()),
            &s[1..],
        ),
    }
}

impl FromStr for Snail {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = parse_snail(s);
        assert_eq!(b, "");
        Ok(a)
    }
}
impl Add for Snail {
    type Output = Snail;
    fn add(self, rhs: Self) -> Self::Output {
        let mut a = Self::new_pair(self, rhs);
        a.reduce();
        a
    }
}
impl Display for Snail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Leaf(n) => f.write_fmt(format_args!("{n}"))?,
            Self::Pair(c) => f.write_fmt(format_args!("[{},{}]", c.0, c.1))?,
        }
        Ok(())
    }
}
impl Snail {
    fn new_leaf(n: usize) -> Self {
        Self::Leaf(n)
    }
    fn new_pair(a: Self, b: Self) -> Self {
        Self::Pair(Box::new((a, b)))
    }
    fn try_split(&mut self) -> bool {
        match self {
            &mut Self::Leaf(x) if x >= 10 => {
                *self = Self::new_pair(Self::new_leaf(x / 2), Self::new_leaf(x / 2 + x % 2));
                true
            }
            Self::Leaf(_) => false,
            Self::Pair(c) => c.0.try_split() || c.1.try_split(),
        }
    }
    fn try_explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        if depth == 0 {
            //explode here. we assert that the values in c are leaf nodes.
            if let Self::Pair(c) = self {
                let l = c.0.get_value().unwrap();
                let r = c.1.get_value().unwrap();
                *self = Self::Leaf(0);
                return Some((l, r));
            }
            return None;
        }
        match self {
            Self::Leaf(_) => None,
            Self::Pair(c) => {
                let mut exploded = false;
                let mut left_bits = 0;
                let mut right_bits = 0;
                //try recursing down each path
                if let Some((l, r)) = c.0.try_explode(depth - 1) {
                    exploded = true;
                    //left child exploded, we can push the right bit now.
                    if r > 0 {
                        c.1.add_to_left_edge(r);
                    }
                    left_bits = l;
                }
                if let Some((l, r)) = c.1.try_explode(depth - 1) {
                    exploded = true;
                    //right child exploded, we can push the left bit now.
                    if l > 0 {
                        c.0.add_to_right_edge(l);
                    }
                    right_bits = r;
                }
                if exploded {
                    Some((left_bits, right_bits))
                } else {
                    None
                }
            }
        }
    }
    fn add_to_right_edge(&mut self, val: usize) {
        match self {
            Self::Leaf(n) => {
                *n += val;
            }
            Self::Pair(c) => {
                c.1.add_to_right_edge(val);
            }
        }
    }
    fn add_to_left_edge(&mut self, val: usize) {
        match self {
            Self::Leaf(n) => {
                *n += val;
            }
            Self::Pair(c) => {
                c.0.add_to_left_edge(val);
            }
        }
    }
    fn reduce(&mut self) {
        while {
            self.try_explode(4);
            //assert that it's now unexplodable.
            self.try_split()
        } {}
    }

    fn get_value(&self) -> Option<usize> {
        match &self {
            Self::Leaf(l) => Some(*l),
            Self::Pair(_) => None,
        }
    }
    fn magnitude(&self) -> usize {
        match &self {
            Self::Leaf(a) => *a,
            Self::Pair(c) => 3 * c.0.magnitude() + 2 * c.1.magnitude(),
        }
    }
}
fn p1(input: &[Snail]) -> usize {
    input
        .iter()
        .cloned()
        .reduce(|item, accum| item + accum)
        .unwrap()
        .magnitude()
}
fn p2_parallel(input: &[Snail]) -> usize {
    let l = input.len();
    (0..l)
        .into_par_iter()
        .map(|x| {
            chain!(0..x, x + 1..l)
                .map(|y| (input[x].clone() + input[y].clone()).magnitude())
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}
fn p2(input: &[Snail]) -> usize {
    input
        .iter()
        .permutations(2)
        .map(|v| (v[0].clone() + v[1].clone()).magnitude())
        .max()
        .unwrap()
}
#[cfg(test)]
mod snailtest {

    use super::*;

    fn exp(input: &str) -> String {
        let mut p: Snail = input.parse().unwrap();
        p.try_explode(4);
        format!("{p}")
    }
    fn spl(input: &str) -> String {
        let mut p: Snail = input.parse().unwrap();
        p.try_split();
        format!("{p}")
    }
    fn add(a: &str, b: &str) -> String {
        let a: Snail = a.parse().unwrap();
        let b: Snail = b.parse().unwrap();
        let c = a + b;
        format!("{c}")
    }
    #[test]
    fn test_explodes() {
        // assert_eq!(
        //     exp("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
        //     "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
        // );
        assert_eq!(exp("[[6,[5,[4,[3,2]]]],1]"), "[[6,[5,[7,0]]],3]");
        assert_eq!(exp("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");
        assert_eq!(exp("[7,[6,[5,[4,[3,2]]]]]"), "[7,[6,[5,[7,0]]]]");
        assert_eq!(
            exp("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        );
    }
    #[test]
    fn test_split() {
        assert_eq!(
            spl("[[[[0,7],4],[F,[0,D]]],[1,1]]"),
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
        );
    }
    const ADDITION_SCRIPT: &str = "
  [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
+ [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
= [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]

  [[[[4,3],4],4],[7,[[8,4],9]]]
+ [1,1]
= [[[[0,7],4],[[7,8],[6,0]]],[8,1]]

  [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
+ [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
= [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]

  [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
+ [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
= [[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]

  [[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
+ [7,[5,[[3,8],[1,4]]]]
= [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]

  [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]
+ [[2,[2,2]],[8,[8,1]]]
= [[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]

  [[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]
+ [2,9]
= [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]

  [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
+ [1,[[[9,3],9],[[9,0],[0,7]]]]
= [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]

  [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]
+ [[[5,[7,4]],7],1]
= [[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]

  [[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]
+ [[[[4,2],2],6],[8,7]]
= [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

    #[test]
    fn test_addition() {
        for sum in ADDITION_SCRIPT.trim_matches('\n').split("\n\n") {
            let mut i = sum.lines();
            let a = &i.next().unwrap()[2..];
            let b = &i.next().unwrap()[2..];
            let c = &i.next().unwrap()[2..];
            assert_eq!(add(a, b), c);
        }
    }
}

//TODO: study https://github.com/erikaderstedt/aoc-2020/blob/9248a45c5ca0b6bfe8887c6cabf54f0e074511b5/src/days_2021/day18.rs
