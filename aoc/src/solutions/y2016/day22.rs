

aoc_harness::aoc_main!(2016 day 22, generator gen, part1 [p1] => 1003, part2 [p2] => 192);
use itertools::Itertools;
use pathfinding::directed::astar::astar;
use reformation::Reformation;
use std::collections::HashMap;
use utils::cartesian::Point;

#[derive(Debug, Reformation, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[reformation(r"/dev/grid/node-x{x}-y{y}\s+{size}T\s+{used}T\s+{avail}T\s+{free}%")]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
    free: usize,
}

impl Node {
    fn viable_to(&self, b: &Self) -> bool {
        self.used > 0 && self.used < (b.size - b.used)
    }
}

fn gen(input: &str) -> Vec<Node> {
    input
        .lines()
        .skip(2)
        .map(|l| Node::parse(l).unwrap())
        .collect()
}

fn p1(nodes: &[Node]) -> usize {
    nodes
        .iter()
        .permutations(2)
        .filter(|v| v[0].viable_to(v[1]))
        .count()
}

fn p2(nodes: &[Node]) -> usize {
    //assumptions: This is a true sliding tiles puzzle, and we're never going to manage
    //to squeeze two sets of data onto one node. Max(free) = 99, Min(used) = 64.
    let target_data_x = nodes
        .iter()
        .filter_map(|n| if n.y == 0 { Some(n.x) } else { None })
        .max()
        .unwrap();
    let drawn: HashMap<Point<usize>, char> = nodes
        .iter()
        .map(|x| {
            (
                Point::new(x.x, x.y),
                match (x.x, x.y, x.used) {
                    (x, 0, _) if x == target_data_x => 'G',
                    (_, _, 0) => '_',
                    (_, _, a) => {
                        if a > 100 {
                            '#'
                        } else {
                            '.'
                        }
                    }
                },
            )
        })
        .collect();
    let empty = nodes.iter().find(|x| x.used == 0).unwrap();
    let empty_point = Point::new(empty.x, empty.y);
    let step1_target = Point::new(target_data_x - 1, 0);
    let step1_dist = astar(
        &empty_point,
        |n| {
            let ns = n.neighbours();
            ns.iter()
                .filter_map(|nn| {
                    if Some(&'.') == drawn.get(nn) {
                        Some((*nn, 1))
                    } else {
                        None
                    }
                })
                .collect_vec()
        },
        |n| n.y + target_data_x - n.x,
        |n| n == &step1_target,
    )
    .unwrap()
    .1;
    step1_dist + 5 * (target_data_x - 1) + 1
}
