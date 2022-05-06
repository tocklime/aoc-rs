use aoc::utils::algorithms::{bfs_dist_all, to_lookup};
use aoc::utils::points::{as_point_map, Point};
use aoc::utils::prelude::HashMap;
use aoc_harness::aoc_main;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use std::convert::TryInto;
use std::hash::Hash;

aoc_main!(2019 day 20, part1 [p1] => 692, part2 [p2] => 8314,
example part1 EG0 => 58,
example part2 EG2A => 11,
example part2 EG2B => 396);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Telepad {
    pos: Point,
    depth_change: isize,
}

//#[aoc(day20, part1)]
pub fn p1(input: &str) -> u32 {
    solve(input, 0)
}
//#[aoc(day20, part2)]
pub fn p2(input: &str) -> u32 {
    solve(input, 1)
}
pub fn solve(input: &str, depth_step: isize) -> u32 {
    let maz = as_point_map(input);
    let width: isize = input.lines().next().unwrap().len().try_into().unwrap();
    let height: isize = input.lines().count().try_into().unwrap();
    let telepads = to_lookup::<_, String, Telepad>(maz.iter().filter_map(|(p, c)| {
        if c.is_ascii_alphabetic() {
            let n: Option<Vec<_>> = p.neighbours().iter().map(|x| maz.get(x)).collect();
            if let Some(n) = n {
                let a = if n[0].is_ascii_alphabetic() && n[2] == &'.' {
                    Some((format!("{}{}", c, n[0]), p.down()))
                } else if n[2].is_ascii_alphabetic() && n[0] == &'.' {
                    Some((format!("{}{}", n[2], c), p.up()))
                } else if n[1].is_ascii_alphabetic() && n[3] == &'.' {
                    Some((format!("{}{}", n[1], c), p.right()))
                } else if n[3].is_ascii_alphabetic() && n[1] == &'.' {
                    Some((format!("{}{}", c, n[3]), p.left()))
                } else {
                    None
                };
                let is_outer = p.0 < 3 || ((width - p.0) < 3) || p.1 < 3 || ((height - p.1) < 3);
                a.map(|(name, pos)| {
                    (
                        name,
                        Telepad {
                            pos,
                            depth_change: if is_outer { -1 } else { 1 },
                        },
                    )
                })
            } else {
                None
            }
        } else {
            None
        }
    }));
    let teleports: HashMap<Point, (Point, isize)> = telepads
        .values()
        .flat_map(|vs| {
            if vs.len() == 2 {
                vec![
                    (vs[0].pos, (vs[1].pos, vs[0].depth_change)),
                    (vs[1].pos, (vs[0].pos, vs[1].depth_change)),
                ]
            } else {
                vec![]
            }
        })
        .collect();

    let start = telepads["AA"][0].pos;
    let end = telepads["ZZ"][0].pos;
    let walking: HashMap<Point, HashMap<Point, (u32, isize)>> = telepads
        .values()
        .flatten()
        .map(|tp| {
            let s: HashMap<Point, u32> = bfs_dist_all(&tp.pos, |p| {
                p.neighbours()
                    .iter()
                    .filter_map(|n| {
                        if maz.get(n) == Some(&'.') {
                            Some((*n, 1))
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            });
            let filtered = s
                .iter()
                .filter_map(|(p, &dist)| {
                    if *p == end {
                        Some((*p, (dist, 0)))
                    } else {
                        teleports
                            .get(p)
                            .map(|(p, dc)| (*p, (dist + 1, *dc * depth_step)))
                    }
                })
                .collect::<HashMap<Point, (u32, isize)>>();
            (tp.pos, filtered)
        })
        .collect();
    dijkstra(
        &(start, 0),
        |(pos, depth)| {
            walking[pos]
                .iter()
                .filter_map(|(p, (dist, dc))| {
                    let new_depth = depth + dc;
                    if new_depth >= 0 {
                        Some(((*p, new_depth), *dist))
                    } else {
                        None
                    }
                })
                .collect_vec()
        },
        |(p, d)| *p == end && *d == 0,
    )
    .expect("No solution")
    .1
}

const EG0: &str = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               
";
const EG2A: &str = "       A       
       A       
  #####.#####  
  #####.#####  
AB..###.#####  
  #.## A ####  
BC..## BC....ZZ
  ###########  
  ###########  
               
               
";
const EG2B: &str = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     
";
