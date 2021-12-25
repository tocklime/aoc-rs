use std::collections::HashMap;

use aoc_harness::*;
use utils::{aabb::Aabb, cartesian::Point, grid2d::Grid2d};

aoc_main!(2021 day 25, part1 [hashmap, grid] => 601, example part1 EG => 58);

const EG: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

fn step(
    input: &HashMap<Point<usize>, char>,
    bb: Aabb<usize>,
) -> (usize, HashMap<Point<usize>, char>) {
    let mut move_count = 0;
    let map1: HashMap<_, _> = input
        .iter()
        .map(|(&k, &v)| {
            let t = if bb.contains(&k.right()) {
                k.right()
            } else {
                Point::new(0, k.y)
            };
            if v == '>' {
                if !input.contains_key(&t) {
                    move_count += 1;
                    (t, v)
                } else {
                    (k, v)
                }
            } else {
                (k, v)
            }
        })
        .collect();
    let map2: HashMap<_, _> = map1
        .iter()
        .map(|(&k, &v)| {
            if v == 'v' {
                let t = if bb.contains(&k.up()) {
                    k.up()
                } else {
                    Point::new(k.x, 0)
                };
                if !map1.contains_key(&t) {
                    move_count += 1;
                    (t, v)
                } else {
                    (k, v)
                }
            } else {
                (k, v)
            }
        })
        .collect();
    assert_eq!(input.len(), map1.len());
    assert_eq!(map2.len(), map1.len());
    (move_count, map2)
}
fn hashmap(input: &str) -> usize {
    let map = utils::cartesian::as_point_map::<usize>(input, false);
    let mut map = map
        .into_iter()
        .filter_map(|(k, v)| match v {
            '>' => Some((k, '>')),
            'v' => Some((k, 'v')),
            _ => None,
        })
        .collect::<HashMap<_, _>>();
    let bb: Aabb<usize> = map.keys().collect();
    for i in 1.. {
        let (count, new_map) = step(&map, bb);
        map = new_map;
        if count == 0 {
            return i;
        }
    }
    0
}

type Grid = Grid2d<u8>;

fn step_grid(g: &mut Grid) -> usize {
    let mut intermediate = g.clone();
    let mut count = 0;
    for (p, &d) in g.indexed_iter() {
        if d == 1 {
            let tx = (p.1 + 1) % g.dim().1;
            if g[(p.0, tx)] == 0 {
                intermediate[p] = 3;
                intermediate[(p.0, tx)] = d;
                count += 1;
            }
        }
    }
    for (p, &d) in intermediate.indexed_iter() {
        if d == 1 {
            g[p] = d;
        } else if d == 2 {
            let ty = (p.0 + 1) % g.dim().0;
            match intermediate[(ty, p.1)] {
                0 | 3 => {
                    g[p] = 0;
                    g[(ty, p.1)] = d;
                    count += 1;
                }
                _ => {}
            }
        } else if d == 3 && g[p] == 1 {
            g[p] = 0;
        }
    }
    count
}
fn grid(input: &str) -> usize {
    let mut g: Grid = utils::grid2d::Grid2d::from_str(input, |x| match x {
        '>' => 1,
        'v' => 2,
        _ => 0,
    });
    for i in 1.. {
        if step_grid(&mut g) == 0 {
            return i;
        }
    }
    0
}
