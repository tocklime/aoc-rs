use aoc_harness::*;

aoc_main!(2021 day 6, generator input::<usize, ','>, [solve::<80>], [solve::<256>],
          example both EG => (5934, 26984457539_usize));

fn solve<const GENERATIONS: usize>(input: &[usize]) -> usize {
    let mut counts = [0; 9];
    for &f in input {
        counts[f] += 1;
    }
    for d in 0..GENERATIONS {
        let base = d % 9;
        let new_fish = counts[base];
        counts[(base + 7) % 9] += new_fish;
    }
    counts.iter().sum()
}

const EG: &str = "3,4,3,1,2";
