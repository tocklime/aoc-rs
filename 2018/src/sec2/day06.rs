use crate::utils::cartesian::{Point, point_map_bounding_box};
use itertools::Itertools;
use std::collections::HashMap;

fn unique_min_by<A, B, F, T>(iter: T, f: F) -> Option<A>
    where A : Copy, B : Ord,
          F : FnMut(&A) -> B,
          T : Iterator<Item = A> {
    let xs = iter.sorted_by_key(f).collect_vec();
    if f(&xs[0]) == f(&xs[1]) {
        None
    } else {
        Some(xs[0])
    }
}

#[aoc(day6,part1)]
fn p1(input: &str) -> usize {
    let mut grid = HashMap::new();
    let mut points = Vec::new();
    for l in input.lines() {
        let x = l.split(',').collect_vec();
        let p: Point<u32> = Point::new(x[0].parse().unwrap(), x[1].parse().unwrap());
        grid.insert(p,'X');
        points.push(p);
    }
    let bb = point_map_bounding_box(&grid);
    for p in bb.all_points() {
        let nearest = unique_min_by(&points, |x|)

    }

    0
}