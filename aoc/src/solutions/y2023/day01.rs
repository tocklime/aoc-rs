use aoc_harness::*;

aoc_main!(2023 day 1, part1 [p1], example part1 EG => 1); 

fn p1(_input: &str) -> usize {
    println!("Hello, World");
    dbg!(&_input);
    _input.len()
}

const EG : &str = "
";