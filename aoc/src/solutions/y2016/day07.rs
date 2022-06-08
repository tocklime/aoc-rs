use aoc_harness::aoc_main;

aoc_main!(2016 day 7, part1 [p1], part2 [p2]);
use itertools::Itertools;

fn has_abba(s: &str) -> bool {
    s.chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn has_tls(l: &&str) -> bool {
    let (outers, inners) = outers_and_inners(l);
    outers.into_iter().any(has_abba) && !inners.into_iter().any(has_abba)
}

fn outers_and_inners(s: &str) -> (Vec<&str>, Vec<&str>) {
    let mut outers = Vec::new();
    let mut inners = Vec::new();
    for (ix, sec) in s.split(|c| c == '[' || c == ']').enumerate() {
        if ix % 2 == 0 {
            outers.push(sec)
        } else {
            inners.push(sec)
        }
    }
    (outers, inners)
}

fn find_abas(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars().tuple_windows().filter_map(
        |(a, b, c)| {
            if a == c && a != b {
                Some((a, b))
            } else {
                None
            }
        },
    )
}

fn has_bab(s: &str, a: char, b: char) -> bool {
    s.chars()
        .tuple_windows()
        .any(|(x, y, z)| x == b && y == a && z == b)
}

fn has_ssl(l: &&str) -> bool {
    let (outers, inners) = outers_and_inners(l);
    outers
        .iter()
        .flat_map(|&i| find_abas(i))
        .any(|(a, b)| inners.iter().any(|&i| has_bab(i, a, b)))
}

fn p1(input: &str) -> usize {
    input.lines().filter(has_tls).count()
}

fn p2(input: &str) -> usize {
    input.lines().filter(has_ssl).count()
}
