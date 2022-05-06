use aoc_harness::aoc_main;
use counter::Counter;
use itertools::Itertools;
use std::cmp::max;
use transpose::transpose_inplace;
use utils::ocr::OcrString;

aoc_main!(2019 day 8, part1 [p1] => 2460, part2 [p2_forloop, p2_transpose, p2_steps] => "LRFKU");

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const IM_SIZE: usize = WIDTH * HEIGHT;

fn conv(c: char) -> char {
    match c {
        '1' => '█',
        '0' => ' ',
        '2' => 'X',
        _ => panic!("Unknown pixel"),
    }
}

//#[aoc(day8, part1)]
pub fn p1(input: &str) -> usize {
    let fewest0 = input
        .trim()
        .chars()
        .chunks(IM_SIZE)
        .into_iter()
        .map(std::iter::Iterator::collect::<Counter<_>>)
        .min_by_key(|x| x[&'0'])
        .unwrap();
    fewest0[&'1'] * fewest0[&'2']
}

pub fn p2_forloop(input: &str) -> OcrString {
    let layers = input.trim().chars().map(conv).chunks(IM_SIZE);
    let mut image = vec![vec!['X'; WIDTH]; HEIGHT];
    for l in &layers {
        for (p, c) in l.enumerate() {
            let x = p % WIDTH;
            let y = p / WIDTH;
            if image[y][x] == 'X' && c != 'X' {
                image[y][x] = c;
            }
        }
    }
    image
        .iter()
        .map(|x| format!("\n{}", x.iter().format("")))
        .join("")
        .into()
}

pub fn p2_transpose(input: &str) -> OcrString {
    let mut pixels: Vec<_> = input.trim().chars().map(conv).collect();
    let layer_count = pixels.len() / IM_SIZE;
    let mut scratch = vec!['X'; max(IM_SIZE, layer_count)];
    transpose_inplace(&mut pixels, &mut scratch, IM_SIZE, layer_count);
    pixels
        .chunks(layer_count)
        .map(|l| l.iter().cloned().find(|&x| x != 'X').unwrap())
        .chunks(WIDTH)
        .into_iter()
        .map(|x| format!("\n{}", x.format("")))
        .join("")
        .into()
}

pub fn p2_steps(input: &str) -> OcrString {
    let pixels: Vec<_> = input.trim().chars().map(conv).collect();
    (0..IM_SIZE)
        .map(|p| {
            pixels
                .iter()
                .skip(p)
                .step_by(IM_SIZE)
                .find(|&x| *x != 'X')
                .unwrap()
        })
        .chunks(WIDTH)
        .into_iter()
        .map(|x| format!("\n{}", x.format("")))
        .join("")
        .into()
}
