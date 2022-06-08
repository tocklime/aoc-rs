fn calc(l: char, r: char) -> char {
    let l = l == '^';
    let r = r == '^';
    if l && !r || !l && r
    { '^' } else { '.' }
}

fn next_line(input: &[char]) -> Vec<char> {
    (0..input.len()).map(|x| {
        let l = if x > 0 { input.get(x - 1).unwrap_or(&'.') } else { &'.' };
        let r = input.get(x + 1).unwrap_or(&'.');
        calc(*l, *r)
    }).collect()
}



fn p1(input: &str) -> usize {
    solve(input, 40)
}



fn p2(input: &str) -> usize {
    solve(input, 400_000)
}

fn solve(input: &str, n: usize) -> usize {
    let mut row: Vec<char> = input.trim().chars().collect();
    let mut safe_count = row.iter().filter(|x| x == &&'.').count();
    for _ in 0..n - 1 {
        let next = next_line(&row);
        safe_count += next.iter().filter(|x| x == &&'.').count();
        row = next;
    }
    safe_count
}


#[test]
fn test18() {
    use itertools::Itertools;
    assert_eq!(next_line(&".^^.^.^^^^".chars().collect_vec()), "^^^...^..^".chars().collect_vec())
}