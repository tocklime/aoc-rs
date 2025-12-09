use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::all_consuming,
    multi::separated_list1,
};
use utils::{aabb::Aabb, cartesian::Point, grid2d::Grid2d, nom::NomError};

aoc_harness::aoc_main!(2025 day 9, generator generate, part1 [p1] => 4_733_727_792, part2 [p2] => 1_566_346_198, example part1 EG => 50, example part2 EG => 24);

fn metric(a: [i64; 2], b: [i64; 2]) -> i64 {
    (0..2).map(|d| (a[d] - b[d]).abs() + 1).product::<i64>()
}

fn generate(input: &str) -> Vec<[i64; 2]> {
    all_consuming(separated_list1(
        newline::<_, NomError>,
        separated_list1(tag(","), complete::i64).map(|v| [v[0], v[1]]),
    ))
    .parse(input.trim())
    .unwrap()
    .1
}

fn p1(points: &[[i64; 2]]) -> i64 {
    points
        .iter()
        .combinations(2)
        .map(|x| metric(*x[0], *x[1]))
        .max()
        .unwrap()
}

fn p2(points: &[[i64; 2]]) -> i64 {
    let x_conv = CoordinateCompressor::new(points.iter().map(|x| x[0]));
    let y_conv = CoordinateCompressor::new(points.iter().map(|x| x[1]));
    let conv_points = points
        .iter()
        .map(|p| [x_conv.big_to_small[&p[0]], y_conv.big_to_small[&p[1]]])
        .collect_vec();
    let max_x = x_conv.big_to_small.len();
    let max_y = y_conv.big_to_small.len();
    let mut grid = Grid2d::from_elem(Point::new(max_x, max_y), 'o');
    for red_point in &conv_points {
        grid[(red_point[1], red_point[0])] = '#';
    }
    for (&r1, &r2) in conv_points
        .iter()
        .cycle()
        .take(conv_points.len() + 1)
        .tuple_windows()
    {
        if r1[0] == r2[0] {
            let sm = r1[1].min(r2[1]);
            let bi = r1[1].max(r2[1]);
            for n in sm + 1..bi {
                grid[(n, r1[0])] = 'X';
            }
        } else {
            assert_eq!(r1[1], r2[1]);
            let sm = r1[0].min(r2[0]);
            let bi = r1[0].max(r2[0]);
            for n in sm + 1..bi {
                grid[(r1[1], n)] = 'X';
            }
        }
    }
    //now fill outside from the outer elements.
    let aabb = Aabb::origin_and(grid.dim() - Point::new(1, 1));
    // dbg!(grid.dim());
    let mut to_fill: Vec<_> = aabb.perimeter().filter(|p| grid[p] == 'o').collect();
    while let Some(p) = to_fill.pop() {
        grid[p] = '.';
        let ns = grid.neighbours(p).filter(|x| grid[*x] == 'o');
        to_fill.extend(ns);
    }

    conv_points
        .iter()
        .combinations(2)
        .filter(|x| {
            let aabb: Aabb<usize> = x.iter().map(|x| **x).collect();
            for p in aabb.perimeter() {
                if grid[p] == '.' {
                    return false;
                }
            }
            true
        })
        .map(|x| {
            let big_a = [x_conv.small_to_big[x[0][0]], y_conv.small_to_big[x[0][1]]];
            let big_b = [x_conv.small_to_big[x[1][0]], y_conv.small_to_big[x[1][1]]];
            metric(big_a, big_b)
        })
        .max()
        .unwrap()
}

#[derive(Debug)]
struct CoordinateCompressor<BigCoord> {
    small_to_big: Vec<BigCoord>,
    big_to_small: HashMap<BigCoord, usize>,
}
impl<BigCoord: Ord + Copy + std::hash::Hash> CoordinateCompressor<BigCoord> {
    fn new<I: Iterator<Item = BigCoord>>(interesting: I) -> Self {
        let small_to_big: Vec<BigCoord> = interesting.sorted().unique().collect();
        let big_to_small = small_to_big
            .iter()
            .enumerate()
            .map(|(ix, b)| (*b, ix))
            .collect();
        Self {
            small_to_big,
            big_to_small,
        }
    }
}

const EG: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

//4654927679 too low.
