use std::collections::HashSet;

use aoc_harness::*;
use ndarray::Array2;

aoc_main!(2021 day 9, part1 [p1], part2 [p2],
          example part1 EG => 15, example part2 EG => 1134);
const EG: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
fn p1(input: &str) -> usize {
    let wid = input.lines().next().unwrap().len();
    let hei = input.lines().count();
    let mut grid = Array2::from_elem((hei, wid), 0_usize);
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid[(y, x)] = (c as u32 - '0' as u32) as usize;
        }
    }
    println!("{}", &grid);
    (0..hei)
        .cartesian_product(0..wid)
        .filter(|&(y, x)| {
            let here = grid[(y, x)];
            let up = y == 0 || grid[(y - 1, x)] > here;
            let down = y + 1 == hei || grid[(y + 1, x)] > here;
            let left = x == 0 || grid[(y, x - 1)] > here;
            let right = x + 1 == wid || grid[(y, x + 1)] > here;
            if up && down && left && right {
                println!("{} {} {}", x, y, here);
                true
            } else {
                false
            }
        })
        .map(|x| grid[x] + 1)
        .sum::<usize>()
}

fn p2(input: &str) -> usize {
    let wid = input.lines().next().unwrap().len();
    let hei = input.lines().count();
    let mut grid = Array2::from_elem((hei, wid), 0_usize);
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid[(y, x)] = (c as u32 - '0' as u32) as usize;
        }
    }
    println!("{}", &grid);
    let low_points = (0..hei)
        .cartesian_product(0..wid)
        .filter(|&(y, x)| {
            let here = grid[(y, x)];
            let up = y == 0 || grid[(y - 1, x)] > here;
            let down = y + 1 == hei || grid[(y + 1, x)] > here;
            let left = x == 0 || grid[(y, x - 1)] > here;
            let right = x + 1 == wid || grid[(y, x + 1)] > here;
            if up && down && left && right {
                println!("{} {} {}", x, y, here);
                true
            } else {
                false
            }
        })
        .collect_vec();
    let mut sizes = Vec::new();
    for b in low_points {
        let mut fringe = vec![b];
        let mut done = HashSet::new();
        while !fringe.is_empty() {
            let (y, x) = fringe.pop().unwrap();
            if !done.contains(&(y, x)) {
                let up = if y == 0 { 9 } else { grid[(y - 1, x)] };
                let down = if y + 1 == hei { 9 } else { grid[(y + 1, x)] };
                let left = if x == 0 { 9 } else { grid[(y, x - 1)] };
                let right = if x + 1 == wid { 9 } else { grid[(y, x + 1)] };
                if up < 9 {
                    fringe.push((y - 1, x));
                }
                if down < 9 {
                    fringe.push((y + 1, x))
                };
                if left < 9 {
                    fringe.push((y, x - 1))
                };
                if right < 9 {
                    fringe.push((y, x + 1))
                };
                done.insert((y, x));
            }
        }
        sizes.push(done.len())
    }
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}
