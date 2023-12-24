use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use utils::{
    cartesian::Point,
    grid2d::{Coord, Grid2d},
};

aoc_harness::aoc_main!(2023 day 23, part1 [p1], part2 [p2], example both EG => (94, 154));

fn steps2(g: &Grid2d<char>, pos: Coord) -> impl Iterator<Item = Coord> + '_ {
    let allowed = match g[pos] {
        _ => pos.neighbours().to_vec(),
    };
    allowed
        .into_iter()
        .filter(|n| g.get(*n).map(|c| c != &'#').unwrap_or_default())
}
fn steps(g: &Grid2d<char>, pos: Coord) -> impl Iterator<Item = Coord> + '_ {
    let allowed = match g[pos] {
        '>' => vec![pos.right()],
        '^' => vec![pos.down()],
        '<' => vec![pos.left()],
        'v' => vec![pos.up()],
        '.' => pos.neighbours().to_vec(),
        x => panic!("Unknown grid square: {x}"),
    };
    allowed
        .into_iter()
        .filter(|n| g.get(*n).map(|c| c != &'#').unwrap_or_default())
}

type Graph = HashMap<Coord, Vec<(Coord, usize)>>;
fn fill_topo_order(g: &Graph, pos: Coord, stack: &mut Vec<Coord>, seen: &mut HashSet<Coord>) {
    seen.insert(pos);
    if let Some(nexts) = g.get(&pos) {
        for (next, _) in nexts {
            if !seen.contains(next) {
                fill_topo_order(g, *next, stack, seen);
            }
        }
    }
    stack.push(pos);
}
fn topo_order(g: &Graph, start: Coord) -> Vec<Coord> {
    let mut stack = Vec::new();
    let mut seen = HashSet::new();
    fill_topo_order(g, start, &mut stack, &mut seen);
    stack
}

#[allow(dead_code)]
fn draw_digraph(g: &Graph) {
    println!("digraph {{");
    for (from, targets) in g {
        let from_str = format!("p_{}_{}", from.x, from.y);
        for (t, cost) in targets {
            let to_str = format!("p_{}_{}", t.x, t.y);
            println!("  {from_str} -> {to_str} [label = \"{cost}\"]");
        }
    }
    println!("}}");

}

fn p2(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let start = Point::new(1, 0);
    let target = g.dim() - Point::new(2, 1);
    let mut joints: Graph = Default::default();
    let mut to_explore = vec![start];
    let mut explored = HashSet::new();
    while let Some(pos) = to_explore.pop() {
        if !explored.contains(&pos) {
            explored.insert(pos);
            for n in steps2(&g, pos) {
                let mut step_count = 1;
                let mut last = pos;
                let mut next = steps2(&g, n).filter(|x| x != &last).collect::<Vec<_>>();
                last = n;
                while next.len() == 1 {
                    step_count += 1;
                    let here = next[0];
                    next = steps2(&g, next[0]).filter(|x| x != &last).collect();
                    last = here;
                }
                if next.len() > 1 {
                    joints.entry(pos).or_default().push((last, step_count));
                    to_explore.push(last);
                } else if last == target {
                    joints.entry(pos).or_default().push((last, step_count));
                }
            }
        }
    }

    let mut dist = HashMap::new();
    dist.insert(start, 0);
    let mut todo = VecDeque::new();
    todo.push_back((vec![start], 0));
    let mut max_seen = 0;
    while let Some((path, cost)) = todo.pop_front() {
        // assert!(limit > 0);
        let pos = path.last().unwrap();
        if pos == &target {
            if cost > max_seen {
                println!("{cost}");
                max_seen = cost;
            }
        } else {
            if let Some(nexts) = joints.get(&pos) {
                for (n, cost2) in nexts {
                    if !path.contains(n) {
                        let mut new_path = path.clone();
                        new_path.push(*n);
                        todo.push_back((new_path, cost + cost2));
                    }
                }
            }
        }
    }
    max_seen
}

fn p1(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let start = Point::new(1, 0);
    let target = g.dim() - Point::new(2, 1);
    let mut joints: Graph = Default::default();
    let mut to_explore = vec![start];
    let mut explored = HashSet::new();
    while let Some(pos) = to_explore.pop() {
        if !explored.contains(&pos) {
            explored.insert(pos);
            for n in steps(&g, pos) {
                let mut step_count = 1;
                let mut last = pos;
                let mut next = steps(&g, n).filter(|x| x != &last).collect::<Vec<_>>();
                last = n;
                while next.len() == 1 {
                    step_count += 1;
                    let here = next[0];
                    next = steps(&g, next[0]).filter(|x| x != &last).collect();
                    last = here;
                }
                if next.len() > 1 {
                    joints.entry(pos).or_default().push((last, step_count));
                    to_explore.push(last);
                    println!("Start at {pos:?}, go to {last:?}, taking {step_count} steps then choose between {next:?}");
                } else if last == target {
                    joints.entry(pos).or_default().push((last, step_count));
                    println!(
                        "Start at {pos:?}, go to {last:?}, taking {step_count} steps then FINISH"
                    );
                } else {
                    println!("Start at {pos:?}, go to {last:?}, taking {step_count} steps then have no choices");
                }
            }
        }
    }
    println!("digraph {{");
    for (from, targets) in &joints {
        let from_str = format!("p_{}_{}", from.x, from.y);
        for (t, cost) in targets {
            let to_str = format!("p_{}_{}", t.x, t.y);
            println!("  {from_str} -> {to_str} [label = \"{cost}\"]");
        }
    }
    println!("}}");

    let mut dist = HashMap::new();
    dist.insert(start, 0);
    let mut stack = topo_order(&joints, start);
    while let Some(u) = stack.pop() {
        if let Some(&to_here) = dist.get(&u) {
            if let Some(nexts) = joints.get(&u) {
                for (next, cost) in nexts {
                    match dist.entry(*next) {
                        Entry::Occupied(mut x) => {
                            *x.get_mut() = (*x.get()).max(to_here + cost);
                        }
                        Entry::Vacant(x) => {
                            let _ = *x.insert(to_here + cost);
                        }
                    }
                }
            }
        }
    }
    dbg!(&dist);
    dist[&target]
}

const EG: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";
