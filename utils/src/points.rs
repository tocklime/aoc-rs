use num::integer::gcd;
use num_enum::TryFromPrimitive;
use std::cmp::{max, min};
use std::convert::TryInto;
use std::f64::consts::PI;
use std::hash::BuildHasher;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
#[derive(PartialEq, Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Dir {
    U,
    L,
    D,
    R,
}

impl Dir {
    pub fn all() -> [Self; 4] {
        [Self::U, Self::D, Self::L, Self::R]
    }
    pub fn from_udlr(c: &str) -> Option<Self> {
        match c {
            "U" => Some(Self::U),
            "L" => Some(Self::L),
            "D" => Some(Self::D),
            "R" => Some(Self::R),
            _ => None,
        }
    }
    pub fn from_nsew(c: &str) -> Option<Self> {
        match c {
            "N" => Some(Self::U),
            "W" => Some(Self::L),
            "S" => Some(Self::D),
            "E" => Some(Self::R),
            _ => None,
        }
    }
    pub fn as_point_delta(self) -> Point {
        match self {
            Self::U => Point(0, 1),
            Self::D => Point(0, -1),
            Self::L => Point(-1, 0),
            Self::R => Point(1, 0),
        }
    }
    pub fn rotate_left(self) -> Self {
        match self {
            Self::U => Self::L,
            Self::L => Self::D,
            Self::D => Self::R,
            Self::R => Self::U,
        }
    }
    pub fn about_turn(self) -> Self {
        match self {
            Self::U => Self::D,
            Self::L => Self::R,
            Self::D => Self::U,
            Self::R => Self::L,
        }
    }
    pub fn rotate_right(self) -> Self {
        match self {
            Self::U => Self::R,
            Self::L => Self::U,
            Self::D => Self::L,
            Self::R => Self::D,
        }
    }
    pub fn is_horizontal(self) -> bool {
        self == Self::R || self == Self::L
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Point(pub isize, pub isize);
impl Mul<isize> for Point {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
impl Mul<usize> for Point {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        let i: isize = rhs.try_into().unwrap();
        self * i
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Div<isize> for Point {
    type Output = Self;
    fn div(self, rhs: isize) -> Self {
        Self(self.0 / rhs, self.1 / rhs)
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}
impl Point {
    pub fn origin() -> Self {
        Self(0, 0)
    }
    pub fn manhattan_from_origin(self) -> usize {
        (self.0.abs() + self.1.abs()).try_into().unwrap()
    }
    pub fn manhattan(self, other: Self) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs())
            .try_into()
            .unwrap()
    }
    pub fn gcd(self) -> isize {
        gcd(self.0, self.1)
    }
    pub fn size_squared(self) -> isize {
        self.0 * self.0 + self.1 * self.1
    }
    pub fn simplest_direction(self) -> Self {
        self / self.gcd()
    }
    pub fn quadrant_clockwise(self) -> usize {
        match (self.0 >= 0, self.1 >= 0) {
            (true, false) => 1,
            (true, true) => 2,
            (false, true) => 3,
            (false, false) => 4,
        }
    }
    pub fn neighbours(self) -> [Self; 4] {
        [
            self.step(Dir::U),
            self.step(Dir::L),
            self.step(Dir::D),
            self.step(Dir::R),
        ]
    }
    pub fn step(self, d: Dir) -> Self {
        self + d.as_point_delta()
    }
    pub fn up(self) -> Self {
        self.step(Dir::U)
    }
    pub fn down(self) -> Self {
        self.step(Dir::D)
    }
    pub fn left(self) -> Self {
        self.step(Dir::L)
    }
    pub fn right(self) -> Self {
        self.step(Dir::R)
    }
}
#[derive(Debug)]
pub struct PolarCoord {
    pub r: f64,
    pub theta: f64,
}
impl PolarCoord {
    #[allow(clippy::cast_precision_loss)]
    pub fn from_point(p: Point) -> Self {
        Self {
            r: (p.size_squared() as f64).sqrt(),
            theta: (p.0 as f64).atan2(p.1 as f64),
        }
    }
    pub fn simplify(self) -> Self {
        Self {
            r: self.r,
            theta: if self.theta > 2. * PI {
                self.theta % (2. * PI)
            } else {
                self.theta
            },
        }
    }
    pub fn rotate(self, rad: f64) -> Self {
        Self {
            r: self.r,
            theta: self.theta + rad,
        }
        .simplify()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub bottom_left: Point,
    pub top_right: Point,
}

impl Aabb {
    pub fn new(p: Point) -> Self {
        Self {
            bottom_left: p,
            top_right: p,
        }
    }

    pub fn extend(&self, p: Point) -> Self {
        let mut ans = *self;
        ans.bottom_left.0 = min(ans.bottom_left.0, p.0);
        ans.bottom_left.1 = min(ans.bottom_left.1, p.1);
        ans.top_right.0 = max(ans.top_right.0, p.0);
        ans.top_right.1 = max(ans.top_right.1, p.1);
        ans
    }
    pub fn contains(&self, p: Point) -> bool {
        self.bottom_left.0 <= p.0
            && self.bottom_left.1 <= p.1
            && self.top_right.0 >= p.0
            && self.top_right.1 >= p.1
    }
    pub fn extend_box(&self, b: Self) -> Self {
        self.extend(b.bottom_left).extend(b.top_right)
    }
    pub fn intersect(&self, b: Self) -> Self {
        Self {
            bottom_left: Point(
                max(self.bottom_left.0, b.bottom_left.0),
                max(self.bottom_left.1, b.bottom_left.1),
            ),
            top_right: Point(
                min(self.top_right.0, b.top_right.0),
                min(self.top_right.1, b.top_right.1),
            ),
        }
    }
    pub fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        (self.bottom_left.1..=self.top_right.1)
            .flat_map(move |y| (self.bottom_left.0..=self.top_right.0).map(move |x| Point(x, y)))
    }
    pub fn vec_with<T: Default + Clone>(&self, ft: impl Fn(Point) -> T) -> Vec<Vec<T>> {
        let offset = self.bottom_left;
        let mut v = vec![vec![Default::default(); self.width()]; self.height()];
        for p in self.all_points() {
            let rel = p - offset;
            let x: usize = rel.0.try_into().unwrap();
            let y: usize = rel.1.try_into().unwrap();
            v[y][x] = ft(p);
        }
        v
    }
    pub fn width(&self) -> usize {
        (1 + self.top_right.0 - self.bottom_left.0)
            .try_into()
            .unwrap()
    }
    pub fn height(&self) -> usize {
        (1 + self.top_right.1 - self.bottom_left.1)
            .try_into()
            .unwrap()
    }
}

#[test]
pub fn bb_tests() {
    let a = Aabb::new(Point(0, 0)).extend(Point(0, 10));
    let b = Aabb::new(Point(-3, 4)).extend(Point(8, 4));
    let i = a.intersect(b);
    println!("{:?}", i);
    assert_eq!(i.bottom_left, Point(0, 4));
    assert_eq!(i.top_right, Point(0, 4));
}

use std::collections::HashMap;
pub fn point_map_bounding_box<T, S: BuildHasher>(hm: &HashMap<Point, T, S>) -> Aabb {
    let a_point = hm.keys().next().unwrap();
    hm.keys().fold(Aabb::new(*a_point), |bb, &k| bb.extend(k))
}
pub fn render_char_map<S: BuildHasher>(m: &HashMap<Point, char, S>) -> String {
    render_char_map_w(m, 1, ' ')
}
pub fn render_char_map_w<S: BuildHasher>(
    m: &HashMap<Point, char, S>,
    width: u8,
    default: char,
) -> String {
    let bb = crate::points::point_map_bounding_box(m);
    let v = bb.vec_with(|p| *m.get(&p).unwrap_or(&default));
    v.iter()
        .map(|l| {
            "\n".to_string()
                + &l.iter()
                    .flat_map(|&x| (0..width).map(move |_| x))
                    .collect::<String>()
        })
        .rev() //looks upside down...
        .collect()
}
pub fn as_point_map(input: &str) -> HashMap<Point, char> {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point(x.try_into().unwrap(), y.try_into().unwrap()), c))
        })
        .collect()
}
