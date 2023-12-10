

aoc_harness::aoc_main!(2016 day 23, part1 [p1] => 12775, part2 [p2] => 479009335);
use utils::assembunny::Computer;

fn p1(input: &str) -> i64 {
    let mut c = Computer::parse(input);
    c.set_reg('a', 7);
    c.run();
    c.get_reg('a')
}

fn p2(input: &str) -> i64 {
    let mut c = Computer::parse(input);
    c.set_reg('a', 12);
    while c.running() {
        if c.get_instruction_pointer() == 5 {
            c.set_reg('a', c.get_reg('b') * c.get_reg('d'));
            c.set_reg('c', 0);
            c.set_reg('d', 0);
            c.set_instruction_pointer(9);
        }
        c.step();
    }
    c.get_reg('a')
}
