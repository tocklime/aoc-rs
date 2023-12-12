use itertools::{repeat_n, Itertools};

aoc_harness::aoc_main!(2023 day 12,
    part1 [ solve_all::<1>] => 7221,
    part2 [solve_all::<5>] => 7_139_671_893_722,
    example both EG => (21,525_152), example part1 EG1 => 10);

const BIGGEST_GROUP: usize = 16;
const MAX_GROUP_COUNT: usize = 32;

fn solve(springs: &str, counts: &[usize]) -> usize {
    let mut dp = [[0usize; BIGGEST_GROUP]; MAX_GROUP_COUNT];
    // Indexes to dp are
    // [how many blocks of broken springs we've seen the end of]
    // [how many broken springs we've seen so far this block.]
    // values of dp are how many ways there is to get to that state.
    // we mutate dp in place for each character in the string.
    // it has been constructed so that each 'row' (options of how many
    // broken springs we've seen so far) is one more than the relevant number in `counts`.

    // Initial state: there is 1 way to get to 0 groups done, 0 #s in current run.
    dp[0][0] = 1;

    // Things in the 2d array `dp` move to the right, down, or stay still; so iterate
    // backward to avoid double stepping anything.
    // also, append a '.', to avoid annoying corner cases.
    for &c in springs.as_bytes().iter().chain(b".") {
        for done_counts in (0..=counts.len()).rev() {
            let max_count_here = counts.get(done_counts).copied().unwrap_or_default();
            // assert_eq!(dp[done_counts].len(), max_count_here + 1);
            for so_far in (0..=max_count_here).rev() {
                let count = dp[done_counts][so_far];
                if count > 0 {
                    //consider the case where we've done `done_counts`,
                    //the current block has `so_far`, so far, and we have a c.

                    //Don't usually stay still, so there's no ways (yet) of reaching this state now.
                    dp[done_counts][so_far] = 0;

                    if c != b'.' {
                        //this could extend the current count -> move one space to the right.
                        if so_far < max_count_here {
                            dp[done_counts][so_far + 1] += count;
                        }
                    }
                    if c != b'#' {
                        match so_far {
                            //We haven't seen any #s yet, state remains as is.
                            0 => dp[done_counts][0] += count,
                            //This ends a correctly-lengthed block of #s. Move to next line.
                            x if x == max_count_here => {
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
    // Finally, we want the final state.
    // We already appended a '.' to the input, so all blocks should be finished.
    dp[counts.len()][0]
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
            let counts = repeat_n(counts.into_iter(), N).flatten().collect_vec();
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
