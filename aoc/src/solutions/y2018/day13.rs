aoc_harness::aoc_main!(2018 day 13, generator gen, part1 [p1], part2 [p2]);
use std::collections::{BTreeMap, HashMap, HashSet};
use utils::cartesian::{as_point_map, Dir, Point};

#[derive(Clone, Debug)]
struct State {
    map: HashMap<Point<u32>, char>,
    carts: BTreeMap<Point<u32>, (Dir, usize)>,
}

impl State {
    fn step_carts(&mut self) -> Vec<Point<u32>> {
        let mut new_carts = BTreeMap::new();
        let mut crashes: Vec<Point<u32>> = Vec::new();
        let mut orig_posses = self.carts.keys().copied().collect::<HashSet<Point<u32>>>();
        for (p, &(d, turn_count)) in &self.carts {
            if !orig_posses.contains(p) {
                //already crashed?
                continue;
            }
            let new_pos = p.step(d);
            orig_posses.remove(p);
            let new_dir = match (d, self.map.get(&new_pos)) {
                (Dir::Down, Some('/')) | (Dir::Up, Some('\\')) => (Dir::Right, turn_count),
                (Dir::Up, Some('/')) | (Dir::Down, Some('\\')) => (Dir::Left, turn_count),
                (Dir::Left, Some('/')) | (Dir::Right, Some('\\')) => (Dir::Up, turn_count),
                (Dir::Right, Some('/')) | (Dir::Left, Some('\\')) => (Dir::Down, turn_count),
                (d, Some('+')) => match turn_count % 3 {
                    0 => (d.turn_right(), turn_count + 1),
                    1 => (d, turn_count + 1),
                    2 => (d.turn_left(), turn_count + 1),
                    _ => unreachable!(),
                },
                _ => (d, turn_count),
            };
            if orig_posses.remove(&new_pos) {
                //crash into a not-yet-moved cart.
                orig_posses.remove(&new_pos);
                crashes.push(new_pos);
            } else if new_carts.remove(&new_pos).is_some() {
                //crash into a moved cart.
                crashes.push(new_pos);
            } else {
                //dodged everything.
                new_carts.insert(new_pos, new_dir);
            }
        }
        self.carts = new_carts;
        crashes
    }
}

fn gen(input: &str) -> State {
    let map = as_point_map(input, false);
    let carts = map
        .iter()
        .filter_map(|(k, &v)| Dir::try_from_x("v^<>", v).map(|d| (*k, (d, 0))))
        .collect();
    State { map, carts }
}

fn p1(input: &State) -> String {
    let mut s = input.clone();
    loop {
        let crashes = s.step_carts();
        if !crashes.is_empty() {
            return format!("{},{}", crashes[0].x, crashes[0].y);
        }
    }
}

fn p2(input: &State) -> String {
    let mut s = input.clone();
    loop {
        s.step_carts();
        if s.carts.len() == 1 {
            let a = *s.carts.iter().next().unwrap().0;
            return format!("{},{}", a.x, a.y);
        }
    }
}
