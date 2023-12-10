

aoc_harness::aoc_main!(2015 day 4, part1 [p1] => 346_386, part2 [p2] => 9_958_218, example part1 EG => 609_043, example part1 EG2 => 1_048_970);

fn p1(input: &str) -> usize {
    let input = input.trim();
    #[allow(clippy::maybe_infinite_iter)]
    (0..)
        .find(|i| {
            let str = format!("{input}{i}");
            let md5 = md5::compute(str);
            md5[0] == 0 && md5[1] == 0 && (md5[2] & 0xF0) == 0
        })
        .unwrap()
}

fn p2(input: &str) -> usize {
    let input = input.trim();
    #[allow(clippy::maybe_infinite_iter)]
    (0..)
        .find(|i| {
            let str = format!("{input}{i}");
            let md5 = md5::compute(str);
            md5[0] == 0 && md5[1] == 0 && md5[2] == 0
        })
        .unwrap()
}

const EG: &str = "abcdef";
const EG2: &str = "pqrstuv";