
use num_modular::{ModularInteger, Montgomery, MontgomeryInt, ReducedInt};
use std::convert::TryInto;

aoc_harness::aoc_main!(2019 day 22, part1 [p1] => 6526, part2 [p2] => 79_855_812_422_607);

type NT = u64;
type T = ReducedInt<NT, Montgomery<NT>>;

pub fn handle_deck(input: &str, deck_size: NT) -> (T, T) {
    let mut offset = MontgomeryInt::new(0, &deck_size);
    let mut increment = offset.convert(1);
    for l in input.trim().lines() {
        //println!("Deck now {:?} {:?}   {}", offset, increment, l);
        if l.trim().starts_with("deal into new stack") {
            increment = increment * (deck_size - 1);
            offset = offset + increment;
        } else if l.trim().starts_with("cut") {
            let n = l
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<i128>()
                .expect("int for cut");
            let as_u: NT = n
                .rem_euclid(deck_size.try_into().unwrap())
                .try_into()
                .unwrap();
            offset = offset + (increment * as_u);
        } else if l.trim().starts_with("deal with increment") {
            let n = l.split(' ').nth(3).unwrap().parse::<NT>().expect(
                "int for 
                deal",
            );
            let n = increment.convert(n);
            increment = increment * n.inv().unwrap();
        } else {
            panic!("Unknown instr: {l}");
        }
    }
    (offset, increment)
}

pub fn p1(input: &str) -> u32 {
    const DECK_SIZE: u32 = 10007_u32;
    const CARD: NT = 2019;
    let (offset, increment) = handle_deck(input, DECK_SIZE.into());
    let mut cur_val = offset;
    (1..DECK_SIZE)
        .find(|_| {
            cur_val = cur_val + increment;
            cur_val.residue() == CARD
        })
        .unwrap()
}

pub fn p2(input: &str) -> NT {
    const DECK_SIZE: NT = 119_315_717_514_047;
    const SHUFFLE_COUNT: NT = 101_741_582_076_661;
    const CARD: NT = 2020;
    let (offset_one, increment_one) = handle_deck(input, DECK_SIZE);
    let increment_final = increment_one.pow(&SHUFFLE_COUNT);
    let offset_final = (increment_final - 1) * offset_one * (increment_one - 1).inv().unwrap();
    (offset_final + increment_final * CARD).residue()
}
