use rayon::prelude::*;
use std::convert::TryInto;
use std::cmp::{max,min};

//#[aoc_generator(day16)]
pub fn gen(input: &str) -> Vec<usize> {
    input.trim().bytes().map(|x| (x - b'0').try_into().unwrap()).collect()
}
//#[aoc(day16, part1)]
pub fn p1(input: &[usize]) -> usize {
    let mut x = input.to_vec();
    (0..100).for_each(|_| x = fft(&x));
    list_to_int(&x[..8])
}
pub fn list_to_int(l: &[usize]) -> usize {
    l.iter().fold(0, |n, &d| 10 * n + d)
}
//#[aoc(day16, part2, second_half_optimisation)]
pub fn p2(input: &[usize]) -> usize {
    let offset = list_to_int(&input[..7]);
    assert!(offset > (input.len() * 10_000 / 2));
    let mut input: Vec<_> = input
        .iter()
        .cycle()
        .take(input.len() * 10000)
        .skip(offset)
        .cloned()
        .collect();
    for _ in 0..100 {
        let mut sum = input.iter().sum::<usize>();
        for i in &mut input {
            let tmp = *i;
            *i = sum % 10;
            sum -= tmp;
        }
    }
    list_to_int(&input[..8])
}
////#[aoc(day16, part2, naive)]
pub fn p2_naive(input: &[usize]) -> usize {
    let offset = list_to_int(&input[..7]);
    let mut x : Vec<usize> = input.iter().cycle().take(input.len() * 10_000).cloned().collect::<Vec<_>>();
    (0..100).for_each(|n| {
        println!("{}",n);
        x = fft(&x)
    });
    list_to_int(&x[offset..8 + offset])
}

pub fn fft(input: &[usize]) -> Vec<usize> {
    let mut psums = input.iter().scan(0,|a,&b| {*a += b;  Some(*a)}).collect::<Vec<_>>();
    psums.insert(0,0);
    let len = input.len();
    (0..len).into_par_iter()
        .map(|ix| {
            let pos = (ix..len)
                .step_by(4 * (ix + 1))
                .map(|start| {
                    let end = min(len,start + 1 + ix);
                    psums[end] - psums[start]
                }).sum::<usize>();
            let neg = (3 * ix + 2..len)
                .step_by(4 * (ix + 1))
                .map(|start| {
                    let end = min(len,start + 1 + ix);
                    psums[end] - psums[start]
                }).sum::<usize>();
            (max(pos,neg) - min(pos,neg)) % 10
        })
        .collect()
}
#[test]
pub fn day16p1test() {
    assert_eq!(fft(&[1, 2, 3, 4, 5, 6, 7, 8]), [4, 8, 2, 2, 6, 1, 5, 8]);
    assert_eq!(fft(&[4, 8, 2, 2, 6, 1, 5, 8]), [3, 4, 0, 4, 0, 4, 3, 8]);
    assert_eq!(p1(&gen(&"80871224585914546619083218645595")), 24176176);
    assert_eq!(p1(&gen(&"19617804207202209144916044189917")), 73745418);
    assert_eq!(p1(&gen(&"69317163492948606335995924319873")), 52432133);
}

#[test]
pub fn day16p2test() {
    assert_eq!(p2(&gen(&"03036732577212944063491565474664")), 84462026);
    assert_eq!(p2(&gen(&"02935109699940807407585447034323")), 78725270);
    assert_eq!(p2(&gen(&"03081770884921959731165446850517")), 53553731);
}
