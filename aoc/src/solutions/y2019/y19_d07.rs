use itertools::Itertools;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use utils::intcode::Computer;

use aoc_harness::*;
aoc_main!(2019 day 7, part1 [p1] => 273814, part2 [p2] => 34579864,
    example part1 EG => 43210
);

const EG: &str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
pub fn p1(input: &str) -> isize {
    (0..5)
        .permutations(5)
        .map(|x| run_comp_loop(input, &x))
        .max()
        .unwrap()
}

pub fn p2(input: &str) -> isize {
    (5..10)
        .permutations(5)
        .map(|x| run_comp_loop(input, &x))
        .max()
        .unwrap()
}
fn run_comp_loop(input: &str, a: &[isize]) -> isize {
    let c_count = a.len();
    let comps: Vec<_> =
        std::iter::repeat_with(|| Arc::new(Mutex::new(Computer::from_str(input).unwrap())))
            .take(c_count)
            .collect();
    for (ix, v) in a.iter().enumerate() {
        let mut this_comp = comps[ix].lock().unwrap();
        let mut prev_comp = comps[(ix + c_count - 1) % c_count].lock().unwrap();
        let mut input = vec![*v];
        if ix == 0 {
            input.push(0)
        }
        this_comp
            .with_name(format!("C-{}-{}", ix, v))
            .connect_output_from(&mut prev_comp, &input);
    }
    let ts: Vec<_> = comps
        .iter()
        .map(|c| {
            let cc = c.clone();
            thread::spawn(move || {
                let mut m = cc.lock().unwrap();
                m.run();
            })
        })
        .collect();
    for t in ts {
        t.join().unwrap();
    }
    let last_comp = comps.last().unwrap().lock().unwrap();
    last_comp.get_last_output()
}

#[test]
pub fn p2_tests() {
    let e0 =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    assert_eq!(run_comp_loop(e0, &vec![9, 8, 7, 6, 5]), 139629729);
    assert_eq!(p2(e0), 139629729);
    let e1= "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    assert_eq!(run_comp_loop(e1, &vec![9, 7, 8, 5, 6]), 18216);
    assert_eq!(p2(e1), 18216);
}
