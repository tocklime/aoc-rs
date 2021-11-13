use utils::cartesian::{Dir, Point};
use utils::nums::NumExt;

pub fn solve<F>(input: &str,waypoint : Point<i64>, move_fn : F) -> i64 
    where F : Fn(Point<i64>,Point<i64>,Point<i64>) -> (Point<i64>,Point<i64>)
{
    input
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[1..].parse().unwrap()))
        .fold((Point::new(0, 0), waypoint), |(ship, way), (c, d)| match c {
            'F' => (ship + way * d, way),
            'L' => (ship, (d / 90).applications_of(way, |d| d.rotate_left_about_origin())),
            'R' => (ship, (d / 90).applications_of(way, |d| d.rotate_right_about_origin())),
            c => move_fn(ship,way,Dir::from_x("NSWE", c) * d)
        })
        .0
        .manhattan()
}
#[aoc(day12, part1)]
pub fn p1(input: &str) -> i64 {
    solve(input,Point::new(1,0), |ship,way,step| (ship+step,way))
}
#[aoc(day12, part2)]
pub fn p2(input: &str) -> i64 {
    solve(input,Point::new(10,1), |ship,way,step| (ship,way+step))
}
