use aoc2019::utils::nums::{mod_add, mod_inv, mod_mul, mod_pow};
use std::convert::TryInto;

//#[aoc(day22, part1)]
pub fn p1(input: &str) -> usize {
    let card_count = 10007_u32;
    let (offset, increment) = handle_deck(input, card_count.try_into().unwrap());
    let mut deck = vec![0; card_count as usize];
    let mut cur_val = offset;
    for i in 0..card_count {
        deck[i as usize] = cur_val % 10007;
        cur_val += increment
    }
    deck.iter().enumerate().find(|x| x.1 == &2019).unwrap().0
}
pub fn handle_deck(input: &str, deck_size: u128) -> (u128, u128) {
    let mut offset = 0_u128;
    let mut increment = 1_u128;
    for l in input.trim().lines() {
        //println!("Deck now {:?} {:?}   {}", offset, increment, l);
        if l.trim().starts_with("deal into new stack") {
            increment = mod_mul(&increment, &(deck_size - 1), deck_size);
            offset = mod_add(&increment, &offset, deck_size);
        } else if l.trim().starts_with("cut") {
            let n = l
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<i128>()
                .expect("int for cut");
            let as_u = n
                .rem_euclid(deck_size.try_into().unwrap())
                .try_into()
                .unwrap();
            offset = mod_add(&offset, &mod_mul(&increment, &as_u, deck_size), deck_size);
        } else if l.trim().starts_with("deal with increment") {
            let n = l
                .split(' ')
                .nth(3)
                .unwrap()
                .parse::<u128>()
                .expect("int for deal");
            increment = mod_mul(&increment, &mod_inv(n, deck_size), deck_size);
        } else {
            panic!("Unknown instr: {}", l);
        }
    }
    (offset, increment)
}
//#[aoc(day22, part2)]
pub fn p2(input: &str) -> u128 {
    let deck_size = 119_315_717_514_047_u128;
    let shuffle_count = 101_741_582_076_661_u128;
    let card = 2020;
    let (offset, increment) = handle_deck(input, deck_size);
    let final_increment = mod_pow(increment, shuffle_count, deck_size);
    let num = final_increment - 1;
    let denom = mod_inv(increment - 1, deck_size);
    let final_offset = mod_mul(&mod_mul(&offset, &num, deck_size), &denom, deck_size);
    mod_add(
        &final_offset,
        &mod_mul(&final_increment, &card, deck_size),
        deck_size,
    )
}
