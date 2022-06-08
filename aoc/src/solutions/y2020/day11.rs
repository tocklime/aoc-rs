use aoc_harness::aoc_main;

aoc_main!(2020 day 11, generator gen_grid,
    part1 [p1_ndarray_clone, p1_ndarray_mut, p1_ndarray_two],
    part2 [p2_ndarray_clone, p2_ndarray_mut, p2_ndarray_two]);
use itertools::{iterate, Itertools};
use ndarray::Array2;
use num::traits::WrappingAdd;

use utils::{
    cartesian::Point,
    cellular_automata::{mut_grid, step_grid, step_grid_into},
};

fn gen_grid(input: &str) -> Array2<Option<bool>> {
    let cols = input.lines().next().unwrap().len();
    let v = input
        .lines()
        .flat_map(str::chars)
        .map(|c| match c {
            'L' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .collect_vec();
    Array2::from_shape_vec((v.len() / cols, cols), v).unwrap()
}

fn p1_ndarray_mut(input: &Array2<Option<bool>>) -> usize {
    let mut g = input.clone();
    while mut_grid(&mut g, |g, old, (r, c)| {
        match old {
            None => None,
            Some(occ) => {
                let count = Point::new(c, r)
                    .neighbours_with_diagonals()
                    .iter()
                    .filter(|p| g.get((p.y, p.x)) == Some(&Some(true)))
                    .count();
                //is occupied if no visible occupied, or is already and not too many visible occupied.
                Some(count == 0 || *occ && count < 4)
            }
        }
    }) > 0
    {}
    g.iter().filter(|&&x| x == Some(true)).count()
}

fn p2_ndarray_mut(input: &Array2<Option<bool>>) -> usize {
    let mut g = input.clone();
    let lookups: Array2<Vec<Point<usize>>> =
        Array2::from_shape_fn(g.raw_dim(), |(r, c)| -> Vec<Point<usize>> {
            let p = Point::new(c, r);
            Point::new(0, 0)
                .neighbours_with_diagonals()
                .iter()
                .filter_map(|d| {
                    iterate(p.wrapping_add(d), |&p| p.wrapping_add(d))
                        .map(|p| (p, g.get((p.y, p.x))))
                        .take_while(|x| x.1.is_some())
                        .find_map(|x| {
                            if x.1.unwrap().is_some() {
                                Some(x.0)
                            } else {
                                None
                            }
                        })
                })
                .collect()
        });

    while mut_grid(&mut g, |g, old, pos| {
        match old {
            None => None,
            Some(occ) => {
                let ps = &lookups[pos];
                let count = ps.iter().filter(|p| g[(p.y, p.x)] == Some(true)).count();
                //is occupied if no visible occupied, or is already and not too many visible occupied.
                Some(count == 0 || *occ && count < 5)
            }
        }
    }) > 0
    {}
    g.iter().filter(|&&x| x == Some(true)).count()
}

fn p1_ndarray_clone(input: &Array2<Option<bool>>) -> usize {
    let mut g = input.clone();
    loop {
        let (new_g, x) = step_grid(&g, |g, old, (r, c)| {
            old.map(|occ| {
                //is occupied if no visible occupied, or is already and not too many visible occupied.
                //that is, if occupied, then less than 4 adjacent are occupied
                //if not occupied, then only if no visible become occupied.
                Point::new(c, r)
                    .neighbours_with_diagonals()
                    .iter()
                    .filter(|p| g.get((p.y, p.x)) == Some(&Some(true)))
                    .nth(if occ { 3 } else { 0 })
                    .is_none()
            })
        });
        if x == 0 {
            break;
        }
        g = new_g;
    }
    g.iter().filter(|&&x| x == Some(true)).count()
}

fn p2_ndarray_clone(input: &Array2<Option<bool>>) -> usize {
    let mut g = input.clone();
    let lookups: Array2<Vec<Point<usize>>> =
        Array2::from_shape_fn(g.raw_dim(), |(r, c)| -> Vec<Point<usize>> {
            let p = Point::new(c, r);
            Point::new(0, 0)
                .neighbours_with_diagonals()
                .iter()
                .filter_map(|d| {
                    iterate(p.wrapping_add(d), |&p| p.wrapping_add(d))
                        .map(|p| (p, g.get((p.y, p.x))))
                        .take_while(|x| x.1.is_some())
                        .find_map(|x| match x {
                            (p, Some(Some(_))) => Some(p),
                            _ => None,
                        })
                })
                .collect()
        });

    loop {
        let (new_g, changes) = step_grid(&g, |g, old, pos| {
            match old {
                None => None,
                &Some(occ) => {
                    let mut it = lookups[pos].iter().filter(|p| g[(p.y, p.x)] == Some(true));
                    //is occupied if no visible occupied, or is already and not too many visible occupied.
                    //that is, if occupied, is nth(4) none?. If not occupied, then is nth(0) none
                    Some(it.nth(if occ { 4 } else { 0 }).is_none())
                }
            }
        });
        if changes == 0 {
            break;
        }
        g = new_g;
    }
    g.iter().filter(|&&x| x == Some(true)).count()
}

fn p1_ndarray_two(input: &Array2<Option<bool>>) -> usize {
    let mut g1 = input.clone();
    let mut g2 = input.clone();
    let g1_ref = &mut g1;
    let g2_ref = &mut g2;
    while step_grid_into(g1_ref, g2_ref, |g, old, (r, c)| {
        old.map(|occ|
                //is occupied if no visible occupied, or is already and not too many visible occupied.
                //that is, if occupied, then less than 4 adjacent are occupied
                //if not occupied, then only if no visible become occupied.
                    Point::new(c, r)
                        .neighbours_with_diagonals()
                        .iter()
                        .filter(|p| g.get((p.y, p.x)) == Some(&Some(true)))
                        .nth(if occ { 3 } else { 0 })
                        .is_none())
    }) > 0
    {
        std::mem::swap(g1_ref, g2_ref);
    }
    g1.iter().filter(|&&x| x == Some(true)).count()
}

fn p2_ndarray_two(input: &Array2<Option<bool>>) -> usize {
    let mut g1 = input.clone();
    let mut g2 = input.clone();
    let g1_ref = &mut g1;
    let g2_ref = &mut g2;
    let lookups: Array2<Vec<Point<usize>>> =
        Array2::from_shape_fn(g1_ref.raw_dim(), |(r, c)| -> Vec<Point<usize>> {
            let p = Point::new(c, r);
            Point::new(0, 0)
                .neighbours_with_diagonals()
                .iter()
                .filter_map(|d| {
                    iterate(p.wrapping_add(d), |&p| p.wrapping_add(d))
                        .map(|p| (p, g1_ref.get((p.y, p.x))))
                        .take_while(|x| x.1.is_some())
                        .find_map(|x| match x {
                            (p, Some(Some(_))) => Some(p),
                            _ => None,
                        })
                })
                .collect()
        });

    while step_grid_into(g1_ref, g2_ref, |g, old, pos| {
        match old {
            None => None,
            &Some(occ) => {
                let mut it = lookups[pos].iter().filter(|p| g[(p.y, p.x)] == Some(true));
                //is occupied if no visible occupied, or is already and not too many visible occupied.
                //that is, if occupied, is nth(4) none?. If not occupied, then is nth(0) none
                Some(it.nth(if occ { 4 } else { 0 }).is_none())
            }
        }
    }) > 0
    {
        std::mem::swap(g1_ref, g2_ref);
    }
    g1.iter().filter(|&&x| x == Some(true)).count()
}
