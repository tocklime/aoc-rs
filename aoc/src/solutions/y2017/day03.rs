use pathfinding::utils::absdiff;
use utils::cartesian::{Dir, Point};
use std::collections::HashMap;


fn p1(input: &str) -> u32 {
    let n = input.parse::<u32>().unwrap();
    let mut sqrt = (f64::from(n)).sqrt() as u32;
    if sqrt % 2 == 0 {
        sqrt -= 1;
    }
    let lowest_on_ring = sqrt * sqrt + 1;
    let highest_on_ring = (sqrt + 2) * (sqrt + 2);
    let ring_range = highest_on_ring - lowest_on_ring + 1;
    let quad = 4 * (n - lowest_on_ring) / ring_range;
    let quad_start = lowest_on_ring + quad * ring_range / 4;
    let orth_target = quad_start + sqrt / 2;
    absdiff(n, orth_target) + sqrt / 2 + 1
}


fn p2(input: &str) -> u32 {
    let mut grid: HashMap<Point<i32>, u32> = HashMap::new();
    grid.insert(Point::new(0, 0), 1);
    let n = input.parse::<u32>().unwrap();
    let mut pos = Point::new(1, 0);
    let mut dir = Dir::Up;
    loop {
        let n_sum = pos.neighbours_with_diagonals().iter()
            .map(|x| grid.get(x).unwrap_or(&0)).sum();
        grid.insert(pos, n_sum);
        if n_sum > n {
            return n_sum;
        }
        let left_pos = pos.step(dir.turn_left());
        if grid.contains_key(&left_pos) {
            //carry on forward.
            pos = pos.step(dir);
        } else {
            //turn left.
            pos = left_pos;
            dir = dir.turn_left();
        }
    }
}

#[test]
fn test_day_3() {
    assert_eq!(p1("12"), 3);
    assert_eq!(p1("23"), 2);
    assert_eq!(p1("26"), 5);
    assert_eq!(p1("1024"), 31);
}
