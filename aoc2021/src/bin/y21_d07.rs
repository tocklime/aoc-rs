use std::cmp::min;

use aoc_harness::*;

aoc_main!(2021 day 7, generator input::<isize,','>, part1 [p1] => 352_997, part2 [p2] => 101_571_302,
  example both EG => (37, 168)
);
const EG: &str = "16,1,2,0,4,2,7,1,2,14";

fn p1(input: &[isize]) -> isize {
    let mut input = input.to_vec();
    input.sort_unstable();
    let median = input[input.len() / 2];
    input.iter().map(|&x| (x - median).abs()).sum()
}

fn fuel_to_pos2(input: &[isize], pos: isize) -> isize {
    input
        .iter()
        .map(|&x| {
            let steps = (x - pos).abs();
            steps * (steps + 1) / 2
        })
        .sum()
}

fn p2(input: &[isize]) -> isize {
    let len: isize = input.len().try_into().unwrap();
    let mean: isize = input.iter().sum::<isize>() / len;
    min(fuel_to_pos2(input, mean), fuel_to_pos2(input, mean + 1))
}
#[cfg(test)]
mod easter_egg {
    use aoc_harness::Opts;
    use utils::intcode::Computer;

    #[test]
    fn its_intcode() {
        let opts = Opts::for_test();
        let mut c: Computer<isize> = opts.get_input(2021, 7).parse().unwrap();
        let output = c.run().output_as_string();
        assert_eq!(output, "Ceci n'est pas une intcode program\n");
    }
}
