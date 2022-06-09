use aoc_harness::aoc_main;

aoc_main!(2017 day 19, part1 [p1], part2 [p2]);
use utils::cartesian::{as_point_map, Dir};

fn p1(input: &str) -> String {
    let hm = as_point_map::<u32>(input, false);
    let mut pos = *hm.iter().find(|&(&p, &x)| x == '|' && p.y == 0).unwrap().0;
    let mut dir = Dir::Up;
    let mut found = String::new();
    loop {
        pos = pos.step(dir);
        match hm.get(&pos) {
            None | Some(' ') => break,
            Some('|') => (),
            Some('-') => (),
            Some('+') => {
                let can_left = hm
                    .get(&pos.step(dir.turn_left()))
                    .map_or(false, |&c| c != ' ');
                dir = if can_left {
                    dir.turn_left()
                } else {
                    dir.turn_right()
                };
            }
            Some(x) => found.push(*x),
        }
    }
    found
}

fn p2(input: &str) -> u32 {
    let hm = as_point_map::<u32>(input, false);
    let mut pos = *hm.iter().find(|&(&p, &x)| x == '|' && p.y == 0).unwrap().0;
    let mut dir = Dir::Up;
    let mut count = 0;
    loop {
        //dbg!(pos,hm.get(&pos));
        pos = pos.step(dir);
        count += 1;
        match hm.get(&pos) {
            None => break,
            Some(' ') => break,
            Some('|') => (),
            Some('-') => (),
            Some('+') => {
                let can_left = hm
                    .get(&pos.step(dir.turn_left()))
                    .map(|&c| c != ' ')
                    .unwrap_or(false);
                dir = if can_left {
                    dir.turn_left()
                } else {
                    dir.turn_right()
                };
            }
            Some(_) => (),
        }
    }
    count
}
