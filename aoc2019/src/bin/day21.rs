use aoc_harness::aoc_main;
use utils::intcode::Computer;

aoc_main!(2019 day 21, part1 [p1] => 19359316, part2 [p2a] => 1141281622);

pub fn go(input: &str, string_script: &str) -> Option<i32> {
    let mut c: Computer = input.parse().unwrap();
    c.with_string_input(string_script)
        .run()
        .get_output()
        .iter()
        .find(|&&x| x > 255)
        .cloned()
        .or_else(|| {
            println!("{}", c.output_as_string());
            None
        })
}

pub fn p1(input: &str) -> i32 {
    //(a+b+c)*D
    go(
        input,
        "\
OR A J
AND B J
AND C J
NOT J J
AND D J
WALK
",
    )
    .unwrap_or(0)
}

pub fn p2a(input: &str) -> i32 {
    //Jump if there's a hole and we can either step or jump after.
    //(a + b + c) & D & (E + H)
    go(
        input,
        "\
OR A J
AND B J
AND C J
NOT J J
AND D J
OR E T
OR H T
AND T J
RUN
",
    )
    .unwrap_or(0)
}
