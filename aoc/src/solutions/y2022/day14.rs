use aoc_harness::*;
use nom::{
    bytes::complete::tag, character::complete, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult,
};
use utils::{aabb::Aabb, cartesian::Point, grid2d::Grid2d};
use std::string::ToString;

aoc_harness::aoc_main!(2022 day 14, generator gen, part1 [solve::<false>] => 757, part2 [solve::<true>] => 24943, example both EG => (24,93));

const EG: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

fn line(input: &str) -> IResult<&str, Vec<Point<usize>>> {
    separated_list1(
        tag(" -> "),
        map(
            separated_pair(complete::u32, tag(","), complete::u32),
            |(x, y)| Point::new(x as usize, y as usize),
        ),
    )(input)
}
fn gen(input: &str) -> Grid2d<char> {
    let p = input
        .lines()
        .map(|l| line(l).unwrap().1)
        .collect::<Vec<_>>();
    let bb: Aabb<usize> = p.iter().flatten().collect();
    let mut grid = Grid2d::from_elem((bb.top_right.y + 2, 1000), '.');
    for l in p {
        for (a, b) in l.iter().tuple_windows() {
            for p in a.steps_to(*b, true) {
                grid[p] = '#';
            }
        }
    }
    grid
}

fn next_sand(
    grid: &Grid2d<char>,
    path: &mut Vec<Point<usize>>,
    stop_on_bottom: bool,
) -> Option<Point<usize>> {
    while let Some(&p) = path.last() {
        if grid[p] != '.' {
            //fill point blocked!
            path.pop();
            continue;
        }
        if p.y + 1 >= grid.dim().0 {
            //this is the bottom. stop here.
            if stop_on_bottom {
                return Some(p);
            } else {
                return None;
            }
        }
        let next = p.up();
        match [next, next.left(), next.right()]
            .into_iter()
            .find(|p| grid[*p] == '.')
        {
            Some(p) => {
                path.push(p);
            }
            None => {
                path.pop();
                return Some(p);
            }
        }
    }
    None
}
#[allow(dead_code)]
fn draw_grid(grid: &Grid2d<char>, mem: &[Point<usize>]) {
    let mut mine = grid.clone();
    for p in mem {
        mine[*p] = '~';
    }
    let bb = mine.find_bb(|c| c != &'.');
    println!("{}", mine.render_section_with(bb, ToString::to_string));
}

fn solve<const STOP_ON_BOTTOM: bool>(input: &Grid2d<char>) -> usize {
    let mut grid = input.clone();
    let fill_point = Point::new(500, 0);
    let mut memory = vec![fill_point];

    std::iter::from_fn(|| next_sand(&grid, &mut memory, STOP_ON_BOTTOM).map(|p| grid[p] = 'o')).count()
}
