use std::collections::HashMap;

use nom_supreme::parser_ext::OptionalPreceded;
use pathfinding::grid::Grid;
use utils::grid2d::{Grid2d, Coord};

aoc_harness::aoc_main!(2023 day 14, part1 [p1], part2 [p2], example both EG => (136, 64));

fn roll_north(g: &mut Grid2d<char>) {
    let movable_positions : Vec<Coord> = g.indexed_iter().filter(|x| x.1 == &'O').map(|x| x.0).collect();
    for p in movable_positions {
        //going through in order, so we're sure there's nothing above us that's going to move.
        let new_y = (0..p.y).rev().find(|y| g[(*y,p.x)] != '.').map(|y| y+1).unwrap_or_default();
        g[p] = '.';
        g[(new_y,p.x)] = 'O';
    }
}
fn roll_south(g: &mut Grid2d<char>) {
    let movable_positions : Vec<Coord> = g.indexed_iter().filter(|x| x.1 == &'O').map(|x| x.0).collect();
    for p in movable_positions.iter().rev() {
        //going through in order, so we're sure there's nothing below us that's going to move.
        let new_y = (p.y+1..g.dim().y).find(|y| g[(*y,p.x)] != '.').unwrap_or(g.dim().y)-1;
        g[*p] = '.';
        g[(new_y,p.x)] = 'O';
    }
}
fn roll_east(g: &mut Grid2d<char>) {
    let s = g.dim();
    for y in 0..s.y {
        for x in (0..s.x).rev() {
            if g[(y,x)] == 'O' {
                let new_x = (x+1..s.x).find(|nx| g[(y,*nx)] != '.').unwrap_or(s.x)-1;
                g[(y,x)] = '.';
                g[(y,new_x)] = 'O';
            }
        }
    }
}
fn roll_west(g: &mut Grid2d<char>) {
    let s = g.dim();
    for y in 0..s.y {
        for x in 1..s.x {
            if g[(y,x)] == 'O' {
                let new_x = (0..x).rev().find(|nx| g[(y,*nx)] != '.').map(|x| x+1).unwrap_or(0);
                g[(y,x)] = '.';
                g[(y,new_x)] = 'O';
            }
        }
    }
}

fn calc_north_load(g: &Grid2d<char>) -> usize {
    let s = g.dim();
    g.indexed_iter().map(|(p, c)| if c == &'O' {s.y - p.y} else {0}).sum()
}
fn p1(input: &str) -> usize {
    let mut g = Grid2d::from_str(input, |x| x);
    roll_north(&mut g);

    calc_north_load(&g)
}
fn count_rollable(g: &Grid2d<char>) -> usize {
    g.iter().filter(|&x| x == &'O').count()
}
fn cycle(g: &mut Grid2d<char>) {
    let before = count_rollable(g);
    roll_north(g);
    roll_west(g);
    roll_south(g);
    roll_east(g);
    let after = count_rollable(g);
    assert_eq!(after, before);
}
fn p2(input: &str) -> usize {
    let mut g = Grid2d::from_str(input, |x| x);
    let mut seen = HashMap::new();
    seen.insert(g.clone(), 0);
    for x in 0..1_000_000_000 {
        cycle(&mut g);
        println!("After {}: {}", x, calc_north_load(&g));
        if let Some(old_count) = seen.get(&g) {
            // Cycle from {old_count} to {x}.
            let cycle_size = x - old_count;
            let remaining = 1_000_000_000 - x;
            let whole_cycles = remaining / cycle_size;
            let cycles_left_to_do = remaining - (whole_cycles * cycle_size) - 1;
            dbg!(old_count, x, cycle_size, remaining, whole_cycles, cycles_left_to_do);
            assert_eq!(x + (whole_cycles * cycle_size) + cycles_left_to_do, 1_000_000_000 -1);
            for n in 1..=cycles_left_to_do {
                cycle(&mut g);
                println!("After {n} more: {}", calc_north_load(&g));
            }
            return calc_north_load(&g)
        }
        seen.insert(g.clone(), x);
    }

    todo!()
}

const EG: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";