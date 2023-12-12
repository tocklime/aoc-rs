use itertools::{repeat_n, Itertools};

aoc_harness::aoc_main!(2023 day 12,
    part1 [ solve_all::<1>] => 7221,
    part2 [solve_all::<5>] => 7_139_671_893_722,
    example both EG => (21,525_152), example part1 EG1 => 10);

fn solve(springs: &str, counts: &[usize]) -> usize {
    let mut dp = counts.iter().map(|x| vec![0usize; x + 1]).collect_vec();
    dp.push(vec![0]);
    //indexes to dp are
    // [how many blocks of broken springs we've seen the end of]
    // [how many broken springs we've seen so far this block.]
    // values of dp are how many ways there is to get to that state.
    // we mutate dp in place for each character in the string.
    // it has been constructed so that each 'row' (options of how many
    // broken springs we've seen so far) is one more than the relevant number in `counts`.

    dp[0][0] = 1;
    //things move to the right and down in dp, so iterate
    //backward to avoid double stepping anything.
    for c in springs.as_bytes() {
        for done_counts in (0..dp.len()).rev() {
            for so_far in (0..dp[done_counts].len()).rev() {
                let count = dp[done_counts][so_far];
                if count > 0 {
                    //consider the case where we've done `done_counts`,
                    //the current block has `so_far`, so far, and we have a c.

                    //Don't usually stay still, so there's no ways (yet) of reaching this state now.
                    dp[done_counts][so_far] = 0;

                    if b"?#".contains(c) {
                        //this could extend the current count -> move one space to the right.
                        if so_far < dp[done_counts].len() - 1 {
                            dp[done_counts][so_far + 1] += count;
                        }
                    }
                    if b"?.".contains(c) {
                        match so_far {
                            //We haven't seen any #s yet, state remains as is.
                            0 => dp[done_counts][0] += count,
                            //This ends a correctly-lengthed block of #s. Move to next line.
                            x if x == dp[done_counts].len() - 1 => {
                                dp[done_counts + 1][0] += count;
                            }
                            //Otherwise, it's the wrong number of things. Drop it.
                            _ => (),
                        }
                    }
                }
            }
        }
    }
    //Finally, we want total of the final two states. (depending on whether we saw a final '.' or not)
    dp.iter().flatten().rev().take(2).sum()
    // dp[dp.len() - 1][0] + dp[dp.len() - 2].last().unwrap()
}

fn solve_all<const N: usize>(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (springs, counts) = l.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            let i: String = repeat_n(springs, N).join("?");
            let counts = repeat_n(counts.iter(), N).flatten().copied().collect_vec();
            solve(&i, &counts)
        })
        .sum()
}

const EG1: &str = "?###???????? 3,2,1";
const EG: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
