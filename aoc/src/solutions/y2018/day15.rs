use utils::cartesian::{as_point_map, Point};
use itertools::Itertools;
use pathfinding::prelude::astar_bag;
use std::{
    cell::Cell,
    cmp::max,
    collections::{BTreeMap, BTreeSet, HashSet},
};

#[derive(Clone, PartialEq, Eq, Debug)]
enum Side {
    Goblin,
    Elf,
}

enum Err {
    NoTargetsCombatEnded,
}
#[derive(Clone, Debug)]
struct Unit {
    id: usize,
    side: Side,
    attack_power: i32,
    hp: Cell<i32>,
}

#[derive(Clone)]
struct State {
    walls: HashSet<Point<i32>>,
    units: BTreeMap<Point<i32>, Unit>,
    static_units: HashSet<usize>,
}
impl State {
    fn boost_elves(&mut self, p: i32) {
        for u in self.units.values_mut() {
            if u.side == Side::Elf {
                u.attack_power += p;
            }
        }
    }
    fn count_elves(&self) -> usize {
        self.units.values().filter(|x| x.side == Side::Elf).count()
    }
    fn space_is_free(&self, p: &Point<i32>) -> bool {
        !self.units.contains_key(p) && !self.walls.contains(p)
    }
    fn total_health(&self) -> i32 {
        self.units.values().map(|x| max(x.hp.get(), 0)).sum::<i32>()
    }
    #[allow(dead_code)]
    fn render(&self) -> String {
        use std::iter::FromIterator;

        use utils::aabb::Aabb;

        let mut ans = String::new();
        let bb: Aabb<i32> = Aabb::from_iter(&mut self.walls.iter().cloned());
        for y in bb.bottom_left.y..=bb.top_right.y {
            //draw row.
            let mut notes = vec![];
            for x in bb.bottom_left.x..=bb.top_right.x {
                let p = Point::new(x, y);
                let c = if self.walls.contains(&p) {
                    '#'
                } else if let Some(u) = self.units.get(&p) {
                    let c = if u.side == Side::Goblin { 'G' } else { 'E' };
                    notes.push(format!("{}({})", c, u.hp.get()));
                    c
                } else {
                    '.'
                };
                ans.push(c);
            }
            if !notes.is_empty() {
                ans += "    ";
                ans += &notes.join(", ");
            }
            ans.push('\n');
        }
        ans
    }
    fn round(&mut self) -> Result<(), Err> {
        let list = self.units.iter().map(|(k, v)| (*k, v.id)).collect_vec();
        for (p, id) in list {
            match self.units.get(&p) {
                None => {}
                Some(u) => {
                    if u.id == id {
                        //otherwise we died and this is someone else.
                        let mut new_pos = p;
                        if !self.static_units.contains(&id) {
                            new_pos = self.take_step(p, u)?;
                            if new_pos == p {
                                self.static_units.insert(id);
                            } else {
                                self.static_units.clear();
                            }
                            let un = self.units.remove(&p).expect("unit gone?");
                            self.units.insert(new_pos, un);
                        }
                        if let Some(d) = self.attack(new_pos) {
                            self.units.remove(&d);
                            self.static_units.clear();
                        }
                    }
                }
            }
        }
        Ok(())
    }
    fn take_step(&self, p: Point<i32>, u: &Unit) -> Result<Point<i32>, Err> {
        let targets = self
            .units
            .iter()
            .filter(|(_, u2)| u.side != u2.side)
            .collect_vec();
        if targets.is_empty() {
            return Err(Err::NoTargetsCombatEnded);
        }
        let in_range: BTreeSet<Point<i32>> = targets
            .iter()
            //neighbour squares to each enemy unit
            .flat_map(|(p, _)| {
                let n = p.neighbours();
                n.iter().cloned().collect_vec()
            })
            //of those, only those which are not off map and not walls.
            .filter(|p2| &p == p2 || self.space_is_free(p2))
            .collect();

        //println!("{}\n", self.render());
        //dbg!(&p, u, &in_range);
        if !in_range.is_empty() && !in_range.contains(&p) {
            let paths = astar_bag(
                &p,
                |p| {
                    p.neighbours()
                        .iter()
                        .cloned()
                        .filter(|x| self.space_is_free(x))
                        .map(|x| (x, 1))
                        .collect_vec()
                },
                |x| {
                    in_range
                        .iter()
                        .map(|y| (*x - *y).manhattan())
                        .min()
                        .expect("nothing in_range?")
                },
                |x| in_range.contains(x),
            );
            if let Some((a, _)) = paths {
                let chosen = a.min_by_key(|x| (x[x.len() - 1], x[1])).unwrap();
                return Ok(chosen[1]);
            }
        }
        Ok(p)
    }
    //returns location of death.
    fn attack(&self, p: Point<i32>) -> Option<Point<i32>> {
        let me = self.units.get(&p).unwrap();
        let mut neighbours = p.neighbours().iter().cloned().collect_vec();
        neighbours.sort();

        //find all targets
        //find weakest target
        let target = neighbours
            .iter()
            .filter_map(|p| match self.units.get(p) {
                Some(x) if x.side != me.side => Some((p, x)),
                _ => None,
            })
            .min_by_key(|x| x.1.hp.get());
        if let Some((&p, t)) = target {
            //kill unit?
            t.hp.set(t.hp.get() - me.attack_power);
            if t.hp.get() <= 0 {
                return Some(p);
            }
        }
        None
    }
}


fn gen(input: &str) -> State {
    let map = as_point_map(input, false);
    let walls = map
        .iter()
        .filter_map(|(k, v)| if v == &'#' { Some(*k) } else { None })
        .collect();
    let mut id = 0;
    let units = map
        .iter()
        .filter_map(|(p, v)| {
            id += 1;
            match *v {
                'G' => Some((
                    *p,
                    Unit {
                        id,
                        side: Side::Goblin,
                        attack_power: 3,
                        hp: Cell::new(200),
                    },
                )),
                'E' => Some((
                    *p,
                    Unit {
                        id,
                        side: Side::Elf,
                        attack_power: 3,
                        hp: Cell::new(200),
                    },
                )),
                _ => None,
            }
        })
        .collect();
    State {
        walls,
        units,
        static_units: HashSet::new(),
    }
}


fn p1(input: &State) -> i32 {
    let mut st: State = input.clone();
    for t in 0.. {
        match st.round() {
            Ok(_) => {}
            Err(_) => {
                return t * st.total_health();
            }
        }
    }
    unreachable!();
}

fn run_with_no_elf_death(st: &mut State) -> Option<i32> {
    let elf_count = st.count_elves();
    for t in 0.. {
        match st.round() {
            Ok(_) => {
                if st.count_elves() != elf_count {
                    return None;
                }
            }
            Err(_) => {
                if st.count_elves() != elf_count {
                    return None;
                } else {
                    return Some(t * st.total_health());
                }
            }
        }
    }
    unreachable!();
}


fn p2(input: &State) -> i32 {
    for p in 1.. {
        let mut st = input.clone();
        st.boost_elves(p);
        if let Some(x) = run_with_no_elf_death(&mut st) {
            return x;
        }
    }
    unreachable!();
}
