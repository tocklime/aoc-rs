use utils::{
    cartesian::{Dir, Point},
    grid2d::Grid2d,
    numset::NumSet,
};

aoc_harness::aoc_main!(2024 day 6, part1 [p1] => 4696, part2 [p2] => 1443, example part1 EG => 41, example part2 EG => 6);

fn p1(input: &str) -> usize {
    let mut g = Grid2d::from_str_as_char(input).flip_y();
    let mut pos = g.find(|c| c == &'^').unwrap().0;
    let mut dir = Dir::Up;
    loop {
        g[pos] = 'X';
        let next = pos.step(dir);
        match g.get(next) {
            None => break,
            Some('#') => {
                dir = dir.turn_right();
                continue;
            }
            Some(_) => pos = next,
        }
    }
    g.iter().filter(|x| x == &&'X').count()
}
fn cycles(g: &Grid2d<char>, start: Point<usize>, obstruction: Point<usize>) -> bool {
    let mut been = Grid2d::from_fn(g.dim(), |_| NumSet::<u8>::default());
    let mut pos = start;
    let mut dir = Dir::Down;
    loop {
        if !been[pos].insert(dir as u8) {
            return true;
        }
        let next = pos.step(dir);
        match g.get(next) {
            None => return false,
            Some('#') => {
                dir = dir.turn_left();
                continue;
            }
            Some(_) => {
                if next == obstruction {
                    dir = dir.turn_left();
                } else {
                    pos = next
                }
            }
        }
    }
}

fn p2(input: &str) -> usize {
    let g = Grid2d::from_str_as_char(input);
    let start = g.find(|c| c == &'^').unwrap().0;
    g.indexed_iter()
        .filter(|x| x.1 == &'.')
        .filter(|(p, _)| cycles(&g, start, *p))
        .count()
}

const EG: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
