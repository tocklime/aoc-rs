use crate::utils::cartesian::{as_point_map, Dir, point_map_bounding_box};

#[aoc(day22,part1)]
fn p1(input: &str) -> usize {
    let mut hm = as_point_map::<i32>(input, true);
    let mut point = point_map_bounding_box(&hm).center();
    let mut dir = Dir::Up;
    let mut count = 0;
    for _ in 0..10000 {
        if hm.get(&point) == Some(&'#') {
            dir = dir.turn_right();
            hm.insert(point,'.');
        }else {
            dir = dir.turn_left();
            hm.insert(point,'#');
            count += 1;
        }
        point = point.step(dir);
    }
    count
}
#[aoc(day22,part2)]
fn p2(input: &str) -> usize {
    let mut hm = as_point_map::<i32>(input, true);
    let mut point = point_map_bounding_box(&hm).center();
    let mut dir = Dir::Up;
    let mut count = 0;
    for _ in 0..10_000_000 {
        let (new_dir,c) = match hm.get(&point) {
            Some(&'#') /* INFECTED */ => (dir.turn_right(),'F'),
            Some(&'W') /* WEAKENED */ => (dir,'#'),
            Some(&'F') /* FLAGGED */ => (dir.turn_about(),'.'),
            _ /* CLEAN */ => (dir.turn_left(),'W'),
        };
        if c == '#' {
            count += 1;
        }
        dir = new_dir;
        hm.insert(point,c);
        point = point.step(dir);
    }
    count
}
