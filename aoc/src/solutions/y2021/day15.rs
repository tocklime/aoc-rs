
use num::Integer;
use pathfinding::prelude::dijkstra;
use utils::grid2d::{Coord, Grid2d};

aoc_harness::aoc_main!(2021 day 15, generator gen_, 
    part1 [solve::<1>] => 717, example part1 EG => 40, 
    part2 [solve::<5>] => 2993, example part2 EG => 315);

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
        self.map.dim() * self.repeats
    }

    fn neighbours(&'_ self, p: usize) -> impl Iterator<Item = Coord> {
        let s = self.dim();
        let p = p.div_mod_floor(&s.x);
        [
            (p.0.wrapping_sub(1), p.1),
            (p.0, p.1.wrapping_sub(1)),
            (p.0 + 1, p.1),
            (p.0, p.1 + 1),
        ]
        .map(Into::into)
        .into_iter()
        .filter(move |p: &Coord| p.y < s.y && p.x < s.x)
    }
    fn risk_at(&self, index: Coord) -> usize {
        let inner_dim = self.map.dim();
        let (outer_x, inner_x) = index.y.div_mod_floor(&inner_dim.y);
        let (outer_y, inner_y) = index.x.div_mod_floor(&inner_dim.x);
        let inner_val = self.map[(inner_y, inner_x)] as usize;
        1 + (outer_x + outer_y + inner_val - 1) % 9
    }
}

fn gen_(input: &str) -> Grid2d<u8> {
    Grid2d::from_str(input.trim(), |c| u8::try_from(c as u32).unwrap() - b'0')
}
fn solve<const REPEATS: usize>(input: &Grid2d<u8>) -> usize {
    let rg = RepeatingGrid {
        map: input,
        repeats: REPEATS,
    };
    let dim = rg.dim();
    let target = dim.y * dim.x - 1;
    dijkstra(
        &0,
        |&p| rg.neighbours(p).map(|p2| (p2.y * dim.x + p2.x, rg.risk_at(p2))),
        |&p| p == target,
    )
    .unwrap()
    .1
}
