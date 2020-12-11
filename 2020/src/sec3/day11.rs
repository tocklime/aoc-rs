use std::collections::HashMap;

use crate::utils::cartesian::{as_point_map, render_char_map_w, Point};

type M = HashMap<Point<isize>, char>;

pub fn step(input: &M) -> M {
    //0 -> True
    //>=4 -> False
    //_ -> Unchanged
    let mut ans: M = HashMap::new();
    for (&p, &c) in input {
        if c == 'L'
            && p.neighbours_with_diagonals()
                .iter()
                .filter(|&x| input.get(x) == Some(&'#'))
                .count()
                == 0
        {
            ans.insert(p, '#');
        } else if c == '#'
            && p.neighbours_with_diagonals()
                .iter()
                .filter(|&x| input.get(x) == Some(&'#'))
                .count()
                >= 4
        {
            ans.insert(p, 'L');
        } else {
            ans.insert(p, c);
        }
    }
    ans
}

#[aoc(day11, part1)]
pub fn p1(input: &str) -> usize {
    let mut hm: M = as_point_map(input, false);
    loop {
        let next = step(&hm);
        if next == hm {
            hm = next;
            break;
        }
        hm = next;
    }
    hm.values().filter(|&&x| x == '#').count()
}

pub fn visible_neighbours(input: &M, p: Point<isize>) -> Vec<char> {
    //find first visible seat in each direction.
    let dirs = Point::new(0, 0).neighbours_with_diagonals();
    let mut ans = Vec::new();
    for &d in &dirs {
        let c = (1..)
            .map(|dist| (dist, input.get(&(p + (d * dist)))))
            .take_while(|(_, x)| x.is_some())
            .find(|&(_, x)| x == Some(&'L') || x == Some(&'#'));
        if let Some((dist, Some(ch))) = c {
            ans.push(*ch);
        }
    }
    ans
}

pub fn step2(input: &M) -> M {
    //0 -> True
    //>=5 -> False
    //_ -> Unchanged
    let mut ans: M = HashMap::new();
    for (&p, &c) in input {
        ans.insert(p, c);
        if "L#".contains(c) {
            let ns = visible_neighbours(input, p).iter().filter(|x| **x == '#').count();
            if c == 'L' && ns == 0 {
                ans.insert(p, '#');
                if p == Point::new(0,0){
                    //println!("becomes occupied");
                }
            } else if c == '#' && ns >= 5 {
                ans.insert(p, 'L');
                if p == Point::new(0,0){
                    //println!("becomes vacant");
                }
            } 
       
     } }
        ans
}

#[aoc(day11, part2)]
pub fn p2(input: &str) -> usize {
    let mut hm: M = as_point_map(input, false);
    loop {
        let next = step2(&hm);
        if next == hm {
            hm = next;
            break;
        }
        hm = next;
    }
    hm.values().filter(|&&x| x == '#').count()
}
