use itertools::Itertools;

#[aoc(day1, part1)]
fn p1(input: &str) -> u32 {
    let mut digs = input.chars().map(|x| x.to_digit(10).unwrap()).collect_vec();
    digs.push(digs[0]);
    digs.iter().tuple_windows()
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum()
}

#[aoc(day1, part2)]
fn p2(input: &str) -> u32 {
    let digs = input.chars().map(|x| x.to_digit(10).unwrap()).collect_vec();
    let (a, b) = digs.split_at(digs.len() / 2);
    a.into_iter().zip(b).filter_map(|(a, b)| if a == b { Some(*a) } else { None })
        .sum::<u32>() * 2
}