type Input = String;
type Output = i32;
type Output2 = Output;

#[aoc_generator(day1)]
 pub fn input_generator(input: &str) -> Input {
     input.to_string()
}

#[aoc(day1, part1)]
pub fn part1(input: &Input) -> Output {
    0
}

#[aoc(day1, part2)]
pub fn part2(input: &Input) -> Output2 {
    0
}