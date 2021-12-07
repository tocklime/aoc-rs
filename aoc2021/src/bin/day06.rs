use aoc_harness::*;

aoc_main!(2021 day 6, generator input::<usize, ','>, both [solve],
          example both EG => (5934, 26_984_457_539_usize));

fn solve(input: &[usize]) -> (usize, usize) {
    let mut counts = [0; 9];
    input.iter().for_each(|&x|counts[x] +=1);
    for d in 0..80 {
        counts[(d + 7) % 9] += counts[(d % 9)];
    }
    let p1 = counts.iter().sum();
    for d in 80..256 {
        counts[(d + 7) % 9] += counts[(d % 9)];
    }
    (p1, counts.iter().sum())
}

const EG: &str = "3,4,3,1,2";
