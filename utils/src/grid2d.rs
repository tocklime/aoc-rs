use std::{
    fmt::{Display, Write},
    iter,
    ops::{Index, IndexMut},
};

use itertools::Itertools;
use num::Integer;

use crate::{aabb::Aabb, cartesian::Point, nums::add_i_mod};

#[derive(Debug, Clone)]
pub struct Grid2d<T> {
    data: Vec<T>,
    size: Coord,
}

pub type Coord = (usize, usize);
pub type ICoord = (isize, isize);
impl<T: Copy> Grid2d<T> {
    pub fn from_elem(size: Coord, elem: T) -> Self {
        Self {
            data: vec![elem; size.0 * size.1],
            size,
        }
    }
}
impl<T> Grid2d<T> {
    pub fn from_fn<F>(size: Coord, mut f: F) -> Self
    where
        F: FnMut(Coord) -> T,
    {
        let mut data = Vec::with_capacity(size.0 * size.1);
        for a in 0..size.0 {
            for b in 0..size.1 {
                data.push(f((a, b)));
            }
        }
        Self { data, size }
    }
}
impl<T> Index<Point<usize>> for Grid2d<T> {
    type Output = T;

    fn index(&self, index: Point<usize>) -> &Self::Output {
        &self[(index.y, index.x)]
    }
}
impl<T> IndexMut<Point<usize>> for Grid2d<T> {
    fn index_mut(&mut self, index: Point<usize>) -> &mut Self::Output {
        &mut self[(index.y, index.x)]
    }
}
impl<T> Index<usize> for Grid2d<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for Grid2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl<T> Index<Coord> for Grid2d<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.data[index.0 * self.size.1 + index.1]
    }
}
impl<T> IndexMut<Coord> for Grid2d<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.data[index.0 * self.size.1 + index.1]
    }
}
impl<T: Display> Display for Grid2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ((_, x), t) in self.indexed_iter() {
            f.write_fmt(format_args!("{t}"))?;
            if x == self.size.1 - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}
impl<T: Copy> Grid2d<T> {
    pub fn grow_and_invalidate_all_data(&mut self, new_size: Coord, new_t: T) {
        self.size = new_size;
        let need_len = self.size.0 * self.size.1;
        self.data.extend((self.data.len()..need_len).map(|_| new_t));
    }
}
impl<T: PartialEq<T>> Grid2d<T> {
    pub fn insert(&mut self, p: Coord, val: T) -> bool {
        let x = &mut self[p];
        if *x != val {
            *x = val;
            true
        } else {
            false
        }
    }
}
impl<T> Grid2d<T> {
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    #[must_use]
    pub fn get(&self, p: Coord) -> Option<&T> {
        if p.0 < self.size.0 && p.1 < self.size.1 {
            Some(&self[p])
        } else {
            None
        }
    }
    #[must_use]
    pub fn get_mut(&mut self, p: Coord) -> Option<&mut T> {
        if p.0 < self.size.0 && p.1 < self.size.1 {
            Some(&mut self[p])
        } else {
            None
        }
    }
    #[must_use] pub fn to_u(p: ICoord) -> Option<Coord> {
        Some((p.0.try_into().ok()?, p.1.try_into().ok()?))
    }
    #[must_use]
    pub fn get_i(&self, p: ICoord) -> Option<&T> {
        Self::to_u(p).and_then(|p| self.get(p))
    }
    #[must_use]
    pub fn dim(&self) -> Coord {
        self.size
    }
    pub fn indexes(&'_ self) -> impl Iterator<Item = Coord> {
        let max = self.size;
        (0..max.0).cartesian_product(0..max.1)
    }
    pub fn indexed_iter(&self) -> impl Iterator<Item = (Coord, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(x, v)| (x.div_mod_floor(&self.size.1), v))
    }
    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (Coord, &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(x, v)| (x.div_mod_floor(&self.size.1), v))
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
    #[must_use]
    pub fn ordered_neighbours_with_self(&self, p: Coord) -> [Option<Coord>; 9] {
        let s = self.dim();
        let up = p.0.checked_sub(1);
        let left = p.1.checked_sub(1);
        let down = if p.0 + 1 == s.0 { None } else { Some(p.0 + 1) };
        let right = if p.1 + 1 == s.1 { None } else { Some(p.1 + 1) };
        [
            up.and_then(|y| left.map(|x| (y, x))),
            up.map(|y| (y, p.1)),
            up.and_then(|y| right.map(|x| (y, x))),
            left.map(|x| (p.0, x)),
            Some(p),
            right.map(|x| (p.0, x)),
            down.and_then(|y| left.map(|x| (y, x))),
            down.map(|y| (y, p.1)),
            down.and_then(|y| right.map(|x| (y, x))),
        ]
    }
    pub fn neighbours_with_diagonals(&'_ self, p: Coord) -> impl Iterator<Item = Coord> {
        let s = self.dim();
        [
            (p.0.wrapping_sub(1), p.1),
            (p.0, p.1.wrapping_sub(1)),
            (p.0 + 1, p.1),
            (p.0, p.1 + 1),
            (p.0.wrapping_sub(1), p.1.wrapping_sub(1)),
            (p.0 + 1, p.1.wrapping_sub(1)),
            (p.0 + 1, p.1 + 1),
            (p.0.wrapping_sub(1), p.1 + 1),
        ]
        .into_iter()
        .filter(move |&x| x.0 < s.0 && x.1 < s.1)
    }
    pub fn neighbours(&'_ self, p: Coord) -> impl Iterator<Item = Coord> {
        let s = self.dim();
        [
            (p.0.wrapping_sub(1), p.1),
            (p.0, p.1.wrapping_sub(1)),
            (p.0 + 1, p.1),
            (p.0, p.1 + 1),
        ]
        .into_iter()
        .filter(move |&x| x.0 < s.0 && x.1 < s.1)
    }

    /// Returns all values in the grid by taking steps of `relative` from `start`.
    /// Includes the value at `start`.
    pub fn values_in_direction(
        &self,
        start: Coord,
        relative: ICoord,
    ) -> impl Iterator<Item = (Coord, &T)> {
        let mut pos: ICoord = (start.0 as isize, start.1 as isize);
        iter::from_fn(move || {
            let here = Self::to_u(pos)?;
            pos.0 += relative.0;
            pos.1 += relative.1;
            let next = self.get(here);
            next.map(|p| (here, p))
        })
    }
    #[must_use]
    pub fn get_row(&self, y: usize) -> &[T] {
        let w = self.size.1;
        &self.data[y * w..(y + 1) * w]
    }
    #[must_use]
    pub fn relative_lookup(&self, p: Coord, relative: ICoord) -> Option<&T> {
        let y = if relative.0 > 0 {
            p.0.wrapping_add(relative.0 as usize)
        } else {
            p.0.wrapping_sub((-relative.0) as usize)
        };
        let x = if relative.1 > 0 {
            p.1.wrapping_add(relative.1 as usize)
        } else {
            p.1.wrapping_sub((-relative.1) as usize)
        };
        self.get((y, x))
    }
    #[must_use]
    pub fn wraparound_relative_lookup(&self, p: Coord, relative: ICoord) -> &T {
        let d = self.dim();
        let y = add_i_mod(p.0, &relative.0, d.0);
        let x = add_i_mod(p.1, &relative.1, d.1);
        &self[(y, x)]
    }
    #[must_use]
    pub fn wraparound_neighbours(&self, (y, x): Coord) -> [Coord; 4] {
        let (sy, sx) = self.dim();
        [
            ((y + sy - 1) % sy, x),
            (y, (x + sx - 1) % sx),
            (y, (x + 1) % sx),
            ((y + 1) % sy, x),
        ]
    }
    pub fn to_string_with<F>(&self, disp: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut ans = String::with_capacity(self.data.len());
        for ((_, x), t) in self.indexed_iter() {
            ans.push_str(&disp(t));
            if x == self.size.1 - 1 {
                ans.push('\n');
            }
        }
        ans
    }
    pub fn find_bb<F>(&self, pred: F) -> Aabb<usize>
    where
        F: Fn(&T) -> bool,
    {
        self.indexed_iter()
            .filter(|&(_, x)| pred(x))
            .map(|((y, x), _)| Point::new(x, y))
            .collect()
    }
    pub fn render_section_with<F>(&self, bb: Aabb<usize>, disp: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut ans = String::with_capacity(bb.area());
        for y in bb.bottom_left.y..=bb.top_right.y {
            for x in bb.bottom_left.x..=bb.top_right.x {
                ans.push_str(&disp(&self[(y, x)]));
            }
            ans.push('\n');
        }
        ans
    }
    pub fn from_str<F>(input: &str, conv: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let mut stride = None;
        let mut data = Vec::with_capacity(input.len());
        let mut rows = 0;
        for l in input.lines() {
            rows += 1;
            match (stride, l.len()) {
                (None, l) => stride = Some(l),
                (Some(a), b) if a != b => panic!("Not equal line lengths"),
                _ => {}
            }
            for c in l.chars() {
                data.push(conv(c));
            }
        }
        Self {
            data,
            size: (rows, stride.unwrap()),
        }
    }
    pub fn from_str_with_index<F>(input: &str, mut conv: F) -> Self
    where
        F: FnMut(Coord, char) -> T,
    {
        let mut stride = None;
        let mut data = Vec::with_capacity(input.len());
        let mut rows = 0;
        for (row, l) in input.lines().enumerate() {
            match (stride, l.len()) {
                (None, l) => stride = Some(l),
                (Some(a), b) if a != b => panic!("Not equal line lengths"),
                _ => {}
            }
            for (col, c) in l.chars().enumerate() {
                data.push(conv((row, col), c));
            }
            rows += 1;
        }
        Self {
            data,
            size: (rows, stride.unwrap()),
        }
    }
    pub fn map<F, TO>(&self, mut f: F) -> Grid2d<TO>
    where
        F: FnMut(Coord, &T) -> TO,
    {
        Grid2d::from_fn(self.dim(), |p| f(p, &self[p]))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn values_in_direction_test() {
        const GRID: &str = "012
345
678";
        let g = Grid2d::from_str(GRID, |c| c);
        assert_eq!(
            g.values_in_direction((1, 1), (1, 0))
                .map(|x| *x.1)
                .collect_vec(),
            ['4', '7']
        );
        assert_eq!(
            g.values_in_direction((1, 1), (-1, 0))
                .map(|x| *x.1)
                .collect_vec(),
            ['4', '1']
        );
        assert_eq!(
            g.values_in_direction((2, 0), (-1, 0))
                .map(|x| *x.1)
                .collect_vec(),
            ['6', '3', '0']
        );
        assert_eq!(
            g.values_in_direction((0, 2), (-1, 0))
                .map(|x| *x.1)
                .collect_vec(),
            ['2']
        );
        assert_eq!(
            g.values_in_direction((0, 2), (0, -1))
                .map(|x| *x.1)
                .collect_vec(),
            ['2', '1', '0']
        );
        assert_eq!(
            g.values_in_direction((0, 0), (0, 1))
                .map(|x| *x.1)
                .collect_vec(),
            ['0', '1', '2']
        );
    }
}
