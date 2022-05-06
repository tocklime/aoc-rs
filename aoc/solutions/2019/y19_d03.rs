use aoc_harness::*;
use utils::aabb::Aabb;
use utils::cartesian::{Dir, Point};

aoc_main!(2019 day 3, generator gen, part1 [p1] => 1017, part2 [p2] => 11432,
    example part1 EG1 => 159,
    example part1 EG2 => 135,
    example part2 EG1 => 610,
    example part2 EG2 => 410,
);
const EG1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
const EG2: &str =
    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct WireLine {
    pub dir: Dir,
    pub len: usize,
    pub start: Point<isize>,
    pub signal_delay: usize,
}

impl WireLine {
    pub fn end(self) -> Point<isize> {
        self.start + self.dir * (self.len as isize)
    }
    pub fn as_bb(self) -> Aabb<isize> {
        Aabb::new(self.start).extend(self.end())
    }
    pub fn intersects(self, other: &Self) -> Option<Point<isize>> {
        let bb1 = self.as_bb();
        let bb2 = other.as_bb();
        let joined = bb1.extend_box(bb2);
        if joined.height() > (bb1.height() + bb2.height()) {
            return None;
        }
        if joined.width() > (bb1.width() + bb2.width()) {
            return None;
        }
        let a = bb1.intersect(bb2);
        Some(a.bottom_left)
    }
    pub fn signal_delay_at(self, p: Point<isize>) -> usize {
        self.signal_delay + (p - self.start).manhattan() as usize
    }
}

type Wire = Vec<WireLine>;

pub fn gen(input: &str) -> Vec<Wire> {
    input
        .lines()
        .map(|l| {
            let mut p = Point::new(0, 0);
            let mut delay = 0;
            l.split(',')
                .map(|i| {
                    let (d, n) = i.split_at(1);
                    let wl = WireLine {
                        dir: Dir::from_x("UDLR", d.chars().exactly_one().unwrap()),
                        len: n.parse().unwrap(),
                        start: p,
                        signal_delay: delay,
                    };
                    delay += wl.len;
                    p += wl.dir * (wl.len as isize);
                    wl
                })
                .collect()
        })
        .collect()
}

pub fn p1(input: &[Wire]) -> isize {
    input[0]
        .iter()
        .flat_map(move |a| input[1].iter().filter_map(move |b| a.intersects(b)))
        .map(Point::manhattan)
        .filter(|l| *l > 0)
        .min()
        .unwrap()
}

pub fn p2(input: &[Wire]) -> usize {
    input[0]
        .iter()
        .flat_map(move |a| {
            input[1].iter().filter_map(move |b| {
                a.intersects(b)
                    .map(|i| a.signal_delay_at(i) + b.signal_delay_at(i))
            })
        })
        .filter(|l| *l > 0)
        .min()
        .unwrap()
}
