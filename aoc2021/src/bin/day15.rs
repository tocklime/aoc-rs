use std::str::FromStr;

use aoc_harness::*;
use pathfinding::prelude::dijkstra;
use utils::grid2d::Grid2d;

aoc_main!(2021 day 15, generator whole_input_is::<X>, part1 [p1], example part1 EG => 40, part2 [p2], example part2 EG => 315);

const EG: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

#[derive(Debug)]
struct X {
    map: Grid2d<u8>,
}

impl FromStr for X {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = Grid2d::from_str(s, |c| (c as u32) as u8 - b'0');
        Ok(Self { map })
    }
}
fn p1(input: &X) -> usize {
    let a = dijkstra(
        &(0, 0),
        |&p| input.map.neighbours(p).map(|x| (x, input.map[x] as usize)),
        |(a, b)| (a + 1, b + 1) == input.map.dim(),
    );
    a.unwrap().1
}

fn p2(input: &X) -> usize {
    let (a, b) = input.map.dim();
    let mut big_grid = Grid2d::from_elem((a * 5, b * 5), 0);
    for (x, y) in (0..a * 5).cartesian_product(0..b * 5) {
        let (bx, by) = (x / a, y / a);
        let incr = bx + by;
        let val = incr + input.map[(x % a, y % b)] as usize;
        big_grid[(x, y)] = if val > 9 { val - 9 } else { val };
    }
    let a = dijkstra(
        &(0, 0),
        |&p| big_grid.neighbours(p).map(|x| (x, big_grid[x] as usize)),
        |(a, b)| (a + 1, b + 1) == big_grid.dim()
    );
    a.unwrap().1
}
