use utils::points::{Dir, Point};
use aoc_harness::aoc_main;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryInto;
use std::hash::BuildHasher;
use utils::intcode::Computer;

aoc_main!(2019 day 15, generator explore, part1 [p1] => 230, part2 [p2] => 288);

const WALL: char = '█';
const SPACE: char = '.';
const EXPLORED_SPACE: char = ' ';
const OXYGEN: char = 'O';
const DEAD_END: char = 'D';
const BRANCH: char = '╳';
const START: char = 'S';

const RESPS: [char; 3] = [WALL, SPACE, OXYGEN];
pub fn try_move(c: &mut Computer<i32>, d: Dir) -> char {
    let i = match d {
        Dir::U => 1,
        Dir::D => 2,
        Dir::L => 3,
        Dir::R => 4,
    };
    c.with_input(i).run_to_input();
    let o = c.take_output();
    assert_eq!(o.len(), 1);
    let o_u: usize = o[0].try_into().unwrap();
    RESPS[o_u]
}

/// Performs a breadth first search of the map, and returns a map of point to distance from the start.
pub fn bfs_depth<S: BuildHasher>(
    map: &HashMap<Point, char, S>,
    start: Point,
) -> HashMap<Point, u32> {
    let mut points = std::collections::VecDeque::new();
    points.push_back((start, 0));
    let mut min_dist_map = HashMap::new();
    min_dist_map.insert(start, 0);
    while !points.is_empty() {
        let (pos, count) = points.pop_front().unwrap();
        Dir::all().iter().for_each(|d| {
            let p2 = pos + d.as_point_delta();
            if map.get(&p2) != Some(&WALL) && !min_dist_map.contains_key(&p2) {
                min_dist_map.insert(p2, count + 1);
                let t = (p2, count + 1);
                points.push_back(t);
            }
        });
    }
    min_dist_map
}

/// Returns a map of the explored area. chars used:
///  * '.': space, but not explored yet. Should not occur in output.
///  * ' ': space, after exploration.
///  * 'X': space, was a choice point.
///  * 'D': space, was a dead end.
///  * 'O': space, containing oxygen generator
///  * 'S': space, starting position
///  * '#': wall
//#[aoc_generator(day15)]
pub fn explore(input: &str) -> HashMap<Point, char> {
    let c = input.parse::<Computer>().unwrap();
    let position = Point(0, 0);
    let mut known_map: HashMap<Point, char> = [(position, START)].iter().cloned().collect();
    let mut save_points: Vec<(Point, Computer<i32>)> = vec![(position, c)];
    while !save_points.is_empty() {
        let (position, c) = save_points.pop().unwrap();
        //scan around in directions we don't know.
        let mut dirs: Vec<(Point, Computer)> = Dir::all()
            .iter()
            .filter_map(|&d| {
                let new_pos = position.step(d);
                match known_map.entry(new_pos) {
                    Entry::Occupied(_) => None,
                    Entry::Vacant(e) => {
                        let mut c2 = c.clone();
                        let ch = try_move(&mut c2, d);
                        e.insert(ch);
                        if ch == SPACE {
                            Some((new_pos, c2))
                        } else {
                            None
                        }
                    }
                }
            })
            .collect();
        //mark points of interest.
        match dirs.len() {
            0 => known_map.insert(position, DEAD_END),
            1 => known_map.insert(position, EXPLORED_SPACE),
            _ => known_map.insert(position, BRANCH),
        };
        save_points.append(&mut dirs);
    }
    known_map
}

//#[aoc(day15, part1)]
pub fn p1<S: BuildHasher>(input: &HashMap<Point, char, S>) -> u32 {
    let (o_pos, _) = input
        .iter()
        .find(|(_, &v)| v == OXYGEN)
        .expect("No oxygen!");
    bfs_depth(input, Point(0, 0))[o_pos]
}
//#[aoc(day15, part2)]
pub fn p2<S: BuildHasher>(input: &HashMap<Point, char, S>) -> u32 {
    let (&o_pos, _) = input
        .iter()
        .find(|(_, &v)| v == OXYGEN)
        .expect("No oxygen!");
    *bfs_depth(input, o_pos).values().max().unwrap()
}
