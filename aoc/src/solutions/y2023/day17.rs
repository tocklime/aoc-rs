use pathfinding::prelude::*;
use utils::{
    cartesian::{Dir, Point},
    grid2d::{Grid2d, ICoord},
};

aoc_harness::aoc_main!(2023 day 17,
    generator gen,
    part1 [solve_astar::<1,3>,solve_dijk::<1, 3>] => 1110,
    part2 [solve_astar::<4,10>,solve_dijk::<4,10>] => 1294,
    example both EG => (102, 94), example part2 EG2 => 71);

type Grid = Grid2d<u8>;
fn gen(input: &str) -> Grid {
    Grid2d::from_iter(input.as_bytes().iter(), |&x| x - b'0', &b'\n')
}

//State is current location, and the 2 directions we're allowed to go from here.
type State = (Point<usize>, [Dir; 2]);

fn step<const MIN: isize, const MAX: isize>(g: &Grid, &(loc, dirs): &State) -> Vec<(State, usize)> {
    //right, left or straight on.
    //walk for MIN..MAX steps, and then turn left or right.

    dirs.iter().flat_map(|dir| {
        //need to start from 1 to make sure we don't miss costs.
        let cost_to_min_steps : usize = (1..MIN).filter_map(|ss| { 
            let step : ICoord = dir.as_point_step() * ss;
            g.get(loc + step).copied().map(usize::from)
        }).sum();
        (MIN..=MAX).scan(cost_to_min_steps, |cost,step_size| {
            let step: ICoord = dir.as_point_step() * step_size;
            let new_pos = loc + step;
            g.get(new_pos).map(|&x| {
                *cost += usize::from(x);
                ((new_pos, [dir.turn_left(), dir.turn_right()]), *cost)
            })
        })
    }).collect()
}
fn solve_astar<const MIN: isize, const MAX: isize>(g: &Grid) -> usize {
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
fn solve_dijk<const MIN: isize, const MAX: isize>(g: &Grid) -> usize {
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
