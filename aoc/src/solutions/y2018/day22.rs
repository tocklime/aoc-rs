aoc_harness::aoc_main!(2018 day 22, part1 [p1], part2 [p2]);
use utils::cartesian::Point;
use nom::lib::std::collections::HashMap;
use utils::aabb::Aabb;
use pathfinding::directed::astar::astar;

fn erosion_level(geologic_index: i32, depth: i32) -> i32 {
    (geologic_index + depth) % 20183
}

fn make_map(depth: i32, target: Point<i32>) -> HashMap::<Point<i32>, i32> {
    let start = Point::new(0, 0);
    let mut erosion_levels = HashMap::new();
    let mut types = HashMap::new();
    let area = Aabb::new(start).extend(target);
    for p in area.all_points() {
        let geologic_index = if p == start || p == target { 0 } else if p.y == 0 {
            p.x * 16807
        } else if p.x == 0 {
            p.y * 48271
        } else {
            erosion_levels[&p.left()] * erosion_levels[&p.down()]
        };
        let erosion = erosion_level(geologic_index, depth);
        erosion_levels.insert(p, erosion);
        types.insert(p, erosion % 3);
    }
    types
}


fn p1(_input: &str) -> i32 {
    let map = make_map(3066, Point::new(13, 726));
    map.values().sum()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Equipment {
    None,
    ClimbingGear,
    Torch,
}

impl Equipment {
    fn is_compatible(self, typ: i32) -> bool {
        match (self, typ) {
            (Self::None, 0) => false,
            (Self::Torch, 1) => false,
            (Self::ClimbingGear, 2) => false,
            _ => true
        }
    }
}


fn p2(_input: &str) -> i32 {
    let target = Point::new(13, 726);
    //draw a map which is hopefully big enough...
    let map = make_map(3066, Point::new(200_i32, 1000_i32));
    let initial = (Point::new(0_i32, 0_i32), Equipment::Torch);
    let r = astar(&initial,
                  |&(p, equ)| {
                      //can move with same equipment for 1,
                      //or swap equipment (if compatible) for 7.
                      let mut options: Vec<_> = p.neighbours().iter()
                          .filter_map(|&n| {
                              if n.x >= 0 && n.y >= 0 && equ.is_compatible(*map.get(&n).unwrap_or_else(|| panic!("unknown square: {:?}", n)))
                              { Some(((n, equ), 1)) } else { None }
                          }).collect();
                      for &other in &[Equipment::None, Equipment::Torch, Equipment::ClimbingGear] {
                          if other != equ && other.is_compatible(map[&p]) {
                              options.push(((p, other), 7));
                          }
                      }
                      options
                  },
                  |&(p, _)| (target - p).manhattan(),
                  |&(p, equ)| p == target && equ == Equipment::Torch);
    r.unwrap().1
}
