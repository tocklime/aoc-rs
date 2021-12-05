use std::{collections::HashMap, str::FromStr};

use aoc_harness::*;
use utils::{cartesian::Point, aabb::Aabb};

aoc_main!(2021 day 5, generator lines::<X>, [solve::<false>] => 5147, [solve::<true>] => 16925);

#[derive(Debug)]
struct X {
    from: Point<isize>,
    to: Point<isize>,
}
impl FromStr for X {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s1 = s.split(" -> ").collect_vec();
        let from_t = s1[0]
            .split(',')
            .map(|x| x.parse::<isize>().unwrap())
            .collect_vec();
        let to_t = s1[1]
            .split(',')
            .map(|x| x.parse::<isize>().unwrap())
            .collect_vec();
        let from = Point::new(from_t[0], from_t[1]);
        let to = Point::new(to_t[0], to_t[1]);
        Ok(Self { from, to })
    }
}

fn solve<const EVEN_DIAGONALS: bool>(input: &[X]) -> usize {
    let mut grid: HashMap<Point<isize>, usize> = HashMap::new();
    // let bb : Aabb<isize> = input.iter().flat_map(|x| [&x.from, &x.to]).collect();
    // dbg!(bb);
    input.iter().fold(0, |mut ans, i| {
        if EVEN_DIAGONALS || i.from.x == i.to.x || i.from.y == i.to.y {
            for p in i.from.steps_to(i.to, true) {
                let e = grid.entry(p).or_default();
                if *e == 1 {
                    ans += 1;
                }
                *e += 1;
            }
        }
        ans
    })
}

#[cfg(test)]
mod tests {
    use super::*;
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
    #[test]
    fn t1() {
        assert_eq!(solve::<false>(&dbg!(lines::<X>(EG))), 5)
    }
    #[test]
    fn t2() {
        assert_eq!(solve::<true>(&dbg!(lines::<X>(EG))), 12)
    }
}
