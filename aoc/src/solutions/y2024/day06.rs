use rayon::iter::{IntoParallelIterator, ParallelIterator};
use utils::{
    cartesian::{Dir, Point},
    grid2d::Grid2d,
    numset::NumSet,
};

aoc_harness::aoc_main!(2024 day 6, generator gen, part1 [p1] => 4696, part2 [bruteforce,p2,rayon] => 1443, example part1 EG => 41, example part2 EG => 6);

fn gen(input: &str) -> Grid2d<char> {
    Grid2d::from_str_as_char(input).flip_y()
}

fn grid_cycles(g: &Grid2d<char>, mut pos: Point<usize>, mut dir: Dir, new_obstacle: Point<usize>, seen: &Grid2d<NumSet<u8>>) -> bool {
    let mut seen = seen.clone();
    loop {
        if !seen[pos].insert(dir as u8) {
            return true;
        }
        let next = pos.step(dir);
        match g.get(next) {
            None => return false,
            Some('#') => {
                dir = dir.turn_right();
            }
            Some(_) => {
                if next == new_obstacle {
                    dir = dir.turn_right();
                } else {
                    pos = next;
                }
                
            }
        }
    }
}

fn all_touched_cells(input: &Grid2d<char>) -> Vec<Point<usize>> {
    let mut seen = Grid2d::from_elem(input.dim(),false);
    let mut pos = input.find(|c| c == &'^').unwrap().0;
    let mut dir = Dir::Up;
    loop {
        seen[pos] = true;
        let next = pos.step(dir);
        match input.get(next) {
            None => break,
            Some('#') => {
                dir = dir.turn_right();
                continue;
            }
            Some(_) => pos = next,
        }
    }
    seen.indexed_iter().filter_map(|x| x.1.then_some(x.0)).collect()
}

fn p1(input: &Grid2d<char>) -> usize {
    all_touched_cells(input).len()
}
fn cycles(g: &Grid2d<char>, start: Point<usize>, obstruction: Point<usize>) -> bool {
    let mut been = Grid2d::from_fn(g.dim(), |_| NumSet::<u8>::default());
    let mut pos = start;
    let mut dir = Dir::Up;
    loop {
        if !been[pos].insert(dir as u8) {
            return true;
        }
        let next = pos.step(dir);
        match g.get(next) {
            None => return false,
            Some('#') => {
                dir = dir.turn_right();
                continue;
            }
            Some(_) => {
                if next == obstruction {
                    dir = dir.turn_right();
                } else {
                    pos = next;
                }
            }
        }
    }
}

fn p2(input: &Grid2d<char>) -> usize {
    let start = input.find(|c| c == &'^').unwrap().0;
    let mut been = Grid2d::from_fn(input.dim(), |_| NumSet::<u8>::default());
    let mut obstacle_check = Grid2d::from_elem(input.dim(), false);
    let mut pos = start;
    let mut dir = Dir::Up;
    let mut cycle_count = 0;
    obstacle_check[start] = true;
    loop {
        been[pos].insert(dir as u8);
        let next = pos.step(dir);
        match input.get(next) {
            None => return cycle_count,
            Some('#') => {
                dir = dir.turn_right();
                continue;
            }
            Some(_) => {
                //what if this was actually an obstacle?
                if !obstacle_check[next] {
                    obstacle_check[next] = true;
                    if grid_cycles(input, pos, dir.turn_right(), next, &been) {
                        cycle_count += 1;
                    }
                }
                pos = next;
            }
        }
    }

}
fn rayon(input: &Grid2d<char>) -> usize {
    let start = input.find(|c| c == &'^').unwrap().0;
    let to_check = all_touched_cells(input);
    to_check.into_par_iter().filter(|x| {
        cycles(input, start, *x)
    }).count()

}

fn bruteforce(input: &Grid2d<char>) -> usize {
    let start = input.find(|c| c == &'^').unwrap().0;
    input.indexed_iter()
        .filter(|x| x.1 == &'.')
        .filter(|(p, _)| cycles(input, start, *p))
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
