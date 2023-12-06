use aoc_harness::{Itertools};
use std::{collections::HashSet, ops::RangeInclusive};
use utils::grid2d::{Coord, Grid2d};

aoc_harness::aoc_main!(2018 day 17, generator gen_grid,
    example both EG2 => (56, 28),
    example both EG1 => (57, 29),
    both [both] => (33052, 27068)
);
const EG1: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
";
const EG2: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
x=501, y=11
";
#[derive(Debug, PartialEq, Eq)]
struct SingleOrRange(RangeInclusive<usize>);
impl std::str::FromStr for SingleOrRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SingleOrRange(
            if let Some((from, to)) = s.split_once("..") {
                from.parse().unwrap()..=to.parse().unwrap()
            } else {
                let p: usize = s.parse().unwrap();
                p..=p
            },
        ))
    }
}
#[derive(PartialEq, Eq, Debug)]
struct Line {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}
impl std::str::FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(", ").ok_or(())?;
        let a_val: SingleOrRange = a[2..].parse()?;
        let b_val: SingleOrRange = b[2..].parse()?;
        Ok(if a.starts_with('x') {
            Self {
                x: a_val.0,
                y: b_val.0,
            }
        } else {
            Self {
                x: b_val.0,
                y: a_val.0,
            }
        })
    }
}

fn gen_grid(input: &str) -> Grid2d<char> {
    let parsed: Vec<Line> = input.lines().map(|l| l.parse().unwrap()).collect();
    let (&min_x, &max_x) = parsed
        .iter()
        .flat_map(|l| [l.x.start(), l.x.end()])
        .minmax()
        .into_option()
        .unwrap();
    let (&min_y, &max_y) = parsed
        .iter()
        .flat_map(|l| [l.y.start(), l.y.end()])
        .minmax()
        .into_option()
        .unwrap();
    let width = max_x - min_x + 3;
    let mut grid: Grid2d<char> = Grid2d::from_elem((max_y - min_y + 1, width), '.');

    for l in parsed {
        for x in l.x.clone() {
            for y in l.y.clone() {
                grid[(y - min_y, x - min_x + 1)] = '#';
            }
        }
    }
    grid[(0, 500 - min_x + 1)] = '+';

    grid
}
#[derive(Debug)]
enum FillResult {
    KeepFillingHere,
    NowFillFrom(Vec<Coord>),
    DoneFilling,
}
fn is_solid(c: char) -> bool {
    c == '~' || c == '#'
}
fn fill_from(grid: &mut Grid2d<char>, pos: Coord) -> FillResult {
    //straight down until you hit a non-'.'.
    let mut new_falls = vec![];
    if grid[pos] == '~' {
        return FillResult::DoneFilling; //(Fill point is flooded)
    }
    let Some(bottom_of_fall) = (pos.0..grid.dim().0).find(|y| is_solid(grid[(*y, pos.1)])).map(|y| y - 1) else {
        for y in pos.0..grid.dim().0 {
            grid[(y, pos.1)] = '|';
        }
        return FillResult::DoneFilling; //(Fill point goes off bottom of map)
    };
    for y in pos.0..=bottom_of_fall {
        grid[(y, pos.1)] = '|';
    }

    let left_of_fall = (0..pos.1)
        .rev()
        .find(|x| is_solid(grid[(bottom_of_fall, *x)]));
    let right_of_fall = (pos.1..grid.dim().1).find(|x| is_solid(grid[(bottom_of_fall, *x)]));
    let left_stop = left_of_fall.unwrap_or(0);
    let right_stop = right_of_fall.unwrap_or(grid.dim().1);
    let hole_to_left = (left_stop..pos.1)
        .rev()
        .find(|x| !is_solid(grid[(bottom_of_fall + 1, *x)]));
    let hole_to_right = (pos.1..right_stop).find(|x| !is_solid(grid[(bottom_of_fall + 1, *x)]));
    if let (Some(l), Some(r), None, None) =
        (left_of_fall, right_of_fall, hole_to_left, hole_to_right)
    {
        //enclosed with no holes. fill it up.
        for x in l + 1..r {
            grid[(bottom_of_fall, x)] = '~';
        }
        FillResult::KeepFillingHere
    } else {
        //at least one hole.
        let fill_from_l = [hole_to_left, left_of_fall]
            .into_iter()
            .flatten()
            .max()
            .unwrap_or(pos.1);
        let fill_from_r = [hole_to_right, right_of_fall]
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(pos.1);
        for x in fill_from_l + 1..fill_from_r {
            grid[(bottom_of_fall, x)] = '|';
        }
        if let Some(x) = hole_to_left {
            new_falls.push((bottom_of_fall, x));
        }
        if let Some(x) = hole_to_right {
            new_falls.push((bottom_of_fall, x));
        }
        FillResult::NowFillFrom(new_falls)
    }
}
fn both(input: &Grid2d<char>) -> (usize, usize) {
    let start_pos = input.indexed_iter().find(|c| *c.1 == '+').unwrap().0;
    let mut grid = input.clone();
    let mut fills = vec![start_pos];
    let mut done_filling_points = HashSet::new();
    while let Some(next) = fills.last() {
        match fill_from(&mut grid, *next) {
            FillResult::KeepFillingHere => {}
            FillResult::NowFillFrom(v) => {
                let interesting = v
                    .into_iter()
                    .filter(|p| !done_filling_points.contains(p))
                    .collect_vec();
                if interesting.is_empty() {
                    //all the new points are places we have decided don't go anywhere,
                    //so, this doesn't go anywhere either.
                    done_filling_points.insert(fills.pop().unwrap());
                } else {
                    fills.extend(interesting);
                }
            }
            FillResult::DoneFilling => {
                done_filling_points.insert(fills.pop().unwrap());
            }
        }
    }
    let flowing = grid.iter().filter(|c| **c == '|').count();
    let stable = grid.iter().filter(|c| **c == '~').count();
    (flowing + stable, stable)
}
