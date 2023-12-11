use itertools::Itertools;
use utils::grid2d::{Coord, Grid2d};

aoc_harness::aoc_main!(2023 day 11, part1 [solve::<2>, solve_by_dimensions::<2>] => 9_623_138, 
    part2 [solve::<1_000_000>, solve_by_dimensions::<1_000_000>] => 726_820_169_514, example part1 EG => 374);

#[inline]
fn quick_mul<F: Fn() -> usize>(a: usize, b: F) -> usize {
    if a == 0 {
        0
    } else {
        a * b()
    }
}
fn solve_1_dim<const N: usize>(input: &[usize]) -> usize {
    //we have a list of numbers where each num is the count of galaxies in that cell.
    //want the sum of cartesian product distances.
    let dists = (0..input.len())
        .scan(0, |d, x| {
            *d += if input[x] == 0 { N } else { 1 };
            Some(*d)
        })
        .collect_vec();
    (0..input.len())
        .map(|a| {
            quick_mul(input[a], || {
                (0..a)
                    .map(|b| quick_mul(input[b], || dists[a] - dists[b]))
                    .sum()
            })
        })
        .sum()
}

fn solve_by_dimensions<const N: usize>(input: &str) -> usize {
    let map = Grid2d::from_str(input, |x| x);
    let s = map.dim();
    let cols = (0..s.x)
        .map(|x| (0..s.y).filter(|&y| map[(y, x)] == '#').count())
        .collect_vec();
    let rows = (0..s.y)
        .map(|y| (0..s.x).filter(|&x| map[(y, x)] == '#').count())
        .collect_vec();
    solve_1_dim::<N>(&cols) + solve_1_dim::<N>(&rows)
}

fn solve<const N: usize>(input: &str) -> usize {
    let map = Grid2d::from_str(input, |x| x);
    let s = map.dim();
    let x_dists = (0..s.x)
        .scan(0, |d, x| {
            *d += if (0..s.y).all(|y| map[(y, x)] != '#') {
                N
            } else {
                1
            };
            Some(*d)
        })
        .collect_vec();
    let y_dists = (0..s.y)
        .scan(0, |d, y| {
            *d += if (0..s.x).all(|x| map[(y, x)] != '#') {
                N
            } else {
                1
            };
            Some(*d)
        })
        .collect_vec();
    let stars: Vec<Coord> = map
        .indexed_iter()
        .filter(|x| x.1 == &'#')
        .map(|x| x.0)
        .collect();
    stars
        .iter()
        .enumerate()
        .map(|(a_ix, a)| {
            stars
                .iter()
                .skip(a_ix)
                .map(|b| {
                    (x_dists[a.x.max(b.x)] - x_dists[a.x.min(b.x)])
                        + (y_dists[a.y.max(b.y)] - y_dists[a.y.min(b.y)])
                })
                .sum::<usize>()
        })
        .sum()
}

const EG: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

//wrong: 726820896326
#[cfg(test)]
mod test {
    #[test]
    fn tests() {
        assert_eq!(super::solve::<10>(super::EG), 1030);
        assert_eq!(super::solve::<100>(super::EG), 8410);
    }
}
