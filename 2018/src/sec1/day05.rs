
fn reduce(input: &str) -> String {
    let mut output = Vec::new();
    for c in input.chars() {
        let top = output.last();
        if top != Some(&c) && Some(c.to_ascii_lowercase()) == top.map(|c| c.to_ascii_lowercase()) {
            output.pop();
        }else {
            output.push(c);
        }
    }
    output.iter().collect()
}

#[aoc(day5,part1)]
fn p1(input: &str) -> usize {
    reduce(input).len()
}

#[aoc(day5, part2)]
fn p2(input: &str) -> usize {
    (b'a'..=b'z').map(|b| {
        let c = char::from(b);
        let filtered : String = input.chars().filter(|x| x.to_ascii_lowercase() != c).collect();
        reduce(&filtered).len()
    }).min().unwrap()
}