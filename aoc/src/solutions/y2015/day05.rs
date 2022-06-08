use aoc_harness::aoc_main;

aoc_main!(2015 day 5, part1 [p1], part2 [p2]);
use itertools::Itertools;
use std::collections::HashMap;

const BAD: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn is_nice(input: &str) -> bool {
    //3 vowels
    //double
    //none of ab, cd, pq, xy
    let vowels = input.chars().filter(|x| "aeiou".contains(*x)).count();
    let doubles = input.chars().zip(input.chars().skip(1))
        .filter(|&(a, b)| a == b).count();
    let any_bad = BAD.iter().filter(|t| input.contains(*t)).count();
    vowels >= 3 && doubles > 0 && any_bad == 0
}

fn is_nice_2(input: &str) -> bool {
    //repeating (separate) pair
    //xyx pattern.
    let as_vec = input.chars().collect_vec();
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    let mut found_pair = false;
    let mut found_xyx = false;
    for i in 0..as_vec.len() {
        let a = as_vec[i];
        if i < as_vec.len() - 1 {
            let b = as_vec[i + 1];
            let ab = (a, b);
            if let Some(&cd) = pairs.get(&ab) {
                if (i - cd) > 1 {
                    found_pair = true;
                }
            }else {
                pairs.insert(ab, i);
            }
        }
        if i < as_vec.len() - 2 {
            let c = as_vec[i + 2];
            if a == c {
                found_xyx = true;
            }
        }
    }
    found_pair && found_xyx
}


fn p1(input: &str) -> usize {
    input.lines().filter(|x| is_nice(*x)).count()
}


fn p2(input: &str) -> usize {
    input.lines().filter(|x| is_nice_2(*x)).count()
}


#[test]
fn p2_test() {
    assert_eq!(is_nice_2("aaa"),false);
    assert_eq!(is_nice_2("qjhvhtzxzqqjkmpb"), true);// is nice
    assert_eq!(is_nice_2("xxyxx"), true);
    assert_eq!(is_nice_2("uurcxstgmygtbstg"), false);
    assert_eq!(is_nice_2("ieodomkazucvgmuy"), false);
}
