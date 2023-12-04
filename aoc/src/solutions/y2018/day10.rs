aoc_harness::aoc_main!(2018 day 10, generator gen, both [p]);
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};
use utils::{aabb::Aabb, cartesian::render_char_map_w, cartesian::Point};

#[derive(PartialEq, Debug, Clone, Copy)]
//#[from_str(regex = r"position=<\s*(?P<loc.x>[-0-9]+),\s*(?P<loc.y>[-0-9]+)} velocity=<\s*(?P<vel.x>[-0-9]+),\s*(?P<vel.y>[-0-9]+)")]
struct Star {
    loc: Point<i32>,
    vel: Point<i32>,
}

impl FromStr for Star {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"position=<\s*([-0-9]+),\s*([-0-9]+)>\s+velocity=<\s*([-0-9]+),\s*([-0-9]+)>",
        )
        .map_err(|x| format!("{x}"))?;
        let caps = re.captures(s).ok_or_else(|| "No match".to_string())?;
        let nums: Vec<i32> = (1..5)
            .map(|n| caps[n].parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(Self {
            loc: Point::new(nums[0], nums[1]),
            vel: Point::new(nums[2], nums[3]),
        })
    }
}

impl Star {
    fn step(&self) -> Self {
        Self {
            loc: self.loc + self.vel,
            vel: self.vel,
        }
    }
    fn field_bb(l: &[Self]) -> Aabb<i32> {
        l.iter().map(|x| &x.loc).collect()
    }
}

fn gen(input: &str) -> Vec<Star> {
    input
        .trim()
        .lines()
        .map(|l| {
            // position=<-31503, -52596> velocity=< 3,  5>
            match l.parse() {
                Ok(p) => p,
                Err(e) => panic!("can't parse line: {l}: {e}"),
            }
        })
        .collect()
}
fn p(input: &[Star]) -> (String, usize) {
    let mut stars = input.iter().copied().collect_vec();
    let mut last_size = Star::field_bb(&stars).area();
    for t in 0.. {
        let new_stars = stars.iter().map(Star::step).collect_vec();
        let size = Star::field_bb(&new_stars).area();
        if size > last_size {
            let grid: HashMap<Point<i32>, char> = stars.iter().map(|x| (x.loc, 'X')).collect();
            let drawn = render_char_map_w(&grid, 1, " ", false);
            let parsed = utils::ocr::ascii_art_6_to_str(&drawn);
            return (parsed, t);
        }
        stars = new_stars;
        last_size = size;
    }
    unreachable!();
}
