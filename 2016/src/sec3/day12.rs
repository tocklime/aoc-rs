use crate::utils::assembunny::Computer;

#[aoc(day12, part1)]
#[post(ret == 318_077)]
fn p1(input: &str) -> i64 {
    let mut c = Computer::parse(input);
    c.run();
    c.get_reg('a')
}

#[aoc(day12, part2)]
#[post(ret == 9_227_731)]
fn p2(input: &str) -> i64 {
    let mut c = Computer::parse(input);
    c.set_reg('c',1);
    c.run();
    c.get_reg('a')
}
