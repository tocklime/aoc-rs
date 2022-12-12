use std::rc::Rc;

use aoc_harness::*;
use pathfinding::directed::dijkstra;
use utils::grid2d::Grid2d;

aoc_main!(2022 day 12, part1 [p1] => 412, part2 [p2] => 402, example both EG => (31,29));

const EG : &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

fn p1(input: &str) -> usize {
    let mut grid = Grid2d::from_str(input, |c| c as u8);
    println!("{}", grid);
    let start_pos = grid.indexed_iter().find(|&(_, &c) | c == b'S').unwrap().0;
    let end_pos = grid.indexed_iter().find(|&(_, &c)| c == b'E').unwrap().0;
    grid[start_pos] = b'a';
    grid[end_pos] = b'z';
    dbg!(start_pos, end_pos);
    let grid_ref = Rc::new(grid);
    let gr2 = Rc::clone(&grid_ref);
    let neighbours = |p: &(usize,usize)| {
        let ans = gr2.neighbours(*p).filter_map(|n| {
            let my_height = &gr2[*p];
            let new_height = &gr2[n];
            if *my_height + 1 >= *new_height {
                Some((n,1))
            } else {
                None
            }
        }).collect::<Vec<_>>();
        ans
    };
    let a = dijkstra::dijkstra(&start_pos, neighbours,
        |p| *p == end_pos);
    a.unwrap().1
}

fn shortest_from(start_pos: (usize,usize), end_pos: (usize,usize), grid: &Grid2d<u8>) -> Option<usize> {
    let grid_ref = Rc::new(grid);
    let gr2 = Rc::clone(&grid_ref);
    let neighbours = |p: &(usize,usize)| {
        let ans = gr2.neighbours(*p).filter_map(|n| {
            let my_height = &gr2[*p];
            let new_height = &gr2[n];
            if *my_height + 1 >= *new_height {
                Some((n,1))
            } else {
                None
            }
        }).collect::<Vec<_>>();
        ans
    };
    let a = dijkstra::dijkstra(&start_pos, neighbours,
        |p| *p == end_pos);
    Some(a?.1)
}
fn p2(input: &str) -> usize {
    let mut grid = Grid2d::from_str(input, |c| c as u8);
    println!("{}", grid);
    let start_pos = grid.indexed_iter().find(|&(_, &c) | c == b'S').unwrap().0;
    let end_pos = grid.indexed_iter().find(|&(_, &c) | c == b'E').unwrap().0;
    grid[start_pos] = b'a';
    grid[end_pos] = b'z';
    let all_start_pos = grid.indexed_iter().filter(|&(_, &c) | c == b'a');
    all_start_pos.filter_map(|x| shortest_from(x.0, end_pos, &grid)).min().unwrap()

}