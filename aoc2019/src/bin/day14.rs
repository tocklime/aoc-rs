use std::collections::HashMap;

use aoc2019::utils::nums::*;

use itertools::Itertools;

type RecipeBook<'a> = HashMap<&'a str, (usize, Vec<(usize, &'a str)>)>;

pub fn split_pair<'a>(split: &str, input: &'a str) -> (&'a str, &'a str) {
    let x =input.splitn(2, split).collect_vec();
    assert_eq!(x.len(),2);
    (x[0],x[1])
}
pub fn parse_item(i: &str) -> (usize, &str) {
    let (a,b) = split_pair(" ", i.trim());
    (a.parse().unwrap(),b)
}
pub fn mk_rb(input: &str) -> RecipeBook {
    input
        .trim()
        .lines()
        .map(|l| {
            let (ing,out) = split_pair("=>", l);
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
        let (&lets_make, qty_needed) = required.iter().nth(0).unwrap();
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

//#[aoc(day14, part1)]
pub fn p1(input: &str) -> usize {
    ore_for_n_fuel(&mk_rb(input), 1)
}
//#[aoc(day14, part2)]
pub fn p2(input: &str) -> usize {
    let recipes = mk_rb(input);
    unbounded_bin_search(&|x| ore_for_n_fuel(&recipes, x), 1_000_000_000_000)
}
