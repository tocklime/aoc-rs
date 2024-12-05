aoc_harness::aoc_main!(2024 day 3, part1 [p1] => 187_194_524, part2 [p2] => 127_092_535, example part1 EG => 161, example part2 EG2 => 48);

fn parse_num(input: &str) -> Option<(usize, &str)> {
    let first_non_numeric = input.find(|x: char| !x.is_numeric())?;
    if first_non_numeric == 0 {
        None
    } else {
        let num = input[0..first_non_numeric].parse().unwrap();
        Some((num, &input[first_non_numeric..]))
    }
}
fn token<'a>(input: &'a str, token: &'static str) -> Option<&'a str> {
    input.strip_prefix(token)
}
fn parse_mul(input: &str) -> Option<(usize, &str)> {
    let i = token(input, "mul(")?;
    let (num_a, i) = parse_num(i)?;
    let i = token(i, ",")?;
    let (num_b, i) = parse_num(i)?;
    let i = token(i, ")")?;
    Some((num_a * num_b, i))
}
fn parse_do_dont(input: &str) -> Option<(bool, &str)> {
    match token(input, "don't()") {
        Some(i) => Some((false, i)),
        None => token(input, "do()").map(|x| (true, x)),
    }
}
fn p1(input: &str) -> usize {
    let mut i = input;
    let mut sum = 0;
    while !i.is_empty() {
        match parse_mul(i) {
            Some((n, new_i)) => {
                sum += n;
                i = new_i;
            }
            None => i = &i[1..],
        }
    }
    sum
}
fn p2(input: &str) -> usize {
    let mut i = input;
    let mut sum = 0;
    let mut enabled = true;
    while !i.is_empty() {
        match parse_do_dont(i) {
            Some((b, new_i)) => {
                enabled = b;
                i = new_i;
            }
            None => {
                if enabled {
                    match parse_mul(i) {
                        Some((n, new_i)) => {
                            // println!("Got {n} at {i}");
                            sum += n;
                            i = new_i;
                        }
                        None => {
                            i = &i[1..];
                        }
                    }
                } else {
                    i = &i[1..];
                }
            }
        }
    }
    sum
}

const EG: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const EG2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
