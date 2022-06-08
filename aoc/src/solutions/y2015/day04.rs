use aoc_harness::aoc_main;

aoc_main!(2015 day 4, part1 [p1], part2 [p2]);

use md5;

fn p1(input: &str) -> usize {
    (0..).find(|i| {
        let str = format!("{}{}",input,i);
        let md5 = md5::compute(&str);
        md5[0] == 0 && md5[1] == 0 && (md5[2] & 0xF0) == 0
    }).unwrap()
}

fn p2(input: &str) -> usize {
    (0..).find(|i| {
        let str = format!("{}{}",input,i);
        let md5 = md5::compute(&str);
        md5[0] == 0 && md5[1] == 0 && md5[2] == 0
    }).unwrap()
}
