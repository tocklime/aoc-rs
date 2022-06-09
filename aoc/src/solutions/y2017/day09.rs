use aoc_harness::aoc_main;

aoc_main!(2017 day 9, generator gen, part1 [p1], part2 [p2]);
#[derive(Debug)]
struct State {
    depth: u32,
    in_garbage: bool,
    ignore_next: bool,
    score: u32,
    total_garbage: u32,
}

fn gen(input: &str) -> State {
    let mut st = State {
        depth: 0,
        in_garbage: false,
        ignore_next: false,
        score: 0,
        total_garbage: 0,
    };
    for c in input.chars() {
        if st.in_garbage {
            if st.ignore_next {
                st.ignore_next = false;
            } else {
                match c {
                    '>' => st.in_garbage = st.ignore_next,
                    '!' => st.ignore_next = true,
                    _ => st.total_garbage += 1,
                }
            }
        } else {
            match c {
                '<' => st.in_garbage = true,
                '{' => st.depth += 1,
                '}' => {
                    st.score += st.depth;
                    st.depth -= 1;
                }
                _ => (),
            }
        }
    }
    st
}

fn p1(input: &State) -> u32 {
    input.score
}

fn p2(input: &State) -> u32 {
    input.total_garbage
}

#[test]
fn day9p1() {
    assert_eq!(p1(&gen("{}")), 1);
    assert_eq!(p1(&gen("{{{}}}")), 6);
    assert_eq!(p1(&gen("{{},{}}")), 5);
    assert_eq!(p1(&gen("{{{},{},{{}}}}")), 16);
    assert_eq!(p1(&gen("{<a>,<a>,<a>,<a>}")), 1);
    assert_eq!(p1(&gen("{{<ab>},{<ab>},{<ab>},{<ab>}}")), 9);
    assert_eq!(p1(&gen("{{<!!>},{<!!>},{<!!>},{<!!>}}")), 9);
    assert_eq!(p1(&gen("{{<a!>},{<a!>},{<a!>},{<ab>}}")), 3);
    assert_eq!(p2(&gen("<>")), 0);
    assert_eq!(p2(&gen("<random characters>")), 17);
    assert_eq!(p2(&gen("<<<<>")), 3);
    assert_eq!(p2(&gen("<{!>}>")), 2);
    assert_eq!(p2(&gen("<!!>")), 0);
    assert_eq!(p2(&gen("<!!!>>")), 0);
    assert_eq!(p2(&gen("<{o\"i!a,<{i<a>")), 10);
}
