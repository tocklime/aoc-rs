use crate::utils::cartesian::{as_point_map, Point};
use nom::lib::std::collections::HashMap;

fn step(hm: &HashMap<Point<u32>,char>) -> HashMap<Point<u32>,char> {
    hm.iter().map(|(&p,&c)| {
        let n = (&p.neighbours_with_diagonals()).iter().filter(|x| hm.get(x) == Some(&'#')).count();
        let next = if n == 3 || (n ==2  && c == '#') {'#'} else {'.'};
        (p,next)
    }).collect()
}

fn light_corners(hm: &mut HashMap<Point<u32>,char>) {
    let max = hm.keys().max().unwrap().clone();
    hm.insert(Point::new(0,0),'#');
    hm.insert(Point::new(0,max.y),'#');
    hm.insert(Point::new(max.x,max.y),'#');
    hm.insert(Point::new(max.x,0),'#');
}

#[aoc(day18,part1)]
fn p1(input: &str) -> usize {
    let mut hm = as_point_map::<u32>(input);
    (0..100).for_each(|_| hm = step(&hm));
    hm.iter().filter(|(_,&x)|x == '#').count()
}
#[aoc(day18,part2)]
fn p2(input: &str) -> usize {
    let mut hm = as_point_map::<u32>(input);
    light_corners(&mut hm);
    (0..100).for_each(|_| {
        hm = step(&hm);
        light_corners(&mut hm);
    });
    hm.iter().filter(|(_,&x)|x == '#').count()
}
