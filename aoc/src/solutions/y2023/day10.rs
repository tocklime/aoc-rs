use std::option::Option;

use itertools::Itertools;
use utils::{
    cartesian::Point,
    grid2d::{Grid2d, ICoord},
};

aoc_harness::aoc_main!(2023 day 10, both [both] => (6909, 461),
    example both EG => (4,1), example both EG2 => (8,1),
    example part2 EG3 => 8, example part2 EG4 => 10);

const NORTH: ICoord = Point::new(0, -1);
const SOUTH: ICoord = Point::new(0, 1);
const EAST: ICoord = Point::new(1, 0);
const WEST: ICoord = Point::new(-1, 0);

fn map_char(c: char) -> impl Iterator<Item = ICoord> {
    match c {
        '|' => Some([NORTH, SOUTH]),
        '-' => Some([EAST, WEST]),
        'F' => Some([SOUTH, EAST]),
        'J' => Some([NORTH, WEST]),
        'L' => Some([NORTH, EAST]),
        '7' => Some([SOUTH, WEST]),
        _ => None,
    }
    .into_iter()
    .flatten()
}
const CHARS: &str = "|-FJL7";

fn both(input: &str) -> (usize, usize) {
    let mut answer = (0, 0);
    let mut map = Grid2d::from_str(input, |x| x);
    let s = map.indexed_iter().find(|x| x.1 == &'S').unwrap().0;

    // Figure out what is really under S by examining neighbours.
    map[s] = CHARS
        .chars()
        .find(|&c| {
            map_char(c).all(|n| {
                map.relative_lookup(s, n)
                    .copied()
                    .map(|x| map_char(x).contains(&-n))
                    .unwrap_or_default()
            })
        })
        .unwrap();

    // seen is a grid of distances from start along the pipes (to visualise the 'true' pipe)
    let mut seen = Grid2d::from_elem(map.dim(), None);
    // next is a list of (up to 2) nodes to explore next (must be BFS).
    let mut next = vec![(s, 0usize)];
    let mut new_next = vec![];
    while !next.is_empty() {
        new_next.clear();
        for &(n, dist) in &next {
            seen[n] = Some(dist);
            answer.0 = answer.0.max(dist);
            let here = *map.get(n).unwrap();
            let x = map_char(here)
                .map(move |rel| n + rel)
                .filter(|&abs| seen.get(abs).map(Option::is_none).unwrap_or_default())
                .map(move |abs| (abs, dist + 1));
            new_next.extend(x);
        }
        std::mem::swap(&mut next, &mut new_next);
    }
    let mut clean_map =
        Grid2d::from_fn(map.dim(), |p| if seen[p].is_some() { map[p] } else { '.' });
    let mut inside = false;
    for c in clean_map.iter_mut() {
        match *c {
            '|' | 'J' | 'L' => inside = !inside,
            '.' if inside => {
                answer.1 += 1;
                *c = 'I';
            }
            '.' if !inside => *c = 'O',
            _ => (),
        }
    }
    answer
}

const EG: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
const EG2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
const EG3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
const EG4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
