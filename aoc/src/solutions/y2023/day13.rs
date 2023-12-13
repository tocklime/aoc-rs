use std::collections::HashSet;

use itertools::Itertools;
use utils::grid2d::Grid2d;

aoc_harness::aoc_main!(2023 day 13, part1 [p1::<0>], part2[p1::<1>], example both EG => (405,400));


fn try_split_at(grid: &Grid2d<char>, row_split: usize) -> usize {
    let gs = grid.dim();
    let y_height = row_split.min(gs.y - row_split);
    let size = (y_height, gs.x);
    (0..y_height).cartesian_product(0..gs.x).filter(|&(y,x)| {
        grid[(row_split+y,x)] != grid[(row_split-(y+1),x)]
    }).count()
}

fn p1<const ERROR_COUNT: usize>(input: &str) -> usize {
    input.split("\n\n").map(|l| {
        let g = Grid2d::from_str(l, |a| a);
        let s = g.dim();
        let g2 = Grid2d::from_fn((s.x,s.y), |x| g[(x.x,x.y)]);

        if let Some(y)= (1..s.y).find(|&x| try_split_at(&g, x) == ERROR_COUNT) {
            return y * 100;
        }
        if let Some(x) = (1..s.x).find(|&x| try_split_at(&g2, x) == ERROR_COUNT) {
            return x;
        }
        

        // let y_mirrors = (0..s.y).map(|y| {
        //     //try to flip all cols after row y...
        //     let line = (0..s.x).map(|x| g[(y,x)]).collect_vec();

        //     (1..s.x).filter(|&x| {
        //         line[0..x].iter().rev().zip(line[x..].iter()).all(|(a,b)| a == b)
        //     }).collect::<HashSet<usize>>()
        // }).tree_fold1(|a,b| a.intersection(&b).copied().collect::<HashSet<usize>>()).unwrap();
        // assert!(y_mirrors.len() < 2);
        // if y_mirrors.len() == 1 {
        //     return *y_mirrors.iter().next().unwrap();
        // }
        // let x_mirrors = (0..s.x).map(|x| {
        //     //try to flip all cols after row y...
        //     let line = (0..s.y).map(|y| g[(y,x)]).collect_vec();

        //     println!("l x{x}: {line:?}");
        //     (1..s.y).filter(|&y| {
        //         line[0..y].iter().rev().zip(line[y..].iter()).all(|(a,b)| a == b)
        //     }).collect::<HashSet<usize>>()
        // }).tree_fold1(|a,b| a.intersection(&b).copied().collect::<HashSet<usize>>()).unwrap();
        // assert!(x_mirrors.len() < 2, "{x_mirrors:?}");
        // if x_mirrors.len() == 1 {
        //     return *x_mirrors.iter().next().unwrap() * 100;
        // }
        panic!("Can't find symmetry in {g}");
        0
    }).sum()
}

const EG: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";