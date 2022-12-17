use std::cmp::max;

use aoc_harness::*;
use utils::{grid2d::Grid2d, numset::NumSet};

aoc_main!(2022 day 17, part1 [p1::<2022>] => 3085, part2 [p1::<1000000000000>] => 1535483870924, example both EG => (3068,1514285714288));

const EG: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
const ROCKS: [&str; 5] = [
    "####",
    ".#.\n###\n.#.",
    "..#\n..#\n###",
    "#\n#\n#\n#",
    "##\n##",
];
//index 0 in grid is the ground.
fn place_rock(
    grid: &mut Vec<NumSet<u8>>,
    r: &[NumSet<u8>],
    winds: &mut impl Iterator<Item = char>,
) -> usize {
    let mut height = grid.len() + 3;
    let mut left = 2;
    // draw(grid, r, left, height);
    loop {
        let go_left = winds.next().unwrap() == '<';
        if !go_left || left > 0 {
            let new_left = if go_left { left - 1 } else { left + 1 };
            if !would_collide(grid, r, new_left, height) {
                left = new_left;
            }
        }
        if height == 0 {
            break;
        }
        let new_height = height - 1;
        if would_collide(grid, r, left, new_height) {
            break;
        }
        height = new_height;
    }
    set_piece(grid, r, left, height)
}
fn set_piece(grid: &mut Vec<NumSet<u8>>, r: &[NumSet<u8>], left: usize, height: usize) -> usize {
    let new_rows = (height + r.len()).saturating_sub(grid.len());
    grid.extend((0..new_rows).map(|_| NumSet::new()));
    for (ix, l) in r.iter().enumerate() {
        for set_bit in l.iter() {
            let c = (height + ix, left as u8 + set_bit);
            assert!(c.1 < 7);
            assert!(grid[c.0].insert(c.1));
        }
    }
    new_rows
}
fn would_collide(grid: &[NumSet<u8>], r: &[NumSet<u8>], left: usize, height: usize) -> bool {
    for (ix, l) in r.iter().enumerate() {
        for set_bit in l.iter() {
            let c = (height + ix, left as u8 + set_bit);
            if c.1 > 6 || grid.get(c.0).map(|n| n.contains(c.1)).unwrap_or(false) {
                return true;
            }
        }
    }
    false
}
#[allow(dead_code)]
fn draw_grid(grid: &[NumSet<u8>]) {
    let h = grid.len();
    let mut d = Grid2d::from_elem((h, 7), '.');
    for (ix, l) in grid.iter().enumerate() {
        for set_bit in l.iter() {
            d[(ix, set_bit as usize)] = '#';
        }
    }
    println!("{}", d);
}
#[allow(dead_code)]
fn draw(grid: &[NumSet<u8>], falling_rock: &Vec<NumSet<u8>>, left: usize, height: usize) {
    let h = height + falling_rock.len();
    let mut d = Grid2d::from_elem((max(grid.len(), h), 7), '.');
    for (ix, l) in grid.iter().enumerate() {
        for set_bit in l.iter() {
            d[(h - ix, set_bit as usize)] = '#';
        }
    }
    for (ix, l) in falling_rock.iter().enumerate() {
        for set_bit in l.iter() {
            let c = (h - (height + ix), left + (set_bit as usize));
            d[c] = match d[c] {
                '#' => 'X',
                _ => '@',
            };
        }
    }
    println!("{}", d);
}

fn find_cycle_on_end<T: PartialEq>(list: &[T]) -> Option<usize> {
    let match_size = 20;

    if list.len() > match_size + 1 {
        for ix1 in 0..list.len() - match_size - 1 {
            let sec1 = &list[ix1..=ix1 + match_size];
            let sec2 = &list[list.len() - match_size - 1..];
            if sec1 == sec2 {
                return Some((list.len() - match_size - 1) - ix1);
            }
        }
    }
    None
}
fn get_rocks() -> Vec<Vec<NumSet<u8>>> {
    ROCKS
        .iter()
        .map(|r| {
            r.lines()
                .map(|l| {
                    l.chars()
                        .enumerate()
                        .filter(|&(_, x)| x == '#')
                        .map(|(a, _)| (a) as u8)
                        .collect::<NumSet<u8>>()
                })
                .rev()
                .collect()
        })
        .collect()
}
fn p1<const ROCK_COUNT: usize>(input: &str) -> usize {
    let mut grid = Vec::new();
    let mut iter = input.trim().chars().cycle();
    let rock_conv = get_rocks();

    let mut height_deltas = Vec::new();

    for (ix, r) in rock_conv.iter().cycle().enumerate() {
        let new_h_delta = place_rock(&mut grid, r, &mut iter);
        height_deltas.push(new_h_delta);
        if ix + 1 == ROCK_COUNT {
            return grid.len();
        }
        if ix > 100 {
            if let Some(cycle_len) = find_cycle_on_end(&height_deltas) {
                // println!(
                //     "After {} rocks, height is {}, and delta is {}",
                //     ix,
                //     grid.len(),
                //     cycle_len
                // );
                let dropped_rocks = ix + 1;
                let left_to_drop = ROCK_COUNT - dropped_rocks;
                let cycle = &height_deltas[height_deltas.len() - cycle_len..];
                let height_per_cycle: usize = cycle.iter().sum();
                let complete_cycles_to_do = left_to_drop / cycle_len;
                let extra_bits = left_to_drop % cycle_len;
                let height_on_the_end = cycle.iter().take(extra_bits).sum::<usize>();
                return grid.len() + complete_cycles_to_do * height_per_cycle + height_on_the_end;
            }
        }
    }
    // draw_grid(&grid);
    //need to find a repetition sequence. there's one in example starting around ix 26 and 79.
    //so, delta height per input cycle is 53.
    //what is delta rock count?

    unreachable!()
}
