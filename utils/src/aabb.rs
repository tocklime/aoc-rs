use crate::cartesian::Point;
use num::{traits::WrappingSub, Num};
use std::cmp::{max, min};
use std::convert::TryInto;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use]
pub struct Aabb<T> {
    /// inclusive
    pub bottom_left: Point<T>,
    /// inclusive
    pub top_right: Point<T>,
}

impl<T> Aabb<T>
where
    T: Num + Copy + TryInto<usize> + Ord + WrappingSub,
    RangeInclusive<T>: std::iter::Iterator<Item = T>,
{
    pub fn new(p: Point<T>) -> Self {
        Self {
            bottom_left: p,
            top_right: p,
        }
    }
    pub fn origin_and(p: Point<T>) -> Self {
        Self {
            bottom_left: Point::origin(),
            top_right: p,
        }
    }
    pub fn area(&self) -> usize {
        self.width() * self.height()
    }
    pub fn corners(&self) -> [Point<T>; 4] {
        let tr = self.top_right;
        let bl = self.bottom_left;
        [bl, tr, Point::new(bl.x, tr.y), Point::new(tr.x, bl.y)]
    }
    pub fn quadrants(&self) -> [Aabb<T>; 4] {
        let mid_point = self.center();
        [
            // 34
            // 12
            Self {
                bottom_left: self.bottom_left,
                top_right: mid_point,
            },
            Self {
                bottom_left: Point::new(mid_point.x + T::one(), self.bottom_left.y),
                top_right: Point::new(self.top_right.x, mid_point.y),
            },
            Self {
                bottom_left: Point::new(self.bottom_left.x, mid_point.y + T::one()),
                top_right: Point::new(mid_point.x, self.top_right.y),
            },
            Self {
                bottom_left: mid_point + Point::new(T::one(), T::one()),
                top_right: self.top_right,
            },
        ]
    }

    pub fn center(&self) -> Point<T> {
        let two = T::one() + T::one();
        let x = (self.bottom_left.x + self.top_right.x) / two;
        let y = (self.bottom_left.y + self.top_right.y) / two;
        Point::new(x, y)
    }

    pub fn extend(&self, p: Point<T>) -> Self {
        let mut ans = *self;
        ans.bottom_left.x = min(ans.bottom_left.x, p.x);
        ans.bottom_left.y = min(ans.bottom_left.y, p.y);
        ans.top_right.x = max(ans.top_right.x, p.x);
        ans.top_right.y = max(ans.top_right.y, p.y);
        ans
    }
    pub fn grow(&self, n: T) -> Self {
        let mut ans = *self;
        ans.bottom_left.x = ans.bottom_left.x - n;
        ans.bottom_left.y = ans.bottom_left.y - n;
        ans.top_right.x = ans.top_right.x + n;
        ans.top_right.y = ans.top_right.y + n;
        ans
    }
    pub fn contains(&self, p: &Point<T>) -> bool {
        self.bottom_left.x <= p.x
            && self.bottom_left.y <= p.y
            && self.top_right.x >= p.x
            && self.top_right.y >= p.y
    }
    pub fn extend_box(&self, b: Self) -> Self {
        self.extend(b.bottom_left).extend(b.top_right)
    }
    pub fn intersect(&self, b: Self) -> Self {
        Self {
            bottom_left: Point::new(
                max(self.bottom_left.x, b.bottom_left.x),
                max(self.bottom_left.y, b.bottom_left.y),
            ),
            top_right: Point::new(
                min(self.top_right.x, b.top_right.x),
                min(self.top_right.y, b.top_right.y),
            ),
        }
    }
    pub fn is_valid(&self) -> bool {
        self.bottom_left.x <= self.top_right.x && self.bottom_left.y <= self.top_right.y
    }
    pub fn all_points(&self) -> impl Iterator<Item = Point<T>> + '_ {
        (self.bottom_left.y..=self.top_right.y).flat_map(move |y| {
            (self.bottom_left.x..=self.top_right.x).map(move |x| Point::new(x, y))
        })
    }
    /// All points around the edge of the Aabb.
    pub fn perimeter(&self) -> impl Iterator<Item = Point<T>> + '_ {
        let x_start = self.bottom_left.x;
        let x_end = self.top_right.x;
        (x_start..=x_end)
            .map(|x| Point::new(x, self.bottom_left.y))
            .chain(
                (self.bottom_left.y + T::one()..=self.top_right.y - T::one())
                    .flat_map(move |y| [Point::new(x_start, y), Point::new(x_end, y)]),
            )
            .chain((x_start..=x_end).map(|x| Point::new(x, self.top_right.y)))
    }
    pub fn vec_with<TO: Clone + Default>(&self, ft: impl Fn(Point<T>) -> TO) -> Vec<Vec<TO>> {
        let offset = self.bottom_left;
        let mut v = vec![vec![Default::default(); self.width()]; self.height()];
        for p in self.all_points() {
            let rel = p - offset;
            let x: usize = Self::t_as_usize(rel.x);
            let y: usize = Self::t_as_usize(rel.y);
            v[y][x] = ft(p);
        }
        v
    }
    fn t_as_usize(t: T) -> usize {
        match t.try_into().ok() {
            Some(x) => x,
            None => panic!("Can't convert to usize"),
        }
    }
    pub fn width(&self) -> usize {
        Self::t_as_usize(T::one() + self.top_right.x - self.bottom_left.x)
    }
    pub fn height(&self) -> usize {
        Self::t_as_usize(T::one() + self.top_right.y - self.bottom_left.y)
    }
}
impl<'a, T> FromIterator<&'a Point<T>> for Aabb<T>
where
    T: 'a + Num + Copy + TryInto<usize> + Ord + WrappingSub,
    RangeInclusive<T>: std::iter::Iterator<Item = T>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a Point<T>>,
    {
        let mut i = iter.into_iter();
        let b = Self::new(*i.next().expect("Non empty iterator"));
        i.fold(b, |b, n| b.extend(*n))
    }
}

impl<'a, T> FromIterator<Point<T>> for Aabb<T>
where
    T: 'a + Num + Copy + TryInto<usize> + Ord + WrappingSub,
    RangeInclusive<T>: std::iter::Iterator<Item = T>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Point<T>>,
    {
        let mut i = iter.into_iter();
        let b = Self::new(i.next().expect("Non empty iterator"));
        i.fold(b, |b, n| b.extend(n))
    }
}
