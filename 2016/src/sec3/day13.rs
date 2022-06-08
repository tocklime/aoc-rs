use crate::utils::cartesian::Point;
use pathfinding::directed::astar::astar;
use bitintr::Popcnt;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra_all;

fn is_open(p: Point<i64>, n: i64) -> bool {
    let val = n + p.x * p.x + 3*p.x + 2*p.x*p.y + p.y + p.y*p.y;
    val.popcnt() % 2 == 0
}

#[aoc(day13,part1)]

fn p1(input: &str) -> usize {
    let favourite_n : i64 = input.parse().unwrap();
    let target:Point<i64> = Point::new(31,39);
    let start:Point<i64> = Point::new(1,1);
    astar(
        &start,
        |n| n.neighbours().iter()
            .filter_map(|&p| if p.x >= 0 && p.y >= 0 && is_open(p,favourite_n) {Some ((p,1))} else {None})
            .collect_vec(),
            |h| (target - *h).manhattan(),
        |g| *g == target
    ).unwrap().0.len() - 1
}

#[aoc(day13,part2)]

fn p2(input: &str) -> usize {
    let favourite_n : i64 = input.parse().unwrap();
    let step_limit = 50;
    let start:Point<i64> = Point::new(1,1);
    let points = dijkstra_all(
        &start,
        |n| n.neighbours().iter()
            .filter_map(|&p|
                if p.x >= 0 && p.y >= 0 && is_open(p,favourite_n) && (p-start).manhattan() <= 50
                    {Some ((p,1))} else {None})
            .collect_vec(),
    );
    points.values().filter(|(_,c)|*c <= step_limit).count() + 1 //add starting loc
}
