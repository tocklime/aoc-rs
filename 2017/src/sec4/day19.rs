use crate::utils::cartesian::{as_point_map, Dir};

#[aoc(day19,part1)]
fn p1(input: &str) -> String {
    let hm = as_point_map::<u32>(input);
    let mut pos = hm.iter().find(|&(&p,&x)| x == '|' && p.y == 0).unwrap().0.clone();
    let mut dir = Dir::Up;
    let mut found = String::new();
    loop {
        //dbg!(pos,hm.get(&pos));
        pos = pos.step(dir);
        match hm.get(&pos) {
            None => break,
            Some(' ') => break,
            Some('|') => (),
            Some('-') => (),
            Some('+') => {
                let can_left = hm.get(&pos.step(dir.turn_left())).map(|&c| c != ' ').unwrap_or(false);
                dir = if can_left { dir.turn_left()} else {dir.turn_right()};
            },
            Some(x) => found.push(*x),
        }
    }
    found
}
#[aoc(day19,part2)]
fn p2(input: &str) -> u32 {
    let hm = as_point_map::<u32>(input);
    let mut pos = hm.iter().find(|&(&p,&x)| x == '|' && p.y == 0).unwrap().0.clone();
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
                let can_left = hm.get(&pos.step(dir.turn_left())).map(|&c| c != ' ').unwrap_or(false);
                dir = if can_left { dir.turn_left()} else {dir.turn_right()};
            },
            Some(_) => (),
        }
    }
    count
}
