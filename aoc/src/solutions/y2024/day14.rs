use itertools::Itertools;
use utils::{
    cartesian::Point, collections::VecLookup, grid2d::Grid2d, nums::chinese_remainder_theorem,
};

aoc_harness::aoc_main!(2024 day 14, part1 [p1] => 230_172_768, part2 [p2] => 8087, example part1 EG => 12);

#[derive(Debug)]
struct BunnyRobot {
    pos: Point<isize>,
    vel: Point<isize>,
    max: Point<isize>,
}

impl BunnyRobot {
    fn loc_after_n(&self, n: isize) -> Point<isize> {
        let x = (self.pos.x + n * self.vel.x).rem_euclid(self.max.x);
        let y = (self.pos.y + n * self.vel.y).rem_euclid(self.max.y);
        Point::new(x, y)
    }
    fn quadrant_after_n(&self, n: isize) -> Option<Point<usize>> {
        let final_pos = self.loc_after_n(n);
        let x_quad = match final_pos.x.cmp(&((self.max.x - 1) / 2)) {
            std::cmp::Ordering::Less => Some(0),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(1),
        }?;
        let y_quad = match final_pos.y.cmp(&((self.max.y - 1) / 2)) {
            std::cmp::Ordering::Less => Some(0),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(1),
        }?;
        Some(Point::new(x_quad, y_quad))
    }
}

fn gen(input: &str, max: Point<isize>) -> Vec<BunnyRobot> {
    utils::inputs::ThingParser::new(input, nom::character::complete::i32)
        .chunks(4)
        .into_iter()
        .map(|mut i| BunnyRobot {
            pos: Point::new(i.next().unwrap() as isize, i.next().unwrap() as isize),
            vel: Point::new(i.next().unwrap() as isize, i.next().unwrap() as isize),
            max,
        })
        .collect()
}

fn p1(input: &str) -> usize {
    let max = if input == EG {
        Point::new(11, 7)
    } else {
        Point::new(101, 103)
    };
    let bs = gen(input, max);
    let mut quads = Grid2d::from_elem((2, 2), 0);
    for b in &bs {
        if let Some(q) = b.quadrant_after_n(100) {
            quads[q] += 1;
        }
    }
    quads.iter().product()
}

fn p2(input: &str) -> isize {
    let max = Point::new(101, 103);
    let bs = gen(input, max);
    //for each axis, find the index in the cycle that has the fewest distinct values represented.
    let best_x = (0..max.x)
        .map(|t| {
            let mut vl = VecLookup::with_capacity(max.x as usize);
            let mut count = 0;
            for b in &bs {
                if vl.insert(b.loc_after_n(t).x as usize, true) {
                    count += 1;
                }
            }
            (count, t)
        })
        .min()
        .unwrap();
    let best_y = (0..max.y)
        .map(|t| {
            let mut vl = VecLookup::with_capacity(max.y as usize);
            let mut count = 0;
            for b in &bs {
                if vl.insert(b.loc_after_n(t).y as usize, true) {
                    count += 1;
                }
            }
            (count, t)
        })
        .min()
        .unwrap();
    //find the time when both x and y axis have the fewest distinct values represented using CRT.
    chinese_remainder_theorem(&[(best_x.1, max.x), (best_y.1, max.y)])
}

const EG: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
