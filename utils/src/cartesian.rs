use crate::aabb::Aabb;
use ndarray::IntoDimension;
use num::{
    abs,
    integer::gcd,
    traits::{WrappingAdd, WrappingSub},
    Integer, Num, Signed, Unsigned,
};
use std::hash::{BuildHasher, Hash};
use std::ops::{Add, AddAssign, Mul, RangeInclusive, Sub};
use std::{collections::HashMap, fmt::Display};
use std::{collections::HashSet, fmt::Debug};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

use crate::nums::NumExt;
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
#[must_use]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
impl<T> IntoDimension for Point<T>
where
    (T, T): IntoDimension,
{
    type Dim = <(T, T) as IntoDimension>::Dim;

    fn into_dimension(self) -> Self::Dim {
        (self.y, self.x).into_dimension()
    }
}

impl<T: FromStr> FromStr for Point<T> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',').map(|x| x.trim().parse::<T>());
        let x = s.next().ok_or("No items")?.map_err(|_| "Bad parse")?;
        let y = s.next().ok_or("Only 1 item")?.map_err(|_| "Bad parse")?;
        if s.next().is_some() {
            return Err(">2 items");
        }
        Ok(Self { x, y })
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
#[must_use]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn from_x(udlr: &str, c: char) -> Self {
        let ix = udlr.find(c).expect("Unknown direction");
        [Self::Up, Self::Down, Self::Left, Self::Right][ix]
    }
    #[must_use]
    pub const fn to_udlr(self) -> char {
        match self {
            Self::Up => 'U',
            Self::Down => 'D',
            Self::Left => 'L',
            Self::Right => 'R',
        }
    }
    pub fn turn_right_n(self, n: u8) -> Self {
        n.applications_of(self, Self::turn_right)
    }
    pub const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    pub const fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
    pub const fn turn_about(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
    pub fn as_point_step<T: Signed + Num>(self) -> Point<T> {
        match self {
            Self::Up => Point::new(T::zero(), T::one()),
            Self::Down => Point::new(T::zero(), T::neg(T::one())),
            Self::Left => Point::new(T::neg(T::one()), T::zero()),
            Self::Right => Point::new(T::one(), T::zero()),
        }
    }
}

impl<T: Default> Default for Point<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl<T: Sized> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn from_dim(p: (T, T)) -> Self {
        Self { x: p.1, y: p.0 }
    }
    pub fn as_tuple_y_first(self) -> (T, T) {
        (self.y, self.x)
    }
}
impl<T: Num> Point<T> {
    pub fn origin() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}
impl<T: Num + WrappingAdd> Point<T> {
    pub fn up(self) -> Self {
        Self {
            x: self.x,
            y: self.y.wrapping_add(&T::one()),
        }
    }
    pub fn right(self) -> Self {
        Self {
            x: self.x.wrapping_add(&T::one()),
            y: self.y,
        }
    }
}
impl<T: Num + WrappingSub> Point<T> {
    pub fn down(self) -> Self {
        Self {
            x: self.x,
            y: self.y.wrapping_sub(&T::one()),
        }
    }
    pub fn left(self) -> Self {
        Self {
            x: self.x.wrapping_sub(&T::one()),
            y: self.y,
        }
    }
}
impl<T: Num + WrappingAdd + WrappingSub> Point<T> {
    pub fn step(self, d: Dir) -> Self {
        match d {
            Dir::Up => self.up(),
            Dir::Down => self.down(),
            Dir::Left => self.left(),
            Dir::Right => self.right(),
        }
    }
    pub fn follow_x(self, udlr: &str, c: char) -> Self {
        self.step(Dir::from_x(udlr, c))
    }

    pub fn follow_arrow(self, arrow: char) -> Self {
        self.follow_x("^v<>", arrow)
    }
    pub fn neighbours(&self) -> [Self; 4]
    where
        T: Copy,
    {
        [self.up(), self.down(), self.left(), self.right()]
    }
    pub fn neighbours_and_self_with_diagonals_in_order(&self) -> [Self; 9]
    where
        T: Copy,
    {
        [
            self.down().left(),
            self.down(),
            self.right().down(),
            self.left(),
            *self,
            self.right(),
            self.left().up(),
            self.up(),
            self.up().right(),
        ]
    }
    pub fn neighbours_with_diagonals(&self) -> [Self; 8]
    where
        T: Copy,
    {
        [
            self.up(),
            self.up().right(),
            self.right(),
            self.right().down(),
            self.down(),
            self.down().left(),
            self.left(),
            self.left().up(),
        ]
    }
    pub fn hex_neighbours(&self) -> [Self; 6]
    where
        T: Copy,
    {
        [
            self.up(),
            self.right(),
            self.down().right(),
            self.down(),
            self.left(),
            self.left().up(),
        ]
    }
}

impl<T: Num + Signed + Copy + WrappingSub> Point<T> {
    pub fn manhattan(self) -> T {
        abs(self.x) + abs(self.y)
    }
    pub fn rotate_left_about_origin(&self) -> Self {
        Self::new(-self.y, self.x)
    }
    pub fn rotate_right_about_origin(&self) -> Self {
        Self::new(self.y, -self.x)
    }
    pub fn rotate_180_about_origin(&self) -> Self {
        Self::new(-self.x, -self.y)
    }
}
struct PointStepper<T> {
    curr: Point<T>,
    target: Point<T>,
    pos_x: bool,
    step_x: T,
    pos_y: bool,
    step_y: T,
    inclusive_end: bool,
}
impl<T: Integer + Copy> Iterator for PointStepper<T> {
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.target {
            if self.inclusive_end {
                self.inclusive_end = false;
                Some(self.curr)
            } else {
                None
            }
        } else {
            let ans = self.curr;
            self.curr = ans.safe_step(self.pos_x, self.step_x, self.pos_y, self.step_y);
            Some(ans)
        }
    }
}
impl<T: Integer + Copy> Point<T> {
    pub fn safe_step(self, pos_x: bool, step_x: T, pos_y: bool, step_y: T) -> Self {
        Point::new(
            if pos_x {
                self.x + step_x
            } else {
                self.x - step_x
            },
            if pos_y {
                self.y + step_y
            } else {
                self.y - step_y
            },
        )
    }
    pub fn steps_to(self, end: Self, inclusive_end: bool) -> impl Iterator<Item = Self> {
        let pos_x = end.x > self.x;
        let delta_x = if pos_x {
            end.x - self.x
        } else {
            self.x - end.x
        };
        let pos_y = end.y > self.y;
        let delta_y = if pos_y {
            end.y - self.y
        } else {
            self.y - end.y
        };
        let g = gcd(delta_x, delta_y);
        let step_x = delta_x / g;
        let step_y = delta_y / g;
        PointStepper {
            target: end,
            step_x,
            step_y,
            pos_x,
            pos_y,
            curr: self,
            inclusive_end,
        }
    }
}

impl<T: Num + Unsigned> Point<T> {
    pub fn manhattan_unsigned(self) -> T {
        self.x + self.y
    }
}
impl<T: Mul> Point<T> {
    pub fn area(self) -> <T as Mul>::Output {
        self.x * self.y
    }
}

#[must_use]
pub fn as_point_map<T>(input: &str, increasing_y_is_up: bool) -> HashMap<Point<T>, char>
where
    T: Num + TryFrom<usize> + Hash + Eq + WrappingSub,
    <T as TryFrom<usize>>::Error: Debug,
{
    let boxed: Box<dyn Iterator<Item = _>> = if increasing_y_is_up {
        Box::new(input.lines().rev())
    } else {
        Box::new(input.lines())
    };
    boxed
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::new(x.try_into().unwrap(), y.try_into().unwrap()), c))
        })
        .collect()
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Add + Num> Add for Point<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl<T: WrappingAdd + Num> WrappingAdd for Point<T> {
    fn wrapping_add(&self, v: &Self) -> Self {
        Self::new(self.x.wrapping_add(&v.x), self.y.wrapping_add(&v.y))
    }
}

impl<T: Sub + Num> Sub for Point<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Mul + Copy + Num> Mul<T> for Point<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
impl<T: Mul + Copy + Num + Signed> Mul<T> for Dir {
    type Output = Point<T>;
    fn mul(self, rhs: T) -> Point<T> {
        self.as_point_step() * rhs
    }
}

pub fn point_map_bounding_box<N, T, S>(hm: &HashMap<Point<N>, T, S>) -> Aabb<N>
where
    N: Copy + Num + TryInto<usize> + Ord + WrappingSub,
    RangeInclusive<N>: Iterator<Item = N>,
    S: BuildHasher,
{
    hm.keys().collect()
}

#[must_use]
pub fn render_set_w<N, S>(
    m: &HashSet<Point<N>, S>,
    present: char,
    absent: char,
    flip: bool,
) -> String
where
    S: BuildHasher,
    N: Copy + Num + TryInto<usize> + Ord + Eq + Hash + WrappingSub,
    RangeInclusive<N>: Iterator<Item = N>,
{
    let map: HashMap<Point<N>, char> = m.iter().map(|&p| (p, present)).collect();
    render_char_map_w(&map, 1, &absent.to_string(), flip)
}

pub fn render_char_map_w<N, S, V: Display + Clone + Copy>(
    m: &HashMap<Point<N>, V, S>,
    width: u8,
    default: &str,
    flip: bool,
) -> String
where
    S: BuildHasher,
    N: Copy + Num + TryInto<usize> + Ord + Eq + Hash + WrappingSub,
    RangeInclusive<N>: Iterator<Item = N>,
{
    let bb = point_map_bounding_box(m);
    let v = bb.vec_with(|p| m.get(&p));
    let x = v.iter().map(|l| {
        "\n".to_string()
            + &l.iter()
                .flat_map(|&x| (0..width).map(move |_| x))
                .map(|x| match x {
                    Some(v) => format!("{}", v),
                    None => default.to_owned(),
                })
                .collect::<String>()
    });
    if flip {
        x.rev().collect()
    } else {
        x.collect()
    }
}
