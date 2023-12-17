use pathfinding::prelude::*;
use utils::{
    cartesian::{Dir, Point},
    grid2d::Grid2d,
};

aoc_harness::aoc_main!(2023 day 17, part1 [p1], part2 [p2], example both EG => (102, 94), example part2 EG2 => 71);

fn p1(input: &str) -> u32 {
    let g = Grid2d::<u32>::from_str(input, |x| ((x as u8) - b'0') as u32);

    let x = dijkstra(
        &(Point::new(0, 0), Dir::Right, 0u32),
        |&(loc, dir, dir_count)| {
            //right, left or straight on.
            let mut ans = Vec::new();
            if let Some(&x) = g.get(loc.step_flip_y(dir.turn_left())) {
                ans.push(((loc.step_flip_y(dir.turn_left()), dir.turn_left(), 0), x));
            }
            if dir_count < 2 {
                if let Some(&x) = g.get(loc.step_flip_y(dir)) {
                    ans.push(((loc.step_flip_y(dir), dir, dir_count + 1), x));
                }
            }
            if let Some(&x) = g.get(loc.step_flip_y(dir.turn_right())) {
                ans.push(((loc.step_flip_y(dir.turn_right()), dir.turn_right(), 0), x));
            }
            ans
        },
        |x| x.0 == Point::new(g.dim().x - 1, g.dim().y - 1),
    )
    .unwrap();
    let mut a = Grid2d::from_fn(g.dim(), |p| g[p].to_string().chars().next().unwrap());
    for (p, d, _) in &x.0 {
        a[*p] = d.map(['^', 'v', '<', '>']);
    }
    println!("{a}");

    // dbg!(&x);
    let ans = x.1;

    ans
}
fn p2(input: &str) -> u32 {
    let g = Grid2d::<u32>::from_str(input, |x| ((x as u8) - b'0') as u32);

    let x = dijkstra(
        &(Point::new(0, 0), Dir::Right, 0u32),
        |&(loc, dir, dir_count)| {
            //right, left or straight on.
            let mut ans = Vec::new();
            if dir_count == 0 || dir_count >= 4 {
                if let Some(&x) = g.get(loc.step_flip_y(dir.turn_left())) {
                    ans.push(((loc.step_flip_y(dir.turn_left()), dir.turn_left(), 1), x));
                }
            }
            if dir_count < 10 {
                if let Some(&x) = g.get(loc.step_flip_y(dir)) {
                    ans.push(((loc.step_flip_y(dir), dir, dir_count + 1), x));
                }
            }
            if dir_count == 0 || dir_count >= 4 {
                if let Some(&x) = g.get(loc.step_flip_y(dir.turn_right())) {
                    ans.push(((loc.step_flip_y(dir.turn_right()), dir.turn_right(), 1), x));
                }
            }
            ans
        },
        |x| x.2 >= 4 && x.0 == Point::new(g.dim().x - 1, g.dim().y - 1),
    )
    .unwrap();
    let mut a = Grid2d::from_fn(g.dim(), |p| g[p].to_string().chars().next().unwrap());
    for (p, d, _) in &x.0 {
        a[*p] = ' ';//d.map(['^', 'v', '<', '>']);
    }
    println!("{a}");

    // dbg!(&x);
    let ans = x.1;

    ans
}
//1289, 1304: wrong.


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