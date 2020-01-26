fn calc(l: char, c: char, r: char) -> char {
    let l = l == '^';
    let c = c == '^';
    let r = r == '^';
    if l && c && !r
        || !l && c && r
        || l && !c && !r
        || !l && !c && r { '^' } else { '.' }
}

fn next_line(input: &[char]) -> Vec<char> {
    (0..input.len()).map(|x| {
        let l = if x > 0 {input.get(x - 1).unwrap_or(&'.')} else {&'.'};
        let c = input.get(x).unwrap_or(&'.');
        let r = input.get(x + 1).unwrap_or(&'.');
        calc(*l, *c, *r)
    }).collect()
}

#[aoc(day18, part1)]
fn p1(input: &str) -> usize {
    solve(input,40)
}
#[aoc(day18, part2)]
fn p2(input: &str) -> usize {
    solve(input,400000)
}
fn solve(input: &str, n: usize) -> usize {
    let mut row: Vec<char> = input.trim().chars().collect();
    let mut safe_count = row.iter().filter(|x| x == &&'.').count();
    for _ in 0..n-1 {
        let next = next_line(&row);
        safe_count += next.iter().filter(|x| x == &&'.').count();
        row = next;
    }
    safe_count
}


#[test]
fn test18() {
    use itertools::Itertools;
    assert_eq!(next_line(&".^^.^.^^^^".chars().collect_vec()),"^^^...^..^".chars().collect_vec())
}