use std::{cmp::{max, min}, cell::{Cell, RefCell}};

use aoc_harness::*;
use itertools::chain;
use num::Integer;
use pathfinding::prelude::{astar, dijkstra};
use utils::grid2d::{Coord, Grid2d};

aoc_main!(2021 day 15, generator gen, 
    part1 [solve_dijkstra::<1>] => 717, example part1 EG => 40, 
    part2 [solve_dijkstra::<5>] => 2993, example part2 EG => 315);

const EG: &str = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

struct RepeatingGrid<'a> {
    map: &'a Grid2d<u8>,
    repeats: usize,
}
impl RepeatingGrid<'_> {
    fn dim(&self) -> Coord {
        let (a, b) = self.map.dim();
        (a * self.repeats, b * self.repeats)
    }

    fn neighbours(&'_ self, p: Coord) -> impl Iterator<Item = Coord> {
        let s = self.dim();
        [
            (p.0.wrapping_sub(1), p.1),
            (p.0, p.1.wrapping_sub(1)),
            (p.0 + 1, p.1),
            (p.0, p.1 + 1),
        ]
        .into_iter()
        .filter(move |&x| x.0 < s.0 && x.1 < s.1)
    }
    fn risk_at(&self, index: Coord) -> usize {
        let inner_dim = self.map.dim();
        let (outer_x, inner_x) = index.0.div_mod_floor(&inner_dim.0);
        let (outer_y, inner_y) = index.1.div_mod_floor(&inner_dim.1);
        let inner_val = self.map[(inner_y, inner_x)] as usize;
        let risk = 1 + (outer_x + outer_y + inner_val - 1) % 9;
        risk
    }
}

fn gen(input: &str) -> Grid2d<u8> {
    Grid2d::from_str(input.trim(), |c| u8::try_from(c as u32).unwrap() - b'0')
}
fn solve_dijkstra<const REPEATS: usize>(input: &Grid2d<u8>) -> usize {
    let (y,x) = input.dim();
    let rg = RepeatingGrid {
        map: input,
        repeats: REPEATS,
    };
    let a = dijkstra(
        &(0, 0),
        |&p| rg.neighbours(p).map(|x| (x, rg.risk_at(x) as usize)),
        |(a, b)| (a + 1, b + 1) == rg.dim(),
    );
    a.unwrap().1
}
