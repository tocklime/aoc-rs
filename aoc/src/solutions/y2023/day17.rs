use pathfinding::prelude::*;
use utils::{
    cartesian::{Dir, Point},
    grid2d::Grid2d,
};

aoc_harness::aoc_main!(2023 day 17,
    generator gen,
    part1 [solve_astar::<0,3>,solve_dijk::<0, 3>] => 1110,
    part2 [solve_astar::<4,10>,solve_dijk::<4,10>] => 1294,
    example both EG => (102, 94), example part2 EG2 => 71);

type Grid = Grid2d<u8>;
fn gen(input: &str) -> Grid {
    Grid2d::from_iter(input.as_bytes().iter(), |&x| x - b'0', &b'\n')
}
type State = (Point<usize>, Dir, usize);

fn step<const MIN: usize, const MAX: usize>(
    g: &Grid,
    &(loc, dir, dir_count): &State,
) -> Vec<(State, usize)> {
    //right, left or straight on.
    let mut ans = Vec::new();
    if dir_count == 0 || dir_count >= MIN {
        //turning OK.
        for d in [dir.turn_left(), dir.turn_right()] {
            let new_pos = loc.step(d);
            if let Some(&x) = g.get(new_pos) {
                ans.push(((new_pos, d, 1), x.into()));
            }
        }
    }
    if dir_count < MAX {
        //straight line OK.
        let new_pos = loc.step(dir);
        if let Some(&x) = g.get(new_pos) {
            ans.push(((new_pos, dir, dir_count + 1), x.into()));
        }
    }
    ans
}
fn solve_astar<const MIN: usize, const MAX: usize>(g: &Grid) -> usize {
    let target = g.indexes().next_back().unwrap();
    astar(
        &(Point::new(0, 0), Dir::Right, 0),
        |s| step::<MIN, MAX>(g, s),
        |(loc, _, _)| loc.manhattan_unsigned(&target),
        |x| x.2 >= MIN && x.0 == target,
    )
    .unwrap()
    .1
}
fn solve_dijk<const MIN: usize, const MAX: usize>(g: &Grid) -> usize {
    let target = g.indexes().next_back().unwrap();
    dijkstra(
        &(Point::new(0, 0), Dir::Right, 0),
        |s| step::<MIN, MAX>(g, s),
        |x| x.2 >= MIN && x.0 == target,
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
