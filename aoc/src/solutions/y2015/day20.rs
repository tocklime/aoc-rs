use aoc_harness::aoc_main;

aoc_main!(2015 day 20, part1 [p1], part2 [p2]);
use primal;
use std::iter;
use itertools::Itertools;
use std::collections::HashSet;

fn presents(s: &primal::Sieve, n: usize) -> usize {
    let f = s.factor(n).unwrap();
    let sum: usize = f.iter()
        .map(|&(a, b)|
            (usize::pow(a, b as u32 + 1) - 1) / (a - 1)
        ).product();
    //println!("n: {}, prime_factors: {:?}, sum: {}",n,f,sum);
    sum * 10
}


fn p1(input: &str) -> usize {
    let target = input.trim().parse::<usize>().unwrap();
    let sieve = primal::Sieve::new(target);
    (1..).find(|&h| presents(&sieve, h) >= target).unwrap()
}

fn presents2(s: &primal::Sieve, n: usize) -> usize {
    let f = s.factor(n).unwrap();
    let factors = f.iter().flat_map(|&(a, b)| iter::repeat(a).take(b)).collect_vec();
    let all_divs: HashSet<usize> = (0..=factors.len())
        .flat_map(|s| factors.iter().combinations(s)
            .map(|n| n.iter().map(|&&x| x).product())
        ).collect();
    let filtered = all_divs.iter()
        .filter(|&&d| n / d <= 50)
        .collect_vec();
    let sum: usize = filtered.iter().map(|x| **x).sum();
    sum * 11
}


fn p2(input: &str) -> usize {
    let target = input.trim().parse::<usize>().expect("Bad input");
    let sieve = primal::Sieve::new(target);
    (1..).find(|&h| presents2(&sieve, h) >= target).unwrap()
}

#[test]
fn day20p1tests() {
    let s = primal::Sieve::new(10000000);
    assert_eq!(presents(&s, 1), 10);
    assert_eq!(presents(&s, 2), 30);
    assert_eq!(presents(&s, 3), 40);
    assert_eq!(presents(&s, 4), 70);
    assert_eq!(presents(&s, 5), 60);
    assert_eq!(presents(&s, 6), 120);
    assert_eq!(presents(&s, 7), 80);
    assert_eq!(presents(&s, 8), 150);
    assert_eq!(presents(&s, 9), 130);
    assert_eq!(presents(&s, 210), 5760);
    assert_eq!(presents(&s, 1274999), 14_361_600);
}

//1067430 too high