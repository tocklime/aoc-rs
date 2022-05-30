use std::collections::HashMap;

use utils::nums::*;

use aoc_harness::aoc_main;
use itertools::Itertools;

aoc_main!(2019 day 14, part1 [p1] => 892207, part2 [p2] => 1935265,
example part1 E1 => 31,
example part1 E2 => 165,
example both E3 => (13312, 82892753));
type RecipeBook<'a> = HashMap<&'a str, (usize, Vec<(usize, &'a str)>)>;

pub fn split_pair<'a>(split: &str, input: &'a str) -> (&'a str, &'a str) {
    let x = input.splitn(2, split).collect_vec();
    assert_eq!(x.len(), 2);
    (x[0], x[1])
}
pub fn parse_item(i: &str) -> (usize, &str) {
    let (a, b) = split_pair(" ", i.trim());
    (a.parse().unwrap(), b)
}
pub fn mk_rb(input: &str) -> RecipeBook {
    input
        .trim()
        .lines()
        .map(|l| {
            let (ing, out) = split_pair("=>", l);
            let (q, output) = parse_item(out);
            let input = ing.split(',').map(parse_item).collect_vec();
            (output, (q, input))
        })
        .collect()
}

pub fn ore_for_n_fuel(recipes: &RecipeBook, n: usize) -> usize {
    let mut required: HashMap<&str, usize> = HashMap::new();
    let mut hold: HashMap<&str, usize> = HashMap::new();
    required.insert("FUEL", n);
    let mut ore_used = 0;
    while !required.is_empty() {
        let (&lets_make, qty_needed) = required.iter().next().unwrap();
        let (qty_per, ingredients) = &recipes[lets_make];
        let left_over = qty_needed % qty_per;
        let iterations_required = (qty_needed / qty_per) + if left_over > 0 { 1 } else { 0 };
        for (q, reagent) in ingredients {
            let needed = q * iterations_required;
            if *reagent == "ORE" {
                ore_used += needed;
            } else {
                let available = *hold.get(reagent).unwrap_or(&0);
                if available > needed {
                    *hold.get_mut(reagent).unwrap() -= needed;
                } else {
                    hold.remove(reagent);
                    *required.entry(reagent).or_insert(0) += needed - available;
                }
            }
        }
        if left_over > 0 {
            *hold.entry(lets_make).or_default() += qty_per - left_over;
        }
        required.remove(lets_make);
    }
    ore_used
}

pub fn p1(input: &str) -> usize {
    ore_for_n_fuel(&mk_rb(input), 1)
}

pub fn p2(input: &str) -> usize {
    let recipes = mk_rb(input);
    unbounded_bin_search(&|x| ore_for_n_fuel(&recipes, x), 1_000_000_000_000)
}

const E1: &str = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

const E2: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

const E3: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
