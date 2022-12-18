use ndarray::prelude::*;
use std::collections::HashSet;

use aoc_harness::*;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::{all_consuming, map, opt},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

aoc_main!(2022 day 18, both [fixed_arrays] => (3432,2042),
    part1 [p1] => 3432, part2 [p2] => 2042,
    example both EG0 => (10,10), example both EG => (64,58));

const EG0: &str = "1,1,1\n2,1,1";
const EG: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

fn parse_line(input: &str) -> IResult<&str, (usize, usize, usize)> {
    tuple((
        terminated(map(complete::u32, |x| x as usize), tag(",")),
        terminated(map(complete::u32, |x| x as usize), tag(",")),
        terminated(map(complete::u32, |x| x as usize), opt(newline)),
    ))(input)
}
const WORLD_SIZE: usize = 21;
fn neighbours(c: &(usize, usize, usize)) -> impl Iterator<Item = Option<(usize, usize, usize)>> {
    let &(x, y, z) = c;
    [
        z.checked_sub(1).map(|nz| (x, y, nz)),
        if z + 1 < WORLD_SIZE {
            Some((x, y, z + 1))
        } else {
            None
        },
        y.checked_sub(1).map(|ny| (x, ny, z)),
        if y + 1 < WORLD_SIZE {
            Some((x, y + 1, z))
        } else {
            None
        },
        x.checked_sub(1).map(|nx| (nx, y, z)),
        if x + 1 < WORLD_SIZE {
            Some((x + 1, y, z))
        } else {
            None
        },
    ]
    .into_iter()
}
fn make_hashset(input: &str) -> HashSet<(usize, usize, usize)> {
    all_consuming(many1(parse_line))(input)
        .unwrap()
        .1
        .into_iter()
        .collect()
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Space {
    Air,
    Lava,
    OpenAir,
}
use Space::*;

fn count_neighbours(
    points: &[(usize, usize, usize)],
    world: &Array3<Space>,
    target: Space,
) -> usize {
    points
        .iter()
        .map(|c| {
            neighbours(c)
                .filter(|n| match n {
                    Some(n) => world[*n] == target,
                    None => true,
                })
                .count()
        })
        .sum()
}
fn fixed_arrays(input: &str) -> (usize, usize) {
    let points = all_consuming(many1(parse_line))(input).unwrap().1;
    let mut world = Array3::from_elem((WORLD_SIZE, WORLD_SIZE, WORLD_SIZE), Air);
    for p in &points {
        world[*p] = Lava;
    }
    let p1 = count_neighbours(&points, &world, Air);

    //bfs from origin to find all open air
    let mut fringe = vec![(0,0,0)];
    while let Some(p) = fringe.pop() {
        for n in neighbours(&p).flatten() {
            if let Some(p) = world.get_mut(n) {
                if Air == *p { *p = OpenAir; fringe.push(n); }
            }
        }
    }
    let p2 = count_neighbours(&points, &world, OpenAir);
    (p1, p2)
}
fn p1(input: &str) -> usize {
    make_hashset(input)
        .iter()
        .map(|c| {
            neighbours(c)
                .filter(|n| n.is_none() || !make_hashset(input).contains(&n.unwrap()))
                .count()
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let mut occupied = HashSet::new();
    for l in input.lines() {
        let (_, ns) = parse_line(l).unwrap();
        occupied.insert(ns);
    }
    let mut ans = 0;
    let mut fringe = vec![(0,0,0)];
    let mut open = HashSet::new();
    while let Some(p) = fringe.pop() {
        for n in neighbours(&p).flatten() {
            if !occupied.contains(&n) && open.insert(n) {
                fringe.push(n);
            }
        }

    }
    for c in occupied {
        for n in neighbours(&c) {
            ans += match n {
                Some(n) => usize::from(open.contains(&n)),
                None => 1,
            }
        }
    }
    ans
}
