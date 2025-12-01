use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use ndarray::{Array3, Dim, IntoDimension, Ix3};
use nom::{
    Parser, bytes::complete::tag, character::complete::{self, newline}, combinator::all_consuming, multi::separated_list1, sequence::{separated_pair, terminated}
};
use utils::nom::IResult;

aoc_harness::aoc_main!(2023 day 22, both [both] => (495, 76158), example both EG => (5,7));

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Brick {
    id: usize,
    from: Ix3,
    to: Ix3,
}
#[derive(Debug, Clone)]
struct World {
    space: Array3<Option<usize>>,
    bricks: Vec<Brick>,
    falls: Vec<usize>,
    removed: Option<usize>,
}

impl Brick {
    fn min_z(&self) -> usize {
        self.from[2]
    }
    fn blocks(&self, fall_dist: usize) -> impl Iterator<Item = Ix3> {
        (0..3)
            .map(|x| self.from[x]..=self.to[x])
            .multi_cartesian_product()
            .map(move |x| Dim([x[0], x[1], x[2] - fall_dist]))
    }
}

impl World {
    fn below(&self, b: usize) -> HashSet<usize> {
        let br = &self.bricks[b];
        let fall = self.falls[b];
        br.blocks(fall)
            .flat_map(|x| (x[2] > 0).then(|| self.space[x - Dim([0, 0, 1])]))
            .flatten()
            .filter(|x| *x != b)
            .collect()
    }
    fn remove(&mut self, brick: usize) {
        let fall_dist = self.falls[brick];
        for b in self.bricks[brick].blocks(fall_dist) {
            assert_eq!(self.space[b], Some(brick));
            self.space[b] = None;
        }
    }
    fn do_drops(&mut self) -> usize {
        // println!("Doing drops");
        let mut total_moves = 0;
        loop {
            let mut any_this_loop = false;
            for b in &self.bricks {
                if Some(b.id) == self.removed {
                    continue;
                }
                let current_fall = self.falls[b.id];
                let z = b.min_z();
                let max_fall = (current_fall + 1..z)
                    .take_while(|x| could_place(self, b.id, *x))
                    .last();
                if let Some(dist) = max_fall {
                    let old_fall_dist = self.falls[b.id];
                    any_this_loop = true;
                    total_moves += 1;
                    // println!("Dropping {} from {old_fall_dist} to {dist}", b.id);
                    // draw_world(self);
                    for bl in b.blocks(old_fall_dist) {
                        assert_eq!(
                            self.space[bl],
                            Some(b.id),
                            "brick {b:?} is not where it should be"
                        );
                        self.space[bl] = None;
                    }
                    self.falls[b.id] = dist;
                    for bl in self.bricks[b.id].blocks(dist) {
                        assert_eq!(self.space[bl], None);
                        self.space[bl] = Some(b.id);
                    }
                }
            }
            if !any_this_loop {
                break;
            }
        }
        total_moves
    }
}
fn parse_brick(input: &str) -> IResult<Brick> {
    let (input, (a, b)): (&str, ([usize; 3], [usize; 3])) = separated_pair(
        separated_list1(tag(","), complete::u32.map(|x| x as usize)).map(|x| x.try_into().unwrap()),
        tag("~"),
        separated_list1(tag(","), complete::u32.map(|x| x as usize)).map(|x| x.try_into().unwrap()),
    )
    .parse(input)?;

    let from = a.into_dimension();
    let to = b.into_dimension();
    Ok((input, Brick { id: 0, from, to }))
}
fn parse_bricks(input: &str) -> IResult<Vec<Brick>> {
    terminated(separated_list1(newline, parse_brick), newline).parse(input)
}

fn could_place(world: &World, brick: usize, fall_dist: usize) -> bool {
    world.bricks[brick]
        .blocks(fall_dist)
        .all(|b| world.space[b].is_none() || world.space[b] == Some(brick))
}

fn place(world: &mut World, brick: usize, fall_dist: usize) {
    world.falls[brick] = fall_dist;
    for b in world.bricks[brick].blocks(fall_dist) {
        assert!(world.space[b].is_none());
        world.space[b] = Some(brick);
    }
}

#[allow(dead_code)]
fn draw_world(world: &World) {
    for z in 0..10 {
        println!("z: {z}");
        for y in 0..10 {
            for x in 0..10 {
                print!(
                    "{}",
                    if let Some(b) = world.space[[x, y, z]] {
                        (b % 10).to_string().chars().next().unwrap()
                        // (b'A' + u8::try_from(b % 10).unwrap()) as char
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
        println!("\n");
    }
}

impl World {
    fn new(bricks: Vec<Brick>) -> Self {
        let mut world = World {
            space: Array3::default([10, 10, 400]),
            falls: vec![0; bricks.len()],
            bricks,
            removed: None,
        };
        for br in 0..world.bricks.len() {
            place(&mut world, br, 0);
        }
        world
    }
}

fn both(input: &str) -> (usize, usize) {
    let (_, mut bricks) = all_consuming(parse_bricks)
        .parse(input)
        .expect("parse");
    bricks.sort_by_key(Brick::min_z);
    for (ix, b) in bricks.iter_mut().enumerate() {
        b.id = ix;
    }
    // bricks.sort_by_key(Brick::min_z);
    let mut world = World::new(bricks);
    world.do_drops();
    //can we remove this?
    //removable iff it's not the only brick supporting something.
    //build a map of (A is supported by [B,C,D])
    //then a brick is not removable if its ever alone on the rhs in that map.
    let supported_by: HashMap<&Brick, HashSet<usize>> = world
        .bricks
        .iter()
        .map(|b| (b, world.below(b.id)))
        .collect();
    //a is unsafe because it is the only brick supporting...
    let mut unsafe_to_remove: HashSet<usize> = HashSet::new();
    for supporters in supported_by.values() {
        if supporters.len() == 1 {
            let supporter = *supporters.iter().next().unwrap();
            unsafe_to_remove.insert(supporter);
        }
    }
    let p1 = world.bricks.len() - unsafe_to_remove.len();
    // draw_world(&world);

    let p2 = unsafe_to_remove
        .par_iter()
        // .iter()
        .map(|x| {
            let mut new_world = world.clone();
            new_world.remove(*x);
            new_world.removed = Some(*x);
            new_world.do_drops()
        })
        .sum();

    (p1, p2)
}

const EG: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
