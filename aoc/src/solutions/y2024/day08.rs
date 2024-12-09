use ahash::HashMap;
use itertools::Itertools;
use utils::{cartesian::Point, grid2d::Grid2d};

aoc_harness::aoc_main!(2024 day 8, part1 [solve::<1>] => 348, part2 [solve::<2>] => 1221, example both EG => (14, 34));

fn solve<const PART: u8>(input: &str) -> usize {
    let g = Grid2d::from_str_as_char(input);
    let mut anti_nodes = Grid2d::from_elem(g.dim(), false);
    let mut transmitters: HashMap<char, Vec<Point<usize>>> = HashMap::default();
    for (ix, c) in g.indexed_iter() {
        if *c != '.' {
            transmitters
                .entry(*c)
                .or_default()
                .push(ix);
        }
    }
    for v in transmitters.values() {
        for v in v.iter().combinations(2) {
            let diff = v[0].as_i().unwrap() - v[1].as_i().unwrap();
            if PART == 1 {
                if let Some(x) = anti_nodes.get_i_mut(v[0].as_i().unwrap() + diff) {
                    *x = true;
                }
                if let Some(x) = anti_nodes.get_i_mut(v[1].as_i().unwrap() - diff) {
                    *x = true;
                }
            } else {
                //start at a, and take steps of diff and -diff until you run off the map.
                for d in [diff, -diff] {
                    for (p, _) in g.values_in_direction::<Point<usize>, _>(*v[0], d)
                    {
                        anti_nodes[p] = true;
                    }
                }
            }
        }
    }
    anti_nodes.iter().filter(|&&x| x).count()
}

const EG: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
