use itertools::Itertools;
#[aoc(day2,part1)]
pub fn p1(input: &str) -> u32 {
    input.lines()
        .map(|x|
                 x.split('x')
                     .into_iter()
                     .map(|x| x.parse::<u32>().unwrap())
                     .sorted()
                     .collect_vec()
        ).map(|x| x[0]*x[1]*3+x[0]*x[2]*2+x[1]*x[2]*2)
        .sum()
}
#[aoc(day2,part2)]
pub fn p2(input: &str) -> u32 {
    input.lines()
        .map(|x|
            x.split('x')
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .sorted()
                .collect_vec()
        ).map(|x| 2*(x[0]+x[1]) + x[0]*x[1]*x[2])
        .sum()
}
