use crate::utils::assembunny::Computer;

#[aoc(day23,part1)]
#[post(ret == 12_775)]
fn p1(input: &str) -> i64 {
    let mut c = Computer::parse(input);
    c.set_reg('a',7);
    c.run();
    c.get_reg('a')
}
#[aoc(day23,part2)]
#[post(ret == 479_009_335)]
fn p2(input: &str) -> i64 {
    let mut c = Computer::parse(input);
    c.set_reg('a',12);
    while c.running() {
        if c.instruction_pointer == 5 {
            c.set_reg('a', c.get_reg('b') * c.get_reg('d'));
            c.set_reg('c',0);
            c.set_reg('d',0);
            c.instruction_pointer = 9;
        }
        c.step();
    }
    c.get_reg('a')
}
