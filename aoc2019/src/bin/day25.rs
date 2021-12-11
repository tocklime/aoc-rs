use aoc2019::utils::prelude::*;
use std::io::{stdin, stdout, Write};

const SOLUTION: &str = "east
east
east
north
north
take fuel cell
south
south
west
west
take food ration
south
take prime number
north
west
north
north
west
take mug
east
south
west
north
west
";

//#[aoc(day25, part1)]
pub fn p1(input: &str) -> String {
    let mut c: Computer<i64> = input.parse().unwrap();
    c.with_string_input(SOLUTION);
    c.run_to_input();
    c.clear_output();
    c.with_string_input(&"north\n");
    c.run_to_input();
    c.output_as_string()
}
pub fn interactive(input: &str) -> i64 {
    let mut c: Computer<i64> = input.parse().unwrap();
    let mut save = c.clone();
    c.run_to_input();
    loop {
        let out = c.output_as_string();
        c.clear_output();
        print!("{}", out);
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Bad input");
        let r = s.find("\r").unwrap();
        s.remove(r);
        if s.trim().is_empty() {
            c = save.clone();
        } else {
            save = c.clone();
            c.with_string_input(&s);
            c.run_to_input();
        }
    }
}

/*
                Q-C-S
        |       |
        Ch-St O-A     P
           |    |     |
           E----H     Ho-W
                |     |
                S-F-G-N-K
                  | |
                  L Si

H: Photons - do not take!
A_rcade:
Q_uarters: infinite loop - do not take.
C_orridor: molten lava - do not take.
S_torage: loom
O_bservatory: mug
E_ngineering: escape pod - do not take.
St_ables: giant electromagnet - do not take.
Ch_eckpoint:
F_ountain: food ration
L_ab: prime number
G_ift wrapping centre: manifold
N_avigation:
H_olodeck:
P_assages: fuel cell
W_arp drive maintenance: spool of cat6
K_itchen: jam
Si_ck bay:

items:
Jam, Loom, Mug, Spool of cat6, Prime number,
Food ration, Fuel cell, Manifold

Spool is too heavy by itself. - OUT
All remaining except Prime Number or Food ration is too light - they're IN
Now taking just certain items and Loom is too much - it's OUT
Now taking everything except mug or fuel cell is too light - they're IN
Now taking just certain items (PN, FR, Mu, FC) is THE ANSWER.
*/
