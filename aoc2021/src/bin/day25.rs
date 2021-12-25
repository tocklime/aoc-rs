use std::{collections::HashMap, str::FromStr};

use aoc_harness::*;
use utils::{
    aabb::Aabb,
    cartesian::{render_char_map_w, Point},
};

aoc_main!(2021 day 25, part1 [p1] => 601, example part1 EG => 58);

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
    let mut map1: HashMap<_, _> = input
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
fn p1(input: &str) -> usize {
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
        // println!("{} \n{}", i, render_char_map_w(&map, 1, ".", false));
        let (count, new_map) = step(&map, bb);
        map = new_map;
        if count == 0 {
            return i;
        }
    }
    0
}
