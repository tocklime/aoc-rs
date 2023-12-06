use std::{collections::HashMap, str::FromStr};

use aoc_harness::*;
use utils::cube::Cube;

aoc_harness::aoc_main!(2021 day 22, generator lines::<X>,
    example part1 EG => 39,
    part1 [by_block::<20>, construction::<20>, region_weights::<20>] => 582_644,
    part2 [by_block::<420>, construction::<420>, region_weights::<420>] => 1_263_804_707_062_415);

const EG: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct X {
    target_state: bool,
    cube: Cube,
}

impl FromStr for X {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mode,c_str) = s.split_once(' ').ok_or_else(||"No space".to_owned())?;
        let mut cube : Cube = c_str.parse().map_err(|x : parse_display::ParseError| x.to_string())?;
        cube.make_upper_inclusive();
        Ok(Self {
            target_state: mode == "on",
            cube
        })
    }
}

fn volume_remaining(me: &Cube, future: &[Cube]) -> isize {
    let mut my_vol = me.size();
    let filtered_future = future
        .iter()
        .filter_map(|x| me.intersection(x))
        .collect_vec();
    for ix in 0..filtered_future.len() {
        my_vol -= volume_remaining(&filtered_future[ix], &filtered_future[ix + 1..]);
    }
    my_vol
}

fn by_block<const N: usize>(input: &[X]) -> isize {
    let mut ans = 0;
    let cubes = input.iter().map(|x| x.cube).collect_vec();
    for (ix, i) in input.iter().enumerate().take(N) {
        if i.target_state {
            let x = volume_remaining(&i.cube, &cubes[ix + 1..]);
            ans += x;
        }
    }
    ans
}

fn region_weights<const N: usize>(input: &[X]) -> isize {
    let mut weights: HashMap<Cube, isize> = HashMap::new();
    for i in input.iter().take(N) {
        let mut new_weights = weights.clone();
        for (k, w) in weights {
            if let Some(intersection) = k.intersection(&i.cube) {
                *new_weights.entry(intersection).or_default() -= w;
            }
        }
        if i.target_state {
            *new_weights.entry(i.cube).or_default() += 1;
        }
        new_weights.retain(|_, v| *v != 0);
        weights = new_weights;
    }
    weights.into_iter().map(|(c, s)| c.size() * s).sum()
}

fn construction<const N: usize>(input: &[X]) -> isize {
    let mut ons: Vec<Cube> = Vec::new();
    for i in input.iter().take(N) {
        let mut new_ons = Vec::with_capacity(ons.len());
        for o in ons {
            if o.intersects(&i.cube) {
                new_ons.extend(o.subtract(&i.cube));
            } else {
                new_ons.push(o);
            }
        }
        if i.target_state {
            new_ons.push(i.cube);
        }
        ons = new_ons;
    }
    ons.into_iter().map(|x| x.size()).sum()
}
