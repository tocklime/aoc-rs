use pathfinding::prelude::*;
use utils::{
    cartesian::{Dir, Point},
    grid2d::{Grid2d, ICoord},
};

aoc_harness::aoc_main!(2023 day 17,
    generator gen,
    part1 [manual_astar_no_hash::<1,3>, pf_astar::<1,3>, pf_dijk::<1, 3>] => 1110,
    part2 [manual_astar_no_hash::<4,10>, pf_astar::<4,10>, pf_dijk::<4,10>] => 1294,
    example both EG => (102, 94), example part2 EG2 => 71);

type Grid = Grid2d<u8>;
fn gen(input: &str) -> Grid {
    Grid2d::from_iter(input.as_bytes().iter(), |&x| x - b'0', &b'\n')
}

//State is current location, and the 2 directions we're allowed to go from here.
type State = (Point<usize>, [Dir; 2]);

fn step2<const MIN: isize, const MAX: isize>(
    g: &Grid,
    loc: Point<usize>,
    dir_ix: usize,
) -> Vec<(Point<usize>, usize, usize)> {
    //right, left or straight on.
    //walk for MIN..MAX steps, and then turn left or right.
    let dirs = [[Dir::Up, Dir::Down], [Dir::Left, Dir::Right]][dir_ix];
    dirs.iter()
        .flat_map(|dir| {
            //need to start from 1 to make sure we don't miss costs.
            let cost_to_min_steps: usize = (1..MIN)
                .filter_map(|ss| {
                    let step: ICoord = dir.as_point_step() * ss;
                    g.get(loc + step).copied().map(usize::from)
                })
                .sum();
            (MIN..=MAX).scan(cost_to_min_steps, |cost, step_size| {
                let step: ICoord = dir.as_point_step() * step_size;
                let new_pos = loc + step;
                g.get(new_pos).map(|&x| {
                    *cost += usize::from(x);
                    (new_pos, if dir_ix == 1 { 0 } else { 1 }, *cost)
                })
            })
        })
        .collect()
}
fn step<const MIN: isize, const MAX: isize>(g: &Grid, &(loc, dirs): &State) -> Vec<(State, usize)> {
    //right, left or straight on.
    //walk for MIN..MAX steps, and then turn left or right.
    dirs.iter()
        .flat_map(|dir| {
            //need to start from 1 to make sure we don't miss costs.
            let cost_to_min_steps: usize = (1..MIN)
                .filter_map(|ss| {
                    let step: ICoord = dir.as_point_step() * ss;
                    g.get(loc + step).copied().map(usize::from)
                })
                .sum();
            (MIN..=MAX).scan(cost_to_min_steps, |cost, step_size| {
                let step: ICoord = dir.as_point_step() * step_size;
                let new_pos = loc + step;
                g.get(new_pos).map(|&x| {
                    *cost += usize::from(x);
                    ((new_pos, [dir.turn_left(), dir.turn_right()]), *cost)
                })
            })
        })
        .collect()
}
fn pf_astar<const MIN: isize, const MAX: isize>(g: &Grid) -> usize {
    let target = g.indexes().next_back().unwrap();
    let a = astar(
        &(Point::new(0, 0), [Dir::Up, Dir::Right]),
        |s| step::<MIN, MAX>(g, s),
        |(loc, _)| loc.manhattan_unsigned(&target),
        |x| x.0 == target,
    )
    .unwrap();
    a.1
}

// Inspired by
// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day17.rs
fn manual_astar_no_hash<const MIN: isize, const MAX: isize>(g: &Grid) -> usize {
    let target = g.indexes().next_back().unwrap();
    //bests represents the time of the fastest way to get to each cell.
    //you can arrive at a cell horizontally or vertically. So we store 2 values in each cell.
    let mut bests = Grid2d::from_elem(g.dim(), [usize::MAX, usize::MAX]);
    bests[Point::new(0,0)] = [0,0];
    let max_possible_cost = 10 * (g.dim().y + g.dim().x);
    let mut fringe = vec![vec![]; max_possible_cost];
    fringe[0].push((Point::new(0, 0), 0));
    fringe[0].push((Point::new(0, 0), 1));
    for fringe_ix in 0..fringe.len() {
        while let Some((loc, dir)) = fringe[fringe_ix].pop() {
            let cost_to_here = bests[loc][dir];
            if loc == target {
                return cost_to_here;
            }
            for (loc, dir, cost) in step2::<MIN,MAX>(g, loc, dir) {
                let to_target = loc.manhattan_unsigned(&target);
                let curr = &mut bests[loc][dir];
                if (cost_to_here + cost) < *curr {
                    *curr = cost_to_here + cost;
                    fringe[cost_to_here + cost + to_target].push((loc, dir));
                }
            }
        }
    }
    unreachable!()
}

fn pf_dijk<const MIN: isize, const MAX: isize>(g: &Grid) -> usize {
    let target = g.indexes().next_back().unwrap();
    dijkstra(
        &(Point::new(0, 0), [Dir::Up, Dir::Right]),
        |s| step::<MIN, MAX>(g, s),
        |x| x.0 == target,
    )
    .unwrap()
    .1
}

const EG: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

const EG2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991
";
