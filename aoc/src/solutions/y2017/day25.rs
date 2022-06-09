use aoc_harness::aoc_main;

aoc_main!(2017 day 25, part1 [p1]);
use std::collections::HashSet;

fn p1(_input: &str) -> usize {
    let mut state = 'A';
    let c = 12667664;
    let mut pos = 0;
    let mut tape: HashSet<i64> = HashSet::new();
    for _ in 0..c {
        let (set, mv, st2) = match (state, tape.contains(&pos)) {
            ('A', false) => (true, 1, 'B'),
            ('A', true) => (false, -1, 'C'),
            ('B', false) => (true, -1, 'A'),
            ('B', true) => (true, 1, 'D'),
            ('C', false) => (false, -1, 'B'),
            ('C', true) => (false, -1, 'E'),
            ('D', false) => (true, 1, 'A'),
            ('D', true) => (false, 1, 'B'),
            ('E', false) => (true, -1, 'F'),
            ('E', true) => (true, -1, 'C'),
            ('F', false) => (true, 1, 'D'),
            ('F', true) => (true, 1, 'A'),
            (a, _) => panic!("Unknown state {}", a),
        };
        if set {
            tape.insert(pos);
        } else {
            tape.remove(&pos);
        }
        pos += mv;
        state = st2;
    }
    tape.len()
}
