use reformation::Reformation;
use std::collections::VecDeque;
use itertools::Itertools;

#[derive(Debug,Reformation)]

enum Move {
    #[reformation("s{}")]
    Spin(usize),
    #[reformation("x{}/{}")]
    Exchange(usize,usize),
    #[reformation("p{}/{}")]
    Partner(char,char)
}

fn step(pos: &mut VecDeque<char>, m: &Move) {
    match m {
        Move::Spin(x) => {
            pos.rotate_right(*x);
        },
        Move::Exchange(x,y) => {
            pos.swap(*x,*y);
        },
        Move::Partner(a,b) => {
            let x = pos.iter().enumerate().find(|&(_,c)| c == a).unwrap().0;
            let y = pos.iter().enumerate().find(|&(_,c)| c == b).unwrap().0;
            pos.swap(x,y);
        }
    }
}
fn dance(pos: &mut VecDeque<char>, ms: &[Move]) {
    for i in ms {
        step(pos,i);
    }
}

#[aoc_generator(day16)]
fn gen(input: &str)-> Vec<Move> {
    input.split(',').map(|x| Move::parse(x).unwrap()).collect_vec()
}

#[aoc(day16,part1)]
fn p1(input: &[Move]) -> String {
    let mut pos : VecDeque<char> = "abcdefghijklmnop".chars().collect();
    dance(&mut pos,input);
    pos.iter().collect()
}

#[aoc(day16,part2)]
fn p2(input: &[Move]) -> String {
    let mut pos : VecDeque<char> = "abcdefghijklmnop".chars().collect();
    let orig = pos.clone();
    let loop_size = (1..).find(|_| {
        dance(&mut pos, input);
        pos == orig
    }).unwrap();
    let remain = 1_000_000_000 % loop_size;
    for _ in 0..remain {
        dance(&mut pos, input);
    }
    pos.iter().collect()
}
