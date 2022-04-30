use aoc2019::utils::nums::unbounded_bin_search;
use aoc2019::utils::points::Point;
use aoc_harness::aoc_main;
use itertools::Itertools;
use utils::intcode::Computer;

aoc_main!(2019 day 19, part1 [p1] => 234, part2 [p2a] => 9290812);

pub fn calc(c: &Computer<isize>, p: Point) -> isize {
    let mut c = c.clone();
    c.with_input(p.0).with_input(p.1).run().get_last_output()
}


pub fn p1(input: &str) -> isize {
    let c: Computer<isize> = input.parse().unwrap();
    (0..50)
        .cartesian_product(0..50)
        .map(|(x, y)| calc(&c, Point(x, y)))
        .sum()
}

///This function wrongly assumes that the top edge of the tractor beam is on y=x.
pub fn p2(input: &str) -> isize {
    let c: Computer<isize> = input.parse().unwrap();
    let s = unbounded_bin_search(|s| calc(&c, Point(s + 99, s - 99)), 1);
    s * 10000 + (s - 100)
}

pub fn p2a(input: &str) -> isize {
    let c: Computer<isize> = input.parse().unwrap();
    let mut p = Point::origin();
    loop {
        let up = calc(&c, p + Point(0, 99));
        let right = calc(&c, p + Point(99, 0));
        if up == 1 && right == 1 {
            break;
        }
        p += Point(1 - up, 1 - right);
    }
    p.0 * 10000 + p.1
}
