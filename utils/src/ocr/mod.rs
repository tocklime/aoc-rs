use std::collections::HashSet;
use std::hash::Hash;
use std::ops::RangeInclusive;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use lazy_static::lazy_static;
use num::traits::WrappingSub;
use num::{Num, PrimInt};

use crate::aabb::Aabb;
use crate::cartesian::Point;
use crate::numset::NumSet;
const ALPHA_6_4: &str = include_str!("6x4.txt");
const ALPHA_10_6: &str = include_str!("10x6.txt");
lazy_static! {
    static ref ALPHA_6_4_MAP: HashMap<NumSet<u32>, char> = make_map::<u32>(ALPHA_6_4, 4, 1);
    static ref ALPHA_10_6_MAP: HashMap<NumSet<u64>, char> = make_map::<u64>(ALPHA_10_6, 6, 1);
}
fn convert_art_to_sets<T: PrimInt>(ascii_art: &str, char_width: u8, whitespace_between: u8) -> Vec<NumSet<T>> {
    let wid = char_width + whitespace_between;
    let min_leading_blanks: usize = ascii_art
        .lines()
        .map(|x| x.chars().take_while(|c| char::is_whitespace(*c)).count())
        .min()
        .unwrap();
    let mut sets = Vec::new();
    for (l, y) in ascii_art.lines().zip(0..) {
        for (c, x) in l.chars().skip(min_leading_blanks).zip(0..) {
            if !char::is_whitespace(c) {
                let char_pos: usize = (x / wid).into();
                let char_x = x % wid;
                let bit_pos: u8 = y * char_width + char_x;
                while sets.len() < char_pos + 1 {
                    sets.push(NumSet::new());
                }
                sets[char_pos].insert(bit_pos);
            }
        }
    }
    sets
}
fn make_map<T>(alphabet_ascii_art: &str, char_width: u8, whitespace_between: u8) -> HashMap<NumSet<T>, char>
where
    T: PrimInt + Hash,
{
    convert_art_to_sets(alphabet_ascii_art, char_width,whitespace_between)
        .into_iter()
        .zip('A'..='Z')
        .collect()
}
#[must_use]
pub fn point_cloud_to_str<N, S: ::std::hash::BuildHasher>(set: &HashSet<Point<N>, S>) -> String
where
    N: WrappingSub + Hash + PrimInt + TryInto<usize> + Num + Debug,
    RangeInclusive<N>: std::iter::Iterator<Item = N>,
    <N as TryInto<usize>>::Error: Debug,
{
    const CHAR_WIDTH: usize = 4;
    let bb: Aabb<N> = set.iter().collect();
    let uwidth: usize = bb.width();
    let mut sets = vec![NumSet::new(); 1 + (uwidth / (CHAR_WIDTH + 1))];
    for &Point { x, y } in set {
        let ux: usize = x.try_into().unwrap();
        let uy: usize = y.try_into().unwrap();
        let char_pos: usize = ux / (CHAR_WIDTH + 1);
        let char_x = ux % (CHAR_WIDTH + 1);
        let bit_pos: u8 = (uy * CHAR_WIDTH + char_x).try_into().unwrap();
        sets[char_pos].insert(bit_pos);
    }
    sets.into_iter()
        .map(|c| ALPHA_6_4_MAP.get(&c).copied().unwrap_or('?'))
        .collect()
}
#[must_use]
pub fn ascii_art_4_to_str(input: &str) -> String {
    convert_art_to_sets(input.trim_matches('\n'), 4, 1)
        .into_iter()
        .map(|c| ALPHA_6_4_MAP.get(&c).copied().unwrap_or('?'))
        .collect()
}
#[must_use]
pub fn ascii_art_6_to_str(input: &str) -> String {
    convert_art_to_sets(input.trim_matches('\n'), 6, 2)
        .into_iter()
        .map(|c| ALPHA_10_6_MAP.get(&c).copied().unwrap_or('?'))
        .collect()
}
#[derive(PartialEq, Eq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct OcrString {
    inner: String,
}
impl aoc_harness::answertype::AnswerType for OcrString {
    type Output = Self;
    fn to_option(self) -> Option<Self::Output> {
        Some(self)
    }
}
impl From<String> for OcrString {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl OcrString {
    #[must_use]
    pub fn ocr(&self) -> String {
        ascii_art_4_to_str(&self.inner)
    }
    #[must_use]
    pub fn new(inner: String) -> Self {
        Self { inner }
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
