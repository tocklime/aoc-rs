use ahash::HashMap;
use itertools::Itertools;
use utils::{cartesian::Point, grid2d::Grid2d};

aoc_harness::aoc_main!(2024 day 8, part1 [p1::<1>] => 348, part2 [p1::<2>] => 1221, example both EG => (14, 34));

fn p1<const PART: u8>(input: &str) -> usize {
    let mut g = Grid2d::from_str_as_char(input);
    let mut anti_nodes = Grid2d::from_elem(g.dim(), false);
    let mut transmitters: HashMap<char, Vec<Point<isize>>> = HashMap::default();
    for (ix, c) in g.indexed_iter() {
        if *c != '.' {
            transmitters
                .entry(*c)
                .or_default()
                .push(Point::new(ix.x as isize, ix.y as isize));
        }
    }
    for v in transmitters.values() {
        for (&a, &b) in v.iter().cartesian_product(v.iter()) {
            if a != b {
                let diff = a - b;
                let anti1 = a + diff;
                let anti2 = b - diff;
                if PART == 1 {
                    if let Some(x) = anti_nodes.get_i_mut(anti1) {
                        *x = true;
                        *g.get_i_mut(anti1).unwrap() = 'X';
                    }
                    if let Some(x) = anti_nodes.get_i_mut(anti2) {
                        *x = true;
                        *g.get_i_mut(anti2).unwrap() = 'X';
                    }
                } else {
                    //start at a, and take steps of diff and -diff until you run off the map.
                    let a_u = Grid2d::<usize>::to_u(a).unwrap();
                    for (p, _) in g.values_in_direction(a_u, diff) {
                        anti_nodes[p] = true;
                    }
                    for (p, _) in g.values_in_direction(a_u, -diff) {
                        anti_nodes[p] = true;
                    }
                }
            }
        }
    }
    // println!("{g}");

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
