use nom::{
    character::complete::{self, newline, one_of},
    combinator::all_consuming,
    multi::separated_list1,
    Parser,
};
use num_traits::Euclid;
use utils::nom::NomError;

aoc_harness::aoc_main!(2025 day 1, generator gen, part1 [p1], part2 [p2_slow, p2], example part1 EG => 3, example part2 EG => 6, example part2 EG2 => 4);

fn gen(input: &str)-> Vec<i64> {
    all_consuming::<_, NomError, _>(separated_list1(
        newline,
        (
            one_of("LR").map(|c| if c == 'L' { -1 } else { 1 }),
            complete::i64,
        ).map(|(a,b)| a*b),
    ))
    .parse(input.trim_end()).unwrap().1
}

fn p1(ns: &[i64]) -> usize {
    ns.into_iter().scan(50,|state, dir| {
        *state += dir;
        *state %= 100;
        Some(*state)
    }).filter(|&x| x == 0).count()
}
fn turn_dial_slow(mut state: i64, direction: i64) -> (usize, i64) {
    let mut wraps = 0;
    let sign = direction.signum();
    for _ in 0..direction.abs() {
        state += sign;
        state = state.rem_euclid(100);
        if state == 0 {
            wraps +=1;
        }
    }
    (wraps, state)
}

fn turn_dial(state: i64, direction: i64) -> (usize, i64) {
    let new_s = state + direction;
    let (wraps, remmed) = new_s.div_rem_euclid(&100);
    let mut wraps = wraps.abs() as usize;
    if direction < 0 {
        if remmed == 0 {
            //ended at zero, but it won't have been counted.
            wraps += 1;
        }
        if state == 0 {
            //started at zero, but it will have been counted.
            wraps -= 1;
        }
    }
    (wraps, remmed)
}
fn p2_slow(ns:&[i64]) -> usize {
    ns.into_iter().scan(50,|state, &dir| {
        let (wraps, new_s) = turn_dial_slow(*state, dir);
        *state = new_s;
        Some(wraps)
    }).sum()
}
fn p2(ns:&[i64]) -> usize {
    ns.into_iter().scan(50,|state, &dir| {
        let (wraps, new_s) = turn_dial(*state, dir);
        *state = new_s;
        Some(wraps)
    }).sum()
}

const EG: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
const EG2: &str = "R41\nL391";

#[cfg(test)]
mod test {
    use proptest::proptest;
    use super::*;
    proptest!{
        #[test]
        fn smart_is_equiv_to_slow(dial in 0..100i64, direction in -1000..1000i64) {
            assert_eq!(turn_dial(dial, direction), turn_dial_slow(dial, direction))
        }

    }
}