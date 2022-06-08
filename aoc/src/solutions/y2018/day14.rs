use utils::nums::digits;
use itertools::Itertools;


fn gen(input: &str) -> usize {
    input.trim().parse().unwrap()
}

fn p1(input: &usize) -> String {
    let mut d = vec![3, 7];
    let mut pos = vec![0, 1];

    while d.len() < *input + 10 {
        let sum = d[pos[0]] + d[pos[1]];
        let mut digs = digits(sum).collect_vec();
        d.append(&mut digs);
        for p in pos.iter_mut() {
            *p = (*p + 1 + d[*p]) % d.len();
        }
    }
    d.iter()
        .skip(*input)
        .take(10)
        .map(|x| x.to_string())
        .join("")
}


fn p2(input: &usize) -> usize {
    let mut d = vec![3, 7];
    let mut pos = vec![0, 1];
    let target = digits(*input).collect_vec();
    loop {
        let sum = d[pos[0]] + d[pos[1]];
        for x in digits(sum) {
            d.push(x);
            if d.ends_with(&target) {
                return d.len() - target.len();
            }
        }

        for p in pos.iter_mut() {
            *p = (*p + 1 + d[*p]) % d.len();
        }
    }
}
