use aoc_harness::aoc_main;
use num_modular::{ModularInteger, Montgomery, MontgomeryInt, ReducedInt};
use num_traits::{pow::Pow, Inv};
use std::convert::TryInto;

aoc_main!(2019 day 22, part1 [p1] => 6526, part2 [p2] => 79_855_812_422_607);

type T = ReducedInt<u128, Montgomery<u128, u128>>;

pub fn p1(input: &str) -> usize {
    const DECK_SIZE: u32 = 10007_u32;
    let (offset, increment) = handle_deck(input, DECK_SIZE.into());
    let mut deck = vec![0; DECK_SIZE as usize];
    let mut cur_val = offset;
    for i in 0..DECK_SIZE {
        deck[i as usize] = cur_val.residue();
        cur_val = cur_val + increment;
    }
    deck.iter().enumerate().find(|x| x.1 == &2019).unwrap().0
}

pub fn handle_deck(input: &str, deck_size: u128) -> (T, T) {
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
            let as_u: u128 = n
                .rem_euclid(deck_size.try_into().unwrap())
                .try_into()
                .unwrap();
            offset = offset + (increment * as_u);
        } else if l.trim().starts_with("deal with increment") {
            let n = l.split(' ').nth(3).unwrap().parse::<u128>().expect(
                "int for 
                deal",
            );
            let n = increment.convert(n);
            increment = increment * n.inv();
        } else {
            panic!("Unknown instr: {}", l);
        }
    }
    (offset, increment)
}

pub fn p2(input: &str) -> u128 {
    const DECK_SIZE: u128 = 119_315_717_514_047_u128;
    const SHUFFLE_COUNT: u128 = 101_741_582_076_661_u128;
    let card = 2020;
    let (offset, increment) = handle_deck(input, DECK_SIZE);
    let final_increment = increment.pow(SHUFFLE_COUNT);
    let num = final_increment - 1;
    let denom = (increment - 1).inv();
    let final_offset = num * offset * denom;
    (final_offset + final_increment * card).residue()
}
