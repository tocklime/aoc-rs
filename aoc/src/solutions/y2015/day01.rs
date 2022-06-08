use aoc_harness::aoc_main;

aoc_main!(2015 day 1, part1 [p1], part2 [p2,p2a]);


fn p1(input: &str) -> i32 {
    input.trim().chars().map(|x| if x == '(' {1} else {-1} ).sum()
}


fn p2(input: &str) -> usize {
    input.trim().chars().map(|x| if x == '(' {1} else {-1}).scan(0,|a,b| {
        *a += b;
        Some(*a)
    }).zip(1..)
        .find(|&t| t.0 == -1)
        .unwrap().1
}

fn p2a(input: &str) -> usize {
    let mut floor = 0;
    for (p,c) in input.trim().chars().enumerate() {
        match c {
            '(' => floor += 1,
            _ => floor -= 1,
        }
        if floor == -1 {
            return p+1;
        }
    }
    0
}
