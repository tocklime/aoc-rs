use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use itertools::Itertools;
use num::Integer;

use crate::numset::NumSet;

// read something like:
// #  # #### #### ####  ##  #  #  ##    ##
// #  # #    #       # #  # #  # #  #    #
// #  # ###  ###    #  #    #  # #       #
// #  # #    #     #   #    #  # #       #
// #  # #    #    #    #  # #  # #  # #  #
//  ##  #### #    ####  ##   ##   ##   ##
// or
// #    #  # ###  #### ###  ###  ###  #  #
// #    # #  #  # #    #  # #  # #  # # #
// #    ##   #  # ###  ###  #  # #  # ##
// #    # #  ###  #    #  # ###  ###  # #
// #    # #  # #  #    #  # #    # #  # #
// #### #  # #  # #### ###  #    #  # #  #
//and make the string of it.
const ALPHABET_6: [(char, &str); 18] = [
    ('A', ".##.\n#..#\n#..#\n####\n#..#\n#..#"),
    ('B', "###.\n#..#\n###.\n#..#\n#..#\n###."),
    ('C', ".##.\n#..#\n#...\n#...\n#..#\n.##."),
    ('E', "####\n#...\n###.\n#...\n#...\n####"),
    ('F', "####\n#...\n###.\n#...\n#...\n#..."),
    ('G', ".##.\n#..#\n#...\n#.##\n#..#\n.###"),
    ('H', "#..#\n#..#\n####\n#..#\n#..#\n#..#"),
    ('I', ".###\n..#.\n..#.\n..#.\n..#.\n.###"),
    ('J', "..##\n...#\n...#\n...#\n#..#\n.##."),
    ('K', "#..#\n#.#.\n##..\n#.#.\n#.#.\n#..#"),
    ('L', "#...\n#...\n#...\n#...\n#...\n####"),
    ('O', ".##.\n#..#\n#..#\n#..#\n#..#\n.##."),
    ('P', "###.\n#..#\n#..#\n###.\n#...\n#..."),
    ('R', "###.\n#..#\n#..#\n###.\n#.#.\n#..#"),
    ('S', ".###\n#...\n#...\n.##.\n...#\n###."),
    ('U', "#..#\n#..#\n#..#\n#..#\n#..#\n.##."),
    ('Y', "#...\n#...\n.#.#\n..#.\n..#.\n..#."),
    ('Z', "####\n...#\n..#.\n.#..\n#...\n####"),
];
fn get_dict() -> HashMap<NumSet<u32>, char> {
    ALPHABET_6
        .iter()
        .map(|&(c, s)| (map_char(s, '#'), c))
        .collect()
}
fn map_char(input: &str, c: char) -> NumSet<u32> {
    let l = input.lines().map(|x| x.chars().collect_vec()).collect_vec();
    map_char_vec(&l, c)
}
fn map_char_vec(l: &[Vec<char>], c: char) -> NumSet<u32> {
    (0_u8..30)
        .filter(|&p| {
            let (y, x) = p.div_mod_floor(&5);
            Some(&c) == l[y as usize].get(x as usize)
        })
        .collect()
}
#[must_use]
pub fn ocr(input: &str, c: char) -> String {
    let lookup: HashMap<NumSet<u32>, char> = get_dict();
    let grid = input
        .trim_matches('\n')
        .lines()
        .map(|x| x.chars().collect_vec())
        .collect_vec();
    let mut ans = String::new();
    for c_ix in 0..=grid[0].len() / 5 {
        let this_pattern = grid
            .iter()
            .map(|l| l.iter().skip(c_ix * 5).take(5).copied().collect_vec())
            .collect_vec();
        let this_val = map_char_vec(&this_pattern, c);
        ans.push(lookup.get(&this_val).copied().unwrap_or('?'));
    }
    ans
}

#[derive(PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub struct OcrString {
    inner: String,
    c: char,
}

impl OcrString {
    #[must_use]
    pub fn ocr(&self) -> String {
        ocr(&self.inner, self.c)
    }
    #[must_use]
    pub fn new(inner: String, c: char) -> Self {
        Self { inner, c }
    }
}
impl Debug for OcrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.ocr()))
    }
}
impl Display for OcrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.ocr()))
    }
}

impl PartialEq<&str> for OcrString {
    fn eq(&self, other: &&str) -> bool {
        let x = self.ocr();
        &x == other
    }
}
