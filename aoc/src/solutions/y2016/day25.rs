use aoc_harness::aoc_main;

aoc_main!(2016 day 25, part1 [p1]);
use utils::assembunny::Computer;

fn p1(input: &str) -> i64 {
    let c = Computer::parse(input);
    for x in 0.. {
        let mut c2 = c.clone();
        c2.set_reg('a', x);
        for y in 0..=50 {
            while c2.output_is_empty() && c2.running() {
                if c2.get_instruction_pointer() == 0 {
                    let calc = c2.get_reg('a') + 365 * 7;
                    c2.set_reg('a', calc);
                    c2.set_reg('b', 0);
                    c2.set_reg('c', 0);
                    c2.set_reg('d', calc);
                    c2.set_instruction_pointer(9);
                } else if c2.get_instruction_pointer() == 9 {
                    let a = c2.get_reg('a');
                    c2.set_reg('a', a / 2);
                    c2.set_reg('b', 0);
                    c2.set_reg('c', 2 - (a % 2));
                    c2.set_instruction_pointer(20);
                } else {
                    c2.step();
                }
            }
            let next = c2.take_output().into_iter().next();
            if next != Some(y % 2) {
                break;
            } else if y == 50 {
                return x;
            }
        }
    }
    0
}
/*

0: cpy a d
1: cpy 7 c
2: cpy 365 b
3: inc d
4: dec b
5: jnz b -2
6: dec c
7: jnz c -5
8: cpy d a # a = a + 365 * 7

9: jnz 0 0 #nop

10: cpy a b # b = a
11: cpy 0 a # a = 0
12: cpy 2 c # loop { c = 2

13: jnz b 2 # loop {
14: jnz 1 6 #
15: dec b   # b--
16: dec c   # c--
17: jnz c -4 # } until c == 0. That is, c -= b.
18: inc a   # a++
19: jnz 1 -7 # } until b == 0. That is, a = a / 2

20: cpy 2 b #   b = 2 (c is a % 2)
21: jnz c 2 # if c == 0
22: jnz 1 4 # goto 26
23: dec b   # b--
24: dec c   # c--
25: jnz 1 -4 # goto 21.
26: jnz 0 0 # nop - that is, b =
27: out b
28: jnz a -19
29: jnz 1 -21

*/
