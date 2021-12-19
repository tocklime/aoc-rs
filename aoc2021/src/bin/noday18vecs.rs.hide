use std::{str::FromStr, ops::{Add, AddAssign}};

use aoc_harness::*;

aoc_main!(2021 day 18, generator lines::<Snail>, part1 [p1] => 4641, part2 [] => 4624, example part1 EG => 4140, example part2 EG => 3993);

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

#[derive(Debug,Clone)]
struct Snail {
    values: Vec<usize>,
    depths: Vec<isize>,
}

impl FromStr for Snail {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut depth = 0;
        let mut values = Vec::new();
        let mut depths = Vec::new();
        for c in s.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {}
                c => {
                    values.push(usize::try_from(c.to_digit(10).unwrap()).unwrap());
                    depths.push(depth);
                }
            }
        }
        Ok(Self {
            values,
            depths,
        })
    }
}
impl AddAssign for Snail {
    fn add_assign(&mut self, rhs: Self) {
        self.values.extend(rhs.values);
        self.depths.extend(rhs.depths);
    }
}
impl Snail {
    fn explode(&self) -> Self {
        let mut values = Vec::with_capacity(self.values.len());
        let mut depths = Vec::with_capacity(self.values.len());
        depths.push(self.depths[0]);
        values.push(self.values[0]);
        Self {depths, values}
    }
}

fn p1(input: &[Snail]) -> usize {
    let i = input.iter().cloned().collect_vec();
    0
}
