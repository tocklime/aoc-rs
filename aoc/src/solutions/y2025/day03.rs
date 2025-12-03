aoc_harness::aoc_main!(2025 day 3, part1 [solve::<2>] => 17311, part2 [solve::<12>] => 171_419_245_422_055, example part1 EG => 357, example part2 EG => 3_121_910_778_619);

fn solve<const BATT_COUNT: usize>(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut b = l.as_bytes();
            let mut total = 0usize;
            for d in 1..=BATT_COUNT {
                let need_to_the_right = BATT_COUNT - d;
                let max_ix = b.len() - need_to_the_right;
                let max_char = b[0..max_ix].iter().max().unwrap();
                let max_pos = b[0..max_ix].iter().position(|x| x == max_char).unwrap();
                total = total * 10 + usize::from(max_char - b'0');
                b = &b[max_pos + 1..];
            }
            total
        })
        .sum()
}

const EG: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";
