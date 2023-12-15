aoc_harness::aoc_main!(2023 day 15, part1 [p1], part2 [p2], example both EG => (1320,145));

fn hash(input: &str) -> u8 {
    let mut current_value = 0u8;
    for b in input.as_bytes() {
        if b'\n' != *b {
            current_value = current_value.wrapping_add(*b);
            current_value = current_value.wrapping_mul(17);
        }
    }
    current_value
}
fn p1(input: &str) -> usize {
    input.split(',').map(hash).map(usize::from).sum()
}

fn p2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];
    for i in input.split(',') {
        if let Some((label, val)) = i.split_once('=') {
            let val = val.trim().parse::<u8>().expect(&format!("{i} -> {label} '{val}'"));
            let hash = hash(label);
            let bo: &mut Vec<(&str, u8)> = &mut boxes[usize::from(hash)];
            if let Some(x) = bo.iter().position(|i| i.0 == label) {
                bo[x] = (label, val);
            } else {
                bo.push((label, val));
            }
        } else {
            assert_eq!(i.as_bytes().last().unwrap(), &b'-');
            let label = &i[0..i.len() - 1];
            let hash = hash(label);
            let bo: &mut Vec<(&str, u8)> = &mut boxes[usize::from(hash)];
            if let Some(x) = bo.iter().position(|i| i.0 == label) {
                bo.remove(x);
            }
        }
    }
    boxes
        .iter()
        .zip(1..)
        .map(|(b, n)| {
            b.iter()
                .zip(1..)
                .map(|(lens, lens_n)| usize::from(lens.1) * n * lens_n)
                .sum::<usize>()
        })
        .sum()
}
const EG: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
