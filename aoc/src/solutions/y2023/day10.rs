use itertools::Itertools;
use utils::grid2d::{Grid2d, ICoord};

aoc_harness::aoc_main!(2023 day 10, both [both] => (6909, 461),
    example both EG => (4,1), example both EG2 => (8,1), 
    example part2 EG3 => 8, example part2 EG4 => 10);

fn map_char(c: char) -> Option<[ICoord; 2]> {
    match c {
        '|' => Some([(-1, 0), (1, 0)].map(Into::into)),
        '-' => Some([(0, -1), (0, 1)].map(Into::into)),
        'F' => Some([(1, 0), (0, 1)].map(Into::into)),
        'J' => Some([(-1, 0), (0, -1)].map(Into::into)),
        'L' => Some([(-1, 0), (0, 1)].map(Into::into)),
        '7' => Some([(1, 0), (0, -1)].map(Into::into)),
        _ => None,
    }
}
const CHARS: &str = "|-FJL7";

fn both(input: &str) -> (usize, usize) {
    let mut map = Grid2d::from_str(input, |x| x);
    // println!("Map:\n{map}");
    let s = map.indexed_iter().find(|x| x.1 == &'S').unwrap().0;

    // Figure out what is really under S by examining neighbours.
    map[s] = CHARS
        .chars()
        .find(|&c| {
            map_char(c).unwrap().into_iter().all(|n| {
                map.relative_lookup(s, n)
                    .copied()
                    .map(map_char)
                    .into_iter()
                    .flatten()
                    .flatten()
                    .contains(&-n)
            })
        })
        .unwrap();

    let mut seen = Grid2d::from_elem(map.dim(), None);
    let mut next = vec![(s, 0usize)];
    while !next.is_empty() {
        let mut new_next = vec![];
        for (n, dist) in next {
            if seen.get(n).unwrap().is_none() {
                seen[n] = Some(dist);
                let here = *map.get(n).unwrap();
                let [north, west, south, east] = map.neighbours_array_ordered(n);
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
    let p1 = seen.iter().filter_map(|x| *x).max().unwrap();
    let mut clean_map =
        Grid2d::from_fn(map.dim(), |p| if seen[p].is_some() { map[p] } else { '.' });
    let mut state = TraceState::Outside;
    for (p, c) in clean_map.indexed_iter_mut() {
        if p.x == 0 {
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
            _ => {}
        }
    }
    let p2 = clean_map.iter().filter(|x| x == &&'I').count();
    // println!("{clean_map} contains {p2} 'I's");
    (p1, p2)
}

#[derive(Debug, Copy, Clone)]
enum TraceState {
    Inside,
    OnPipeInBelow,
    OnPipeInAbove,
    Outside,
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
