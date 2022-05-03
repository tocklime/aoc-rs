use aoc::utils::points::Point;
use aoc_harness::aoc_main;
use std::collections::HashMap;
use utils::intcode::Computer;

aoc_main!(2019 day 13, part1 [p1] => 301, part2 [p2] => 14096);

pub fn symbol_for(i: isize) -> char {
    match i {
        0 => ' ',
        1 => '█',
        2 => 'x',
        3 => 'ￚ',
        4 => '●',
        _ => panic!("Unknown block {}", i),
    }
}
//#[aoc(day13, part1)]
pub fn p1(input: &str) -> usize {
    let mut c = input.parse::<Computer<isize>>().unwrap();
    let mut screen = HashMap::new();
    c.run();
    for v in c.get_output().chunks(3) {
        let e = screen.entry(Point(v[0], v[1])).or_default();
        *e = v[2];
    }
    screen.values().filter(|&x| *x == 2).count()
}
//#[aoc(day13, part2)]
pub fn p2(input: &str) -> isize {
    let mut c = input.parse::<Computer<isize>>().unwrap();
    c.abs_store(0, 2);
    //we don't actually need to keep track of the screen, but it feels weird not to.
    let mut screen = HashMap::new();
    let mut paddle_x = 0;
    let mut ball_x = 0;
    let mut score = 0;
    while !c.is_halted() {
        c.run_to_input();
        for v in c.take_output().chunks(3) {
            if v[0] == -1 && v[1] == 0 {
                score = v[2];
            } else {
                let sym = symbol_for(v[2]);
                match sym {
                    '●' => ball_x = v[0],
                    'ￚ' => paddle_x = v[0],
                    _ => (),
                }
                *screen.entry(Point(v[0], v[1])).or_default() = sym;
            }
        }
        let correct_move = (ball_x - paddle_x).signum();
        c.with_input(correct_move);
    }
    score
}
