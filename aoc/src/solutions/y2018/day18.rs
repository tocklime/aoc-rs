use std::collections::HashMap;

use aoc_harness::{aoc_main, Itertools};
use ndarray::{Array2, IntoDimension};
use utils::cartesian::Point;

aoc_main!(2018 day 18, generator gen_grid,
    example part1 EG1 => 1147,
    part1 [p1] => 519_552,
    part2 [p2] => 165_376
);

const TREES: char = '|';
const LUMBERYARD: char = '#';
const OPEN: char = '.';

fn gen_grid(input: &str) -> Array2<char> {
    let cols = input.lines().next().unwrap().len();
    let v = input.lines().flat_map(str::chars).collect_vec();
    Array2::from_shape_vec((v.len() / cols, cols), v).unwrap()
}
fn step_world(input: &Array2<char>) -> Array2<char> {
    let mut counts = Array2::from_elem(input.dim(), (0, 0));
    for (d, ch) in input.indexed_iter() {
        if *ch == TREES || *ch == LUMBERYARD {
            for n in Point::from_dim(d).neighbours_with_diagonals() {
                match (*ch, counts.get_mut(n.into_dimension())) {
                    (TREES, Some(x)) => {
                        x.0 += 1;
                    }
                    (LUMBERYARD, Some(x)) => {
                        x.1 += 1;
                    }
                    _ => (),
                }
            }
        }
    }
    Array2::from_shape_fn(input.dim(), |p| match (input[p], counts[p]) {
        (OPEN, (tr, _)) => {
            if tr >= 3 {
                TREES
            } else {
                OPEN
            }
        }
        (TREES, (_, lu)) => {
            if lu >= 3 {
                LUMBERYARD
            } else {
                TREES
            }
        }
        (LUMBERYARD, (tr, lu)) => {
            if tr >= 1 && lu >= 1 {
                LUMBERYARD
            } else {
                OPEN
            }
        }
        _ => panic!("Unknown character in world"),
    })
}

fn score_world(input: &Array2<char>) -> usize {
    let mut tree_count = 0;
    let mut lumber_count = 0;
    for n in input {
        match *n {
            TREES => tree_count += 1,
            LUMBERYARD => lumber_count += 1,
            _ => (),
        }
    }
    tree_count * lumber_count
}

fn p1(input: &Array2<char>) -> usize {
    fixed_count(input, 10)
}
fn fixed_count(input: &Array2<char>, n: usize) -> usize {
    score_world(&(0..n).fold(input.clone(), |a, _| step_world(&a)))
}
fn p2(input: &Array2<char>) -> usize {
    let mut g = input.clone();
    let mut seen_at_ix = HashMap::new();
    let mut ordered = vec![g.clone()];
    seen_at_ix.insert(g.clone(), 0);
    let (prelude_len, loop_len) = loop {
        g = step_world(&g);
        let ix = ordered.len();
        match seen_at_ix.insert(g.clone(), ix) {
            Some(old_ix) => break (old_ix, (ix - old_ix)),
            None => ordered.push(g.clone()),
        }
    };
    let remaining_after_prelude = 1_000_000_000 - prelude_len;
    let remaining_after_loops = remaining_after_prelude % loop_len;

    score_world(&ordered[prelude_len + remaining_after_loops])
    // fixed_count(input, prelude_len + remaining_after_loops)
}

const EG1: &str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
