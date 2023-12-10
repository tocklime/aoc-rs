

aoc_harness::aoc_main!(2016 day 12, part1 [p1] => 318_077, part2 [p2] => 9_227_731);
use utils::assembunny::Computer;

fn p1(input: &str) -> i64 {
    let mut c = Computer::parse(input);
    c.run();
    c.get_reg('a')
}

fn p2(input: &str) -> i64 {
    let mut c = Computer::parse(input);
    c.set_reg('c',1);
    c.run();
    c.get_reg('a')
}
