use std::{
    cmp::{max, min},
    collections::HashMap,
};

use aoc_harness::*;
use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use utils::{
    cartesian::{self, Point},
    grid2d::{Coord, Grid2d},
    prelude::render_char_map_w, aabb::Aabb,
};

aoc_main!(2022 day 14, part1 [p1], part2 [p2], example both EG => (24,93));

const EG: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

fn line(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(
        tag(" -> "),
        separated_pair(complete::u32, tag(","), complete::u32),
    )(input)
}

fn next_sand(grid: &Grid2d<char>, fill_point: Coord) -> Option<Coord> {
    let mut pos = fill_point;
    loop {
        if grid.get((pos.0 + 1, pos.1))? == &'.' {
            pos.0 += 1;
        } else if grid.get((pos.0 + 1, pos.1.checked_sub(1)?))? == &'.' {
            pos.0 += 1;
            pos.1 -= 1;
        } else if grid.get((pos.0 + 1, pos.1 + 1))? == &'.' {
            pos.0 += 1;
            pos.1 += 1;
        } else {
            return Some(pos);
        }
    }
}
fn p1(input: &str) -> usize {
    let p = input
        .lines()
        .map(|l| line(l).unwrap().1)
        .collect::<Vec<_>>();
    let (min_x, max_x) = p
        .iter()
        .flatten()
        .map(|v| v.0)
        .minmax()
        .into_option()
        .unwrap();
    let max_y = p.iter().flatten().map(|v| v.1).max().unwrap();
    let mut grid = Grid2d::from_elem((1 + max_y as usize, 1 + (max_x - min_x) as usize), '.');
    for l in p {
        for (a, b) in l.iter().tuple_windows() {
            let min = min(a, b);
            let max = max(a, b);
            let x_delta = if max.0 > min.0 { 1 } else { 0 };
            let y_delta = if max.1 > min.1 { 1 } else { 0 };
            let mut pos = (min.1 as usize, (min.0 - min_x) as usize);
            let end = (max.1 as usize, (max.0 - min_x) as usize);
            grid[pos] = '#';
            while pos != end {
                pos.0 += y_delta;
                pos.1 += x_delta;
                grid[pos] = '#';
            }
        }
    }
    let mut c = 0;
    while let Some(p) = next_sand(&grid, (0, 500 - min_x as usize)) {
        c += 1;
        grid[p] = 'o';
    }
    // println!("{}", grid);

    c
}
fn next_sand2(
    grid: &HashMap<Point<i32>, char>,
    fill_point: Point<i32>,
    max_y: i32,
) -> Option<Point<i32>> {
    let mut pos = fill_point;
    loop {
        // dbg!(max_y, pos);
        if pos.y == max_y {
            //this is the bottom. stop here.
            return Some(pos);
        } else if grid.get(&pos.up()) == None {
            pos = pos.up();
        } else if grid.get(&pos.up().left()) == None {
            pos = pos.up().left();
        } else if grid.get(&pos.up().right()) == None {
            pos = pos.up().right();
        } else {
            return Some(pos);
        }
    }
}
fn p2(input: &str) -> usize {
    let p = input
        .lines()
        .map(|l| line(l).unwrap().1)
        .collect::<Vec<_>>();
    let (min_x, max_x) = p
        .iter()
        .flatten()
        .map(|v| v.0)
        .minmax()
        .into_option()
        .unwrap();
    let max_y = p.iter().flatten().map(|v| v.1).max().unwrap();
    let mut grid: HashMap<Point<i32>, char> = HashMap::new(); //Grid2d::from_elem((2 + max_y as usize, 1 + (max_x) as usize), '.');
    for l in p {
        for (a, b) in l.iter().tuple_windows() {
            let min = min(a, b);
            let max = max(a, b);
            let x_delta = if max.0 > min.0 { 1 } else { 0 };
            let y_delta = if max.1 > min.1 { 1 } else { 0 };
            let mut pos = Point::new(min.0 as i32, (min.1) as i32);
            let end = Point::new(max.0 as i32, (max.1) as i32);
            grid.insert(pos, '#');
            while pos != end {
                pos.y += y_delta;
                pos.x += x_delta;
                grid.insert(pos, '#');
            }
        }
    }
    let mut c = 0;
    let fill_point = Point::new(500, 0);
    let mut bb = Aabb::new(fill_point);
    while let Some(p) = next_sand2(&grid, fill_point, max_y as i32 + 1) {
        c += 1;
        bb = bb.extend(p);
        grid.insert(p, 'o');
        if p == fill_point {
            break;
        }
    }
    dbg!(bb);

    c
}
