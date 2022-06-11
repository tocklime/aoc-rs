aoc_harness::aoc_main!(2018 day 6, generator gen, part1 [p1], part2 [p2]);

use utils::cartesian::Point;
use utils::aabb::Aabb;
use std::collections::HashMap;


fn gen(input: &str) -> Vec<Point<i32>> {
    input.trim().lines().map(|l| {
        let p : Vec<&str> = l.split(',').map(|x| x.trim()).collect();
        Point::new(p[0].parse().unwrap(),p[1].parse().unwrap())
    }).collect()
}


fn p1(input: &[Point<i32>]) -> usize {
    let mut nearest_points : HashMap<Point<i32>, (usize,bool)> = HashMap::new();
    let mut bb = Aabb::new(input[0]);
    for &p in input {
        bb = bb.extend(p);
    }
    let bb2 = bb.grow(1);
    for p in bb2.all_points() {
        let dists : Vec<(Point<i32>,i32)> = input.iter().map(|&p2| (p2,(p2 - p).manhattan())).collect();
        let min_dist = dists.iter().map(|x| x.1).min().unwrap();
        let points_at_min_dist : Vec<_> = dists.iter().filter(|x| x.1 == min_dist).collect();
        if points_at_min_dist.len() == 1 {
            let (p2,_) = points_at_min_dist[0];
            let e = nearest_points.entry(*p2).or_default();
            let on_edge = !bb.contains(&p);
            e.0 += 1;
            e.1 |= on_edge;
        }
    }
    nearest_points.values()
    .filter_map(|x|
        if !x.1 {Some(x.0)} else {None})
    .max()
    .unwrap()
}


fn p2(input: &[Point<i32>]) -> usize {
    let mut bb = Aabb::new(input[0]);
    for &p in input {
        bb = bb.extend(p);
    }
    let mut region_size = 0;
    for p in bb.all_points() {
        let total_dist : i32 = input.iter().map(|&p2| (p2-p).manhattan()).sum();
        if total_dist < 10000 {
            region_size += 1;
        }
    }
    region_size
}