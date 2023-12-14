use ahash::HashMap;
use utils::grid2d::Grid2d;

aoc_harness::aoc_main!(2023 day 14, part1 [p1] => 111_339, part2 [p2_by_duplicate_state_detection, p2_by_full_cycle_detection] => 93736, example both EG => (136, 64));

fn roll_north(g: &mut Grid2d<char>) {
    let s = g.dim();
    for y in 0..s.y {
        for x in 0..s.x {
            if g[(y, x)] == 'O' {
                let new_y = (0..y)
                    .rev()
                    .find(|ny| g[(*ny, x)] != '.')
                    .map(|y| y + 1)
                    .unwrap_or(0);
                g[(y, x)] = '.';
                g[(new_y, x)] = 'O';
            }
        }
    }
}
fn roll_south(g: &mut Grid2d<char>) {
    let s = g.dim();
    for y in (0..s.y).rev() {
        for x in 0..s.x {
            if g[(y, x)] == 'O' {
                let new_y = (y + 1..s.y).find(|ny| g[(*ny, x)] != '.').unwrap_or(s.y) - 1;
                g[(y, x)] = '.';
                g[(new_y, x)] = 'O';
            }
        }
    }
}
fn roll_east(g: &mut Grid2d<char>) {
    let s = g.dim();
    for y in 0..s.y {
        for x in (0..s.x).rev() {
            if g[(y, x)] == 'O' {
                let new_x = (x + 1..s.x).find(|nx| g[(y, *nx)] != '.').unwrap_or(s.x) - 1;
                g[(y, x)] = '.';
                g[(y, new_x)] = 'O';
            }
        }
    }
}
fn roll_west(g: &mut Grid2d<char>) {
    let s = g.dim();
    for y in 0..s.y {
        for x in 1..s.x {
            if g[(y, x)] == 'O' {
                let new_x = (0..x)
                    .rev()
                    .find(|nx| g[(y, *nx)] != '.')
                    .map(|x| x + 1)
                    .unwrap_or(0);
                g[(y, x)] = '.';
                g[(y, new_x)] = 'O';
            }
        }
    }
}

fn calc_north_load(g: &Grid2d<char>) -> usize {
    let s = g.dim();
    g.indexed_iter()
        .map(|(p, c)| if c == &'O' { s.y - p.y } else { 0 })
        .sum()
}
fn p1(input: &str) -> usize {
    let mut g = Grid2d::from_str(input, |x| x);
    roll_north(&mut g);
    calc_north_load(&g)
}
fn cycle(g: &mut Grid2d<char>) {
    roll_north(g);
    roll_west(g);
    roll_south(g);
    roll_east(g);
}
const P2_ITER_COUNT: usize = 1_000_000_000;
fn p2_by_duplicate_state_detection(input: &str) -> usize {
    let mut g = Grid2d::from_str(input, |x| x);
    let mut seen = HashMap::default();
    for x in 1..=P2_ITER_COUNT {
        cycle(&mut g);
        if x > 100 {
            if let Some(old_count) = seen.get(&g) {
                // Cycle from {old_count} to {x}.
                let cycles_left_to_do = (P2_ITER_COUNT - x) % (x - old_count);
                for _ in 0..cycles_left_to_do {
                    cycle(&mut g);
                }
                return calc_north_load(&g);
            }
            seen.insert(g.clone(), x);
        }
    }
    unreachable!()
}
fn p2_by_full_cycle_detection(input: &str) -> usize {
    let mut g = Grid2d::from_str(input, |x| x);
    let mut seen = HashMap::default();
    let mut seen_vec = Vec::new();
    for x in 1..=P2_ITER_COUNT {
        cycle(&mut g);
        let load = calc_north_load(&g);
        seen_vec.push(load);
        if let Some(&old) = seen.get(&load) {
            let size = x - old;
            if size > 1 && old > size && seen_vec[old - size..old] == seen_vec[old..] {
                //cycle found!
                let cycles_left_to_do = (P2_ITER_COUNT - x) % size;
                return seen_vec[old + cycles_left_to_do - 1];
            }
        }
        seen.insert(load, x);
    }
    unreachable!()
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
