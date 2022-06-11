aoc_harness::aoc_main!(2018 day 5, part1 [p1], part2 [p2]);

fn react(a: char, b: char) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase() && a != b
}

fn reduce(input: &str) -> String {
    let mut output : Vec<char> = Vec::new();
    for c in input.chars(){
        match (output.last(),c) {
            (Some(&x),c) if react(x,c) => {output.pop();}
            _ => output.push(c)
        }
    }
    output.iter().collect()
}


fn p1(input: &str) -> usize {
    reduce(input.trim()).len()
}


fn p2(input: &str) -> usize {
    (b'a'..b'z').map(|c| {
        let new_input : String = input.trim().chars().filter(|x| x.to_ascii_lowercase() != char::from(c)).collect();
        reduce(&new_input).len()
    }).min().unwrap()
}


#[test]
fn d5p1_test() {
    assert_eq!(reduce("dabAcCaCBAcCcaDA"),"dabCBAcaDA");
}