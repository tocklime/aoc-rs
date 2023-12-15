aoc_harness::aoc_main!(2023 day 15, part1 [p1], part2 [p2], example both EG => (1320,145));

fn hash(input: &str) -> usize {
    let mut current_value = 0;
    for b in input.as_bytes() {
        current_value += usize::from(*b);
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}
fn p1(input: &str) -> usize {
    input.trim().split(',').map(hash).map(usize::from).sum()
}

fn p2(input: &str) -> usize {
    const EMPTY : Vec<(&str, usize)> = Vec::new();
    let mut boxes: [Vec<(&str, usize)>;256] = [EMPTY; 256];
    for i in input.trim().split(',') {
        if let Some((label, val)) = i.split_once('=') {
            let val = val.trim().parse::<usize>().unwrap();
            let bo = &mut boxes[hash(label)];
            if let Some(x) = bo.iter().position(|i| i.0 == label) {
                bo[x].1 = val;
            } else {
                bo.push((label, val));
            }
        } else {
            debug_assert_eq!(i.as_bytes().last().unwrap(), &b'-');
            let label = &i[0..i.len() - 1]; //ignore the final '-'.
            let bo = &mut boxes[hash(label)];
            if let Some(x) = bo.iter().position(|i| i.0 == label) {
                bo.remove(x);
            }
        }
    }
    boxes
        .into_iter()
        .zip(1..)
        .flat_map(|(b, n)| {
            b.into_iter()
                .zip(1..)
                .map(move |(lens, lens_n)| lens.1 * lens_n * n)
        })
        .sum()
}
const EG: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
