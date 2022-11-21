use aoc_harness::aoc_main;
use std::collections::{HashMap, VecDeque};
use utils::intcode::Computer;

aoc_main!(2019 day 23, part1 [p1] => 20225, part2 [p2] => 14348);

pub fn run_comp(
    comp: &mut Computer<i64>,
    input: &[i64],
    send_q: &mut VecDeque<(i64, (i64, i64))>,
) -> bool {
    input.iter().copied().for_each(|x| {
        comp.with_input(x);
        comp.run_to_input();
    });
    let o = comp.take_output();
    for c in o.chunks(3) {
        send_q.push_back((c[0], (c[1], c[2])));
    }
    !o.is_empty()
}

//#[aoc(day23, part1)]
pub fn p1(input: &str) -> i64 {
    run(input, |_| true)
}

//#[aoc(day23, part2)]
pub fn p2(input: &str) -> i64 {
    let mut last = None;
    run(input, |x| {
        let t = last;
        last = Some(x);
        t == last
    })
}

const COMP_COUNT: i64 = 50;

pub fn run<T>(input: &str, mut send_nat_y: T) -> i64
where
    T: FnMut(i64) -> bool,
{
    let master: Computer<i64> = input.parse().expect("Can't parse input");
    let mut send_q: VecDeque<(i64, (i64, i64))> = VecDeque::new();
    let mut comps: HashMap<_, _> = (0..COMP_COUNT)
        .map(|a| {
            let mut c = master.clone();
            run_comp(&mut c, &[a], &mut send_q);
            (a, c)
        })
        .collect();

    let mut nat: Option<(i64, i64)> = None;
    //loop until callback function tells us we're done.
    loop {
        //sending - send messages whilst there are any to send.
        while !send_q.is_empty() {
            let (ix, (x, y)) = send_q.pop_front().unwrap();
            if ix == 255 {
                nat = Some((x, y));
            } else {
                let c = comps.get_mut(&ix).unwrap();
                run_comp(c, &[x, y], &mut send_q);
            }
        }
        //idle loop - try sending -1 to computers until someone sends something.
        for c in comps.values_mut() {
            if run_comp(c, &[-1], &mut send_q) {
                break;
            }
        }
        //...if the idle loop is done, and still nothing in queue, send NAT value.
        if send_q.is_empty() {
            if let Some(n) = nat {
                if send_nat_y(n.1) {
                    break n.1;
                }
                send_q.push_back((0, n));
                nat = None;
            } else {
                panic!("Network quiet, nothing to send");
            }
        }
    }
}
