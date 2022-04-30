use aoc_harness::aoc_main;
use utils::intcode::Computer;
use aoc2019::utils::points::*;
use utils::ocr::OcrString;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

aoc_main!(2019 day 11, part1 [p1] => 2160, part2 [p2] => "LRZECGFE");
const WHITE: char = '█';
const BLACK: char = ' ';

pub fn robot(
    input: &mpsc::Receiver<isize>,
    output: &mpsc::Sender<isize>,
    c: char,
) -> HashMap<Point, char> {
    let mut painted_panels = HashMap::new();
    let mut p = Point(0, 0);
    let mut d = Dir::U;
    painted_panels.insert(p, c);
    loop {
        if output
            .send((painted_panels.get(&p) == Some(&WHITE)).into())
            .is_err()
        {
            break;
        }
        match input.recv() {
            Ok(0) => painted_panels.insert(p, BLACK),
            Ok(1) => painted_panels.insert(p, WHITE),
            Err(_) => break,
            _ => panic!("Unknown paint instruction"),
        };
        match input.recv() {
            Ok(0) => {
                d = d.rotate_left();
            }
            Ok(1) => d = d.rotate_left().rotate_left().rotate_left(),
            Err(_) => break,
            _ => panic!("Unknown turn instruction"),
        }
        p += d.as_point_delta();
    }
    painted_panels
}
pub fn my_run(input: &str, init_c: char) -> HashMap<Point, char> {
    let mut c: Computer<isize> = Computer::from_str(input).unwrap();
    let (tx,rx) = c.make_io_chans();
    let c_thr = thread::spawn(move || {
        c.run();
    });
    let robot_thr = thread::spawn(move || robot(&rx, &tx, init_c));
    c_thr.join().unwrap();
    robot_thr.join().unwrap()
}

pub fn p1(input: &str) -> usize {
    my_run(input, BLACK).len()
}

pub fn p2(input: &str) -> OcrString {
    aoc2019::utils::points::render_char_map(&my_run(input, WHITE)).into()
}
#[test]
pub fn example() {
    let (txa, rxa) = mpsc::channel::<isize>();
    let (txb, rxb) = mpsc::channel::<isize>();
    let r = thread::spawn(move || robot(&rxb, &txa, BLACK));

    let input: Vec<isize> = vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0];
    let correct_output = [0, 0, 0, 0, 1, 0, 0, 0];
    for i in input.iter() {
        txb.send(*i).unwrap()
    }
    drop(txb);
    let mut output = vec![];
    while let Ok(i) = rxa.recv() {
        output.push(i);
    }
    assert_eq!(output, correct_output);
    assert_eq!(r.join().unwrap().len(), 6);
}
