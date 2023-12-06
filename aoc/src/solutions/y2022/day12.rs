
use utils::grid2d::{Coord, Grid2d};

aoc_harness::aoc_main!(2022 day 12, generator gen, part1 [p1_astar, p1_bfs] => 412, part2 [p2] => 402, example both EG => (31,29));

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
impl X {
    fn neighbours(&self, p: &Coord) -> impl IntoIterator<Item = Coord> + '_ {
        let p = *p;
        self.grid
            .neighbours(p)
            .filter(move|n| self.grid[*n] + 1 >= self.grid[p])
    }
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

fn p1_astar(input: &X) -> usize {
    pathfinding::directed::astar::astar(
        &input.e_location,
        |p| input.neighbours(p).into_iter().map(|x| (x, 1)),
        |&(y, x): &Coord| {
            usize::abs_diff(y, input.s_location.0) + usize::abs_diff(x, input.s_location.1)
        },
        |&p| p == input.s_location,
    )
    .unwrap()
    .1
}
fn p1_bfs(input: &X) -> usize {
    pathfinding::directed::bfs::bfs(
        &input.e_location,
        |p| input.neighbours(p),
        |&p| p == input.s_location,
    )
    .unwrap()
    .len()
        - 1
}

fn p2(input: &X) -> usize {
    pathfinding::directed::bfs::bfs(
        &input.e_location,
        |p| input.neighbours(p),
        |p| input.grid[*p] == b'a',
    )
    .unwrap()
    .len()
        - 1
}
