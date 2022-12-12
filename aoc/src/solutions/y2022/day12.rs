use aoc_harness::*;
use utils::grid2d::{Coord, Grid2d};

aoc_main!(2022 day 12, generator gen, part1 [p1] => 412, part2 [p2] => 402, example both EG => (31,29));

const EG: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
struct X {
    grid: Grid2d<u8>,
    s_location: (usize, usize),
    e_location: (usize, usize),
}
fn gen(input: &str) -> X {
    let mut s_location = (0, 0);
    let mut e_location = (0, 0);
    let s = &mut s_location;
    let e = &mut e_location;
    let grid = Grid2d::from_str_with_index(input, move |coord, c| match c {
        'S' => {
            *s = coord;
            b'a'
        }
        'E' => {
            *e = coord;
            b'z'
        }
        c => c as u8,
    });
    X {
        grid,
        s_location,
        e_location,
    }
}

fn p1(input: &X) -> usize {
    pathfinding::directed::astar::astar(
        &input.e_location,
        |p: &(usize, usize)| {
            input
                .grid
                .neighbours(*p)
                .filter(|n| input.grid[*n] + 1 >= input.grid[*p])
                .map(|p| (p, 1))
                .collect::<Vec<_>>()
        },
        |&(y, x): &Coord| {
            usize::abs_diff(y, input.s_location.0) + usize::abs_diff(x, input.s_location.1)
        },
        |&p| p == input.s_location,
    )
    .unwrap()
    .1
}

fn p2(input: &X) -> usize {
    pathfinding::directed::bfs::bfs(
        &input.e_location,
        |p: &(usize, usize)| {
            input
                .grid
                .neighbours(*p)
                .filter(|n| input.grid[*n] + 1 >= input.grid[*p])
                .collect::<Vec<_>>()
        },
        |p| input.grid[*p] == b'a',
    )
    .unwrap()
    .len()
        - 1
}
