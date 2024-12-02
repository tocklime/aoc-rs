use itertools::Itertools;

aoc_harness::aoc_main!(2024 day 2, generator gen, part1 [p1] => 463, part2 [p2] => 514, example part1 EG => 2, example part2 EG => 4);

fn p1(input: &[Vec<usize>]) -> usize {
    input.iter().filter(|l|find_error_ix(l.iter()).is_none()).count()
}

fn p2(input: &[Vec<usize>]) -> usize {
    input.iter().filter(|x| is_safe_by_skipping(x)).count()
}

fn gen(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn is_safe_by_skipping(line: &[usize]) -> bool {
    match find_error_ix(line.iter()) {
        //It's ok as is!
        None => true,
        //Error at n. try ignoring the item before, itself or the one after.
        Some(n) =>
            (n.saturating_sub(1)..n+2).any(|x| find_error_ix(line.iter().take(x).chain(line.iter().skip(x+1))).is_none())
    }
}

fn find_error_ix<'a>(line: impl Iterator<Item = &'a usize>) -> Option<usize> {
    let mut dir = None;
    for (ix, (&a, &b)) in line.tuple_windows().enumerate() {
        let c = a.cmp(&b);
        match (dir, c) {
            (None, _) => dir = Some(c),
            (Some(a), b) if a != b => return Some(ix),
            _ => (),
        };
        if !(1..=3).contains(&a.abs_diff(b)) {
            return Some(ix)
        }
    }
    None
}

const EG: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
