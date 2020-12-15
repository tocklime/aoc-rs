use crate::utils::inputs::parse_input_from_str_sep_by;

pub fn solve(input: &str, turns: usize) -> usize {
    let input = parse_input_from_str_sep_by::<usize>(input, ",");
    //memory is number to turn of last speaking
    let mut memory = vec![None; turns];
    for (y, t) in input[0..input.len() - 1].iter().copied().zip(1..) {
        memory[y] = Some(t);
    }
    let last_spoke = *input.last().unwrap();
    (input.len()..turns).fold(last_spoke, |last_spoke, t| {
        let speak = memory.get(last_spoke).copied().flatten().map_or(0, |x| t - x);
        memory[last_spoke] = Some(t);
        speak
    })
}

#[aoc(day15, part1)]
pub fn p1(input: &str) -> usize {
    solve(input, 2020)
}

#[aoc(day15, part2)]
pub fn p2(input: &str) -> usize {
    solve(input, 30_000_000)
}
