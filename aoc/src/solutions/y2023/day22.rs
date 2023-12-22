use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use ndarray::{Array3, IntoDimension, Ix3};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use nom_supreme::ParserExt;
use utils::nom::IResult;

aoc_harness::aoc_main!(2023 day 22, both [both], example both EG => (5,7));

#[derive(Debug, PartialEq, Eq, Hash)]
struct Brick {
    id: usize,
    from: Ix3,
    to: Ix3,
}
impl Brick {
    fn min_z(&self) -> usize {
        self.from[2].min(self.to[2])
    }
    fn blocks(&self, fall_dist: usize) -> impl Iterator<Item = Ix3> {
        (0..3)
            .map(|x| self.from[x]..=self.to[x])
            .multi_cartesian_product()
            .map(move |x| [x[0], x[1], x[2] - fall_dist].into_dimension())
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

fn could_place<'a>(world: &Array3<Option<&'a Brick>>, brick: &'a Brick, fall_dist: usize) -> bool {
    brick.blocks(fall_dist).all(|b| world[b].is_none())
}
fn place<'a>(world: &mut Array3<Option<&'a Brick>>, brick: &'a Brick, fall_dist: usize) {
    for b in brick.blocks(fall_dist) {
        assert!(world[b].is_none());
        world[b] = Some(brick);
    }
}

fn draw_world(world: &Array3<Option<&Brick>>) {
    for z in 0..10 {
        println!("z: {z}");
        for y in 0..3 {
            for x in 0..3 {
                print!(
                    "{}",
                    if let Some(b) = world[[x, y, z]] {
                        (b'A' + u8::try_from(b.id % 10).unwrap()) as char
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


fn build_world<'a>(bricks: &'a[Brick], without: Option<&Brick>) -> (HashMap<&'a Brick, usize>, Array3<Option<&'a Brick>>) {
    let mut world = ndarray::Array3::<Option<&Brick>>::default([10, 10, 1000]);
    let mut falls = HashMap::new();
    for br in bricks {
        if Some(br) != without {
            let correct_fall = (0..br.min_z())
                .take_while(|x| could_place(&world, br, *x))
                .last()
                .unwrap();
            falls.insert(br, correct_fall);
            place(&mut world, br, correct_fall);
        }
    }
    (falls, world)
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
    let (falls, world) = build_world(&bricks, None);
    //can we remove this?
    //removable iff it's not the only brick supporting something.
    //build a map of (A is supported by [B,C,D])
    //then a brick is not removable if its ever alone on the rhs in that map.
    let mut supported_by: HashMap<&Brick, HashSet<&Brick>> = HashMap::new();
    for b in &bricks {
        let fall = falls[b];
        for block in b.blocks(fall) {
            if block[2] > 1 {
                if let Some(supp) = world[block - [0, 0, 1].into_dimension()] {
                    if supp != b {
                        supported_by.entry(b).or_default().insert(supp);
                    }
                }
            }
        }
    }
    let mut unsafe_to_remove: HashSet<&Brick> = HashSet::new();
    for supporters in supported_by.values() {
        if supporters.len() == 1 {
            unsafe_to_remove.extend(supporters);
        }
    }
    // dbg!(&supported_by);
    // dbg!(&unsafe_to_remove);
    let p1 = bricks.len() - unsafe_to_remove.len();

    progress_bar::init_progress_bar(unsafe_to_remove.len());
    let p2 = unsafe_to_remove.iter().map(|x| {
        progress_bar::inc_progress_bar();
        let (new_falls, _world) = build_world(&bricks, Some(*x));
        new_falls.iter().filter(|(k,v)| falls[**k] != **v).count()
    }).sum();


    (p1,p2)
}

const EG: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
