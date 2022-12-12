use aoc_harness::*;
use pathfinding::directed::bfs;
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

fn solve<FS, FC>(grid: &Grid2d<u8>, start: Coord, step_condition: FC, success: FS) -> usize
where
    FC: Fn(u8, u8) -> bool,
    FS: Fn(Coord, u8) -> bool,
{
    bfs::bfs(
        &start,
        |p: &(usize, usize)| {
                grid
                .neighbours(*p)
                .filter(|n| step_condition(grid[*p], grid[*n]))
                .collect::<Vec<_>>()
        },
        |p| success(*p, grid[*p])
    )
    .unwrap()
    .len()
        - 1
}

fn p1(input: &X) -> usize {
    solve(&input.grid, input.s_location, |p,n| p + 1 >= n, |p,_| p == input.e_location)
}

fn p2(input: &X) -> usize {
    solve(&input.grid, input.e_location, |p, n| n +1 >= p, |_, c| c == b'a')
}
