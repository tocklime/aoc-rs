use ndarray::Array2;
use std::str::FromStr;

use aoc_harness::*;
use utils::{aabb::Aabb, cartesian::Point};

aoc_main!(2021 day 5, generator lines::<X>,
          part1 [solve::<false>] => 5147, part2 [solve::<true>] => 16925,
          example both EG => (5,12));

#[derive(Debug)]
struct X {
    from: Point<usize>,
    to: Point<usize>,
}
impl X {
    fn is_orthogonal(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }
}
impl FromStr for X {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s1 = s.split(" -> ").map(str::parse);
        Ok(Self {
            from: s1.next().unwrap()?,
            to: s1.exactly_one().unwrap()?,
        })
    }
}

fn solve<const EVEN_DIAGONALS: bool>(input: &[X]) -> usize {
    let bb: Aabb<usize> = input.iter().flat_map(|x| [&x.from, &x.to]).collect();
    let mut grid = Array2::from_elem((bb.top_right.x + 1, bb.top_right.y + 1), 0_u8);
    let ps = input
        .iter()
        .filter(|x| EVEN_DIAGONALS || x.is_orthogonal())
        .flat_map(|x| x.from.steps_to(x.to, true));
    let mut ans = 0;
    for p in ps {
        let c = &mut grid[(p.x, p.y)];
        match *c {
            0 => *c = 1,
            1 => {
                ans += 1;
                *c = 2;
            }
            _ => {}
        }
    }
    ans
}

const EG: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
