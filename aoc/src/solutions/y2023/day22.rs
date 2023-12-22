use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use ndarray::{Array3, IntoDimension, Ix3, Dim};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use nom_supreme::ParserExt;
use utils::nom::IResult;

aoc_harness::aoc_main!(2023 day 22, both [both] => (495, 76158), example both EG => (5,7));

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Brick {
    id: usize,
    from: Ix3,
    to: Ix3,
}
struct World {
    space : Array3<Option<usize>>,
    bricks: Vec<Brick>,
    falls: Vec<usize>,
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

    fn below(&self, w: &World) -> HashSet<usize> {
        self.blocks(0).flat_map(|x| (x[2]>0).then(|| w.space[x - Dim([0,0,1])])).flatten().collect()
    }
}

fn parse_brick(input: &str) -> IResult<Brick> {
    let (input, (a, b)): (&str, ([usize; 3], [usize; 3])) = separated_pair(
        separated_list1(tag(","), complete::u32.map(|x| x as usize)).map(|x| x.try_into().unwrap()),
        tag("~"),
        separated_list1(tag(","), complete::u32.map(|x| x as usize)).map(|x| x.try_into().unwrap()),
    )(input)?;

    let from = a.into_dimension();
    let to = b.into_dimension();
    Ok((input, Brick { id: 0, from, to }))
}
fn parse_bricks(input: &str) -> IResult<Vec<Brick>> {
    separated_list1(newline, parse_brick)
        .terminated(newline)
        .parse(input)
}

fn could_place(world: &World, brick: &Brick, fall_dist: usize) -> bool {
    brick.blocks(fall_dist).all(|b| world.space[b].is_none() || world.space[b] == Some(brick.id))
}
fn remove(world:&mut World, brick: &Brick, fall_dist: usize) {
    for b in brick.blocks(fall_dist) {
        assert_eq!(world.space[b], Some(brick.id));
        world.space[b] = None;
    }
}

fn place(world: &mut World, brick: &Brick, fall_dist: usize) {
    for b in brick.blocks(fall_dist) {
        assert!(world.space[b].is_none());
        world.space[b] = Some(brick.id);
    }
}

fn do_drops<'a>(world: &mut World, bricks: &[Brick]) {
    for z in 0..world.space.dim().2 {
        let mut anything_fell = false;


        if !anything_fell {
            break;
        }
    }

}

#[allow(dead_code)]
fn draw_world(world: &World) {
    for z in 0..10 {
        println!("z: {z}");
        for y in 0..3 {
            for x in 0..3 {
                print!(
                    "{}",
                    if let Some(b) = world.space[[x, y, z]] {
                        (b'A' + u8::try_from(b % 10).unwrap()) as char
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

fn build_world(
    bricks: &[Brick],
    without: Option<usize>,
) -> World {
    let mut world = World {
        space : Array3::default([10,10,400]),
        bricks: bricks.to_vec(),
        falls: vec![0;bricks.len()],
    };
    for br in bricks {
        if Some(br.id) != without {
            let correct_fall = (0..br.min_z())
                .take_while(|x| could_place(&world, br, *x))
                .last()
                .unwrap();
            world.falls[br.id] = correct_fall;
            place(&mut world, br, correct_fall);
        }
    }
    world
}

fn both(input: &str) -> (usize, usize) {
    let (_, mut bricks) = parse_bricks
        .all_consuming()
        .complete()
        .parse(input)
        .expect("parse");
    for (ix, b) in bricks.iter_mut().enumerate() {
        b.id = ix;
    }
    bricks.sort_by_key(Brick::min_z);
    let world = build_world(&bricks, None);
    //can we remove this?
    //removable iff it's not the only brick supporting something.
    //build a map of (A is supported by [B,C,D])
    //then a brick is not removable if its ever alone on the rhs in that map.
    let mut supported_by: HashMap<&Brick, HashSet<usize>> = HashMap::new();
    for b in &bricks {
        let fall = world.falls[b.id];
        for block in b.blocks(fall) {
            if block[2] > 1 {
                if let Some(supp) = world.space[block - Dim([0,0,1])] {
                    if supp != b.id {
                        supported_by.entry(b).or_default().insert(supp);
                    }
                }
            }
        }
    }
    //a is unsafe because it is the only brick supporting...
    let mut unsafe_to_remove: HashMap<usize, Vec<&Brick>> = HashMap::new();
    for (supported, supporters) in &supported_by {
        if supporters.len() == 1 {
            let supporter = *supporters.iter().next().unwrap();
            unsafe_to_remove
                .entry(supporter)
                .or_default()
                .push(*supported);
        }
    }
    let p1 = bricks.len() - unsafe_to_remove.len();

    progress_bar::init_progress_bar(unsafe_to_remove.len());
    let p2 = unsafe_to_remove
        .par_iter()
        .map(|(x, _)| {
            progress_bar::inc_progress_bar();
            let new_world = build_world(&bricks, Some(*x));
            new_world.falls.iter().zip(&world.falls).filter(|(a,b)| a != b).count()
            // new_world.falls.iter().enumerate().filter(|(k, v)| world.falls[**k] != **v).count()
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
