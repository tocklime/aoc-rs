use num::Num;
use parse_display::{Display, FromStr};
use std::{
    cmp::{max, min, Ordering},
    ops::{Add, Range, RangeBounds},
};

#[derive(Display, FromStr, PartialEq, Debug, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
#[must_use]
#[display("{start}..{end}")]
pub struct Span<T> {
    pub start: T,
    pub end: T,
}
#[derive(PartialEq, Eq, Debug)]
pub enum CollisionType<T> {
    Equal,
    Before(Span<T>),
    OverlapsStart(Span<T>, Span<T>, Span<T>),
    StrictlyBigger(Span<T>, Span<T>, Span<T>),
    StrictlySmaller(Span<T>, Span<T>, Span<T>),
    OverlapsEnd(Span<T>, Span<T>, Span<T>),
    After(Span<T>),
}

impl IntoIterator for Span<isize> {
    type Item = isize;

    type IntoIter = Range<isize>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..self.end
    }
}
impl<T: Num + std::ops::AddAssign> Span<T> {
    pub fn make_upper_inclusive(&mut self) {
        self.end += T::one();
    }
    pub fn size(self) -> T {
        self.end - self.start
    }
}
impl<T: Eq + Ord + Copy> Span<T> {
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end);
        Self { start, end }
    }
    pub fn new_from_range(range: impl RangeBounds<T>) -> Self {
        match (range.start_bound(), range.end_bound()) {
            (std::ops::Bound::Included(start), std::ops::Bound::Excluded(end)) => {
                Self::new(*start, *end)
            }
            _ => panic!("Range -> Span must be inclusive at start, exclusive at end"),
        }
    }
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);
        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }
    pub fn union(&self, other: &Self) -> Self {
        Self {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }
    pub fn contains(&self, candidate: T) -> bool {
        candidate >= self.start && candidate < self.end
    }
    pub fn intersects(&self, other: &Self) -> bool {
        !self.is_disjoint(other)
    }
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.end <= other.start || other.end <= self.start
    }
    pub fn is_entirely_within(&self, other: &Self) -> bool {
        matches!(
            self.collide_with(other),
            CollisionType::Equal | CollisionType::StrictlySmaller(_, _, _)
        )
    }

    /// use other to cut self into pieces. both ends of other are cut points, if they are contained in self.
    pub fn cut_by(&self, other: &Self) -> Vec<Self> {
        match self.collide_with(other) {
            CollisionType::Equal | CollisionType::Before(_) | CollisionType::After(_) => {
                vec![*self]
            }
            CollisionType::OverlapsStart(a, b, _) => vec![a, b],
            CollisionType::StrictlyBigger(a, b, c) => vec![a, b, c],
            CollisionType::StrictlySmaller(_, b, _) => vec![b],
            CollisionType::OverlapsEnd(_, b, c) => vec![b, c],
        }
    }
    pub fn subtract(&self, other: &Self) -> Vec<Self> {
        match self.collide_with(other) {
            CollisionType::Equal | CollisionType::StrictlySmaller(_, _, _) => vec![],
            CollisionType::Before(_) | CollisionType::After(_) => vec![*self],
            CollisionType::OverlapsStart(a, b, _) => vec![a, b],
            CollisionType::StrictlyBigger(a, _, c) => vec![a, c],
            CollisionType::OverlapsEnd(_, b, c) => vec![b, c],
        }
    }

    pub fn collide_with(&self, other: &Self) -> CollisionType<T> {
        match (
            self.start.cmp(&other.start),
            self.end.cmp(&other.end),
            self.start.cmp(&other.end),
            self.end.cmp(&other.start),
        ) {
            (Ordering::Equal, Ordering::Equal, _, _) => CollisionType::Equal,
            (Ordering::Less | Ordering::Equal, Ordering::Greater | Ordering::Equal, _, _) => {
                CollisionType::StrictlyBigger(
                    Span::new(self.start, other.start),
                    Span::new(other.start, other.end),
                    Span::new(other.end, self.end),
                )
            }
            (_, _, Ordering::Greater | Ordering::Equal, _) => {
                CollisionType::After(self.union(other))
            }
            (_, _, _, Ordering::Less | Ordering::Equal) => CollisionType::Before(self.union(other)),
            //Conditions above are independent. Conditions below depend on something above not matching.
            (Ordering::Greater | Ordering::Equal, Ordering::Less | Ordering::Equal, _, _) => {
                CollisionType::StrictlySmaller(
                    Span::new(other.start, self.start),
                    Span::new(self.start, self.end),
                    Span::new(self.end, other.end),
                )
            }
            (Ordering::Less, _, _, Ordering::Greater) => CollisionType::OverlapsStart(
                Span::new(self.start, other.start),
                Span::new(other.start, self.end),
                Span::new(self.end, other.end),
            ),
            (_, _, _, Ordering::Greater) => CollisionType::OverlapsEnd(
                Span::new(other.start, self.start),
                Span::new(self.start, other.end),
                Span::new(other.end, self.end),
            ),
        }
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Span<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            start: self.start + rhs,
            end: self.end + rhs,
        }
    }
}
#[cfg(test)]
pub mod test {
    use std::ops::Range;

    use super::*;
    fn do_coll(a: Range<usize>, b: Range<usize>) -> CollisionType<usize> {
        Span::new_from_range(a).collide_with(&Span::new_from_range(b))
    }
    #[test]
    fn test_before() {
        assert_eq!(
            do_coll(10..20, 30..40),
            CollisionType::Before(Span::new(10, 40))
        );
        assert_eq!(
            do_coll(10..30, 30..40),
            CollisionType::Before(Span::new(10, 40))
        );
    }
    #[test]
    fn test_after() {
        assert_eq!(
            do_coll(41..45, 30..40),
            CollisionType::After(Span::new(30, 45))
        );
        assert_eq!(
            do_coll(40..45, 30..40),
            CollisionType::After(Span::new(30, 45))
        );
    }
    #[test]
    fn test_overlap_start() {
        assert_eq!(
            do_coll(10..31, 30..40),
            CollisionType::OverlapsStart(Span::new(10, 30), Span::new(30, 31), Span::new(31, 40))
        );
    }
    #[test]
    fn test_overlap_end() {
        assert_eq!(
            do_coll(35..45, 30..40),
            CollisionType::OverlapsEnd(Span::new(30, 35), Span::new(35, 40), Span::new(40, 45))
        );
    }
    #[test]
    fn test_strictly_bigger() {
        assert_eq!(
            do_coll(10..40, 30..40),
            CollisionType::StrictlyBigger(Span::new(10, 30), Span::new(30, 40), Span::new(40, 40))
        );
        assert_eq!(
            do_coll(10..45, 30..40),
            CollisionType::StrictlyBigger(Span::new(10, 30), Span::new(30, 40), Span::new(40, 45))
        );
        assert_eq!(
            do_coll(30..45, 30..40),
            CollisionType::StrictlyBigger(Span::new(30, 30), Span::new(30, 40), Span::new(40, 45))
        );
    }
    #[test]
    fn test_equal() {
        assert_eq!(do_coll(30..40, 30..40), CollisionType::Equal);
    }
    #[test]
    fn test_strictly_smaller() {
        assert_eq!(
            do_coll(30..39, 30..40),
            CollisionType::StrictlySmaller(Span::new(30, 30), Span::new(30, 39), Span::new(39, 40))
        );
        assert_eq!(
            do_coll(31..40, 30..40),
            CollisionType::StrictlySmaller(Span::new(30, 31), Span::new(31, 40), Span::new(40, 40),)
        );
        assert_eq!(
            do_coll(31..39, 30..40),
            CollisionType::StrictlySmaller(Span::new(30, 31), Span::new(31, 39), Span::new(39, 40),)
        );
    }
}
