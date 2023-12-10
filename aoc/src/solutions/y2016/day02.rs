

aoc_harness::aoc_main!(2016 day 2, part1 [p1] => "52981", part2 [p2] => "74CD2");
use utils::cartesian::{as_point_map, Point};
use nom::lib::std::collections::HashMap;

fn solve_pad(pad: &str, input: &str) -> String {
    let pad: HashMap<_, _> = as_point_map(pad,false).into_iter().filter(|&(_, c)| c != ' ').collect();

    let mut pos = Point::new(1, 1);
    input.lines().map(|l| {
        for c in l.chars() {
            let new_pos = pos.follow_x("DULR", c);
            if pad.contains_key(&new_pos) {
                pos = new_pos;
            }
        }
        pad[&pos]
    }).collect()
}

fn p1(input: &str) -> String {
    solve_pad("123\n456\n789", input)
}

fn p2(input: &str) -> String {
    solve_pad("  1  \n 234 \n56789\n ABC \n  D  ", input)
}