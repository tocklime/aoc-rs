use std::collections::{hash_map::Entry, HashMap};

use ahash::HashSet;
use utils::{
    cartesian::render_char_map_w,
    grid2d::{Coord, Grid2d},
    points::render_char_map,
};

aoc_harness::aoc_main!(2023 day 10, part1 [p1], part2 [p2], 
    example both EG => (4,1), example both EG2 => (8,1), 
    example part2 EG3 => 8, example part2 EG4 => 10);

fn p1(input: &str) -> usize {
    let map = Grid2d::from_str(input, |x| x);
    let s = map.indexed_iter().find(|x| x.1 == &'S').unwrap().0;
    let mut seen = Grid2d::from_elem(map.dim(), None);
    let mut next = vec![(s, 0usize)];
    while !next.is_empty() {
        let mut new_next = vec![];
        for (n, dist) in next {
            if seen.get(n).unwrap().is_none() {
                seen[n] = Some(dist);
                let here = *map.get(n).unwrap();
                let [north, west, south, east] = map.neighbours_array_ordered(n);
                // dbg!(n, north, east, south, west,here);
                if "S|JL".contains(here) {
                    if let Some(x) = north {
                        if let Some('7') | Some('F') | Some('|') = map.get(x) {
                            new_next.push((x, dist + 1));
                        }
                    }
                }
                if "S-FL".contains(here) {
                    if let Some(x) = east {
                        if let Some('7') | Some('J') | Some('-') = map.get(x) {
                            new_next.push((x, dist + 1));
                        }
                    }
                }
                if "S|F7".contains(here) {
                    if let Some(x) = south {
                        if let Some('L') | Some('J') | Some('|') = map.get(x) {
                            new_next.push((x, dist + 1));
                        }
                    }
                }
                if "S-J7".contains(here) {
                    if let Some(x) = west {
                        if let Some('L') | Some('F') | Some('-') = map.get(x) {
                            new_next.push((x, dist + 1));
                        }
                    }
                }
            }
        }
        next = new_next;
    }
    seen.iter().filter_map(|x| *x).max().unwrap()
}
fn p2(input: &str) -> usize {
    let mut map = Grid2d::from_str(input, |x| x);
    let s = map.indexed_iter().find(|x| x.1 == &'S').unwrap().0;

    let can_north = "|F7".contains(*map.get((s.0.wrapping_sub(1),s.1)).unwrap_or(&' '));
    let can_south = "|JL".contains(*map.get((s.0+1,s.1)).unwrap_or(&' '));
    let can_east = "-J7".contains(*map.get((s.0,s.1+1)).unwrap_or(&' '));
    let can_west = "-FL".contains(*map.get((s.0,s.1.wrapping_sub(1))).unwrap_or(&' '));
    println!("Before:\n{map}");
    map[s] = match (can_north, can_east, can_south, can_west) {
        (true, true, false, false) => 'L',
        (true, false, true, false) => '|',
        (true, false, false, true) => 'J',
        (false, true, true, false) => 'F',
        (false, true, false, true) => '-',
        (false, false, true, true) => '7',
        _ => panic!("{map}\n{s:?}")
    };
    println!("After:\n{map}");

    let mut seen = Grid2d::from_elem(map.dim(), '.');
    let mut next = vec![s];
    while !next.is_empty() {
        let mut new_next = vec![];
        for n in next {
            if seen.get(n).unwrap() == &'.'{
                let here = *map.get(n).unwrap();
                seen[n] = here;
                let [north, west, south, east] = map.neighbours_array_ordered(n);
                // dbg!(n, north, east, south, west,here);
                if "S|JL".contains(here) {
                    if let Some(x) = north {
                        if let Some('7') | Some('F') | Some('|') = map.get(x) {
                            new_next.push(x);
                        }
                    }
                }
                if "S-FL".contains(here) {
                    if let Some(x) = east {
                        if let Some('7') | Some('J') | Some('-') = map.get(x) {
                            new_next.push(x);
                        }
                    }
                }
                if "S|F7".contains(here) {
                    if let Some(x) = south {
                        if let Some('L') | Some('J') | Some('|') = map.get(x) {
                            new_next.push(x);
                        }
                    }
                }
                if "S-J7".contains(here) {
                    if let Some(x) = west {
                        if let Some('L') | Some('F') | Some('-') = map.get(x) {
                            new_next.push(x);
                        }
                    }
                }
            }
        }
        next = new_next;
    }
    println!("{map}\n{seen}");
    let mut state = TraceState::Outside;
    for (p,c) in seen.indexed_iter_mut() {
        if p.1 == 0 {
            state = TraceState::Outside;
        }
        match (*c, state) {
            ('.', TraceState::Inside) => *c = 'I',
            ('.', TraceState::Outside) => *c = 'O',
            ('|', TraceState::Inside) => state = TraceState::Outside,
            ('|', TraceState::Outside) => state = TraceState::Inside,
            ('F', TraceState::Outside) => state = TraceState::OnPipeInBelow,
            ('F', TraceState::Inside) => state = TraceState::OnPipeInAbove,
            ('L', TraceState::Outside) => state = TraceState::OnPipeInAbove,
            ('L', TraceState::Inside) => state = TraceState::OnPipeInBelow,
            ('7', TraceState::OnPipeInAbove) => state = TraceState::Inside,
            ('7', TraceState::OnPipeInBelow) => state = TraceState::Outside,
            ('J', TraceState::OnPipeInAbove) => state = TraceState::Outside,
            ('J', TraceState::OnPipeInBelow) => state = TraceState::Inside,
            // _ => panic!("{c}, {state:?} ???")
            _ => {}
        }
    }
    println!("{seen}");
    seen.iter().filter(|x| x == &&'I').count()
}

#[derive(Debug, Copy, Clone)]
enum TraceState {
    Inside,OnPipeInBelow,OnPipeInAbove, Outside
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
//too high: 469, 463, 462