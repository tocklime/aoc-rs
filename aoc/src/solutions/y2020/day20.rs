use aoc_harness::aoc_main;

aoc_main!(2020 day 20, part1 [p1], part2 [p2]);
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
};
use utils::collections::ToLookup;

use itertools::Itertools;
use ndarray::Array2;

#[derive(Debug, PartialEq, Eq)]
pub struct Tile {
    id: usize,
    map: Array2<bool>,
}

pub const fn given_a_is_b_what_is_c(a: u8, b: u8, c: u8, flip: bool) -> u8 {
    (if flip {
        b.wrapping_sub(a).wrapping_add(c)
    } else {
        b.wrapping_add(a).wrapping_sub(c)
    }) % 4
}

#[derive(Copy, Clone)]
pub struct Placement<'a> {
    tile: &'a Tile,
    rotations: u8,
    flip: bool,
}

impl<'a> Placement<'a> {
    pub const fn get_edge(&self, edge: u8) -> u8 {
        given_a_is_b_what_is_c(self.rotations, 0, edge, self.flip)
    }
    pub const fn put_edge_a_on_x(tile: &'a Tile, a: u8, x: u8, flip: bool) -> Self {
        let rotations = given_a_is_b_what_is_c(a, x, 0, flip);
        Self { tile, rotations, flip }
    }
    fn edge_matches(&self, edge: u8, connections: &HashMap<usize, Vec<Self>>) -> Vec<Self> {
        //we want `edge` edge, after rotating.
        let r_edge = (self.rotations + edge) % 4;
        let desired_edge = (if self.flip { 4 - r_edge } else { r_edge }) % 4;

        let edge_pattern = self.tile.get_edge_id(desired_edge, self.flip);
        let opposite_edge = match edge {
            0 => 2,
            1 => 1,
            2 => 0,
            3 => 3,
            _ => panic!(),
        };
        connections[&edge_pattern]
            .iter()
            .filter(|opt| opt.tile != self.tile)
            .map(|&opt| Placement::put_edge_a_on_x(opt.tile, opt.rotations, opposite_edge, !opt.flip))
            .collect::<Vec<_>>()
    }
    #[allow(dead_code)]
    fn get_edge_pattern(&self, edge: u8) -> usize {
        self.tile.get_edge_id(self.get_edge(edge), self.flip)
    }
}
impl<'a> Debug for Placement<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Placement")
            .field("tile.id", &self.tile.id)
            .field("rotations", &self.rotations)
            .field("flip", &self.flip)
            .finish()
    }
}

impl Tile {
    fn get_edge_id(&self, n: u8, flip: bool) -> usize {
        let (x, rev) = match (n, flip) {
            (0, false) => (self.map.row(0), false),
            (1, false) => (self.map.column(9), false),
            (2, false) => (self.map.row(9), true),
            (3, false) => (self.map.column(0), true),
            (0, true) => (self.map.row(0), true),
            (3, true) => (self.map.column(0), false),
            (2, true) => (self.map.row(9), false),
            (1, true) => (self.map.column(9), true),
            _ => panic!("where is edge {}?", n),
        };
        if rev {
            x.iter().rev().enumerate().map(|(ix, val)| ((*val as usize) << ix)).sum()
        } else {
            x.iter().enumerate().map(|(ix, val)| ((*val as usize) << ix)).sum()
        }
    }
    fn get_all_possible_ids<'a>(&'a self) -> impl Iterator<Item = (usize, Placement<'a>)> + 'a {
        [false, true].iter().flat_map(move |&flip| {
            (0..4).map(move |rotations| {
                (
                    self.get_edge_id(rotations, flip),
                    Placement {
                        tile: self,
                        rotations,
                        flip,
                    },
                )
            })
        })
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut l = s.lines();
        let id = l.next().unwrap()[5..9].parse().unwrap();
        let mut map = Array2::from_elem((10, 10), false);
        for (r, line) in l.enumerate() {
            for (c, x) in line.trim().chars().enumerate() {
                map[(r, c)] = x == '#';
            }
        }
        Ok(Self { id, map })
    }
}

fn flatten(arr: &Array2<Option<Placement>>) -> Array2<bool> {
    Array2::from_shape_fn((arr.nrows() * 8, arr.ncols() * 8), |(r, c)| {
        //r and c are world coord.
        let p = arr[(r / 8, c / 8)].unwrap();
        let (int_r, int_c) = (r % 8 + 1, c % 8 + 1);
        let (r2, c2) = match (p.rotations, p.flip) {
            (0, false) => (int_r, int_c),
            (3, false) => (9 - int_c, int_r),
            (2, false) => (9 - int_r, 9 - int_c),
            (1, false) => (int_c, 9 - int_r),
            (0, true) => (int_r, 9 - int_c),
            (3, true) => (9 - int_c, 9 - int_r),
            (2, true) => (9 - int_r, int_c),
            (1, true) => (int_c, int_r),
            _ => panic!(),
        };
        p.tile.map[(r2, c2)]
    })
}
#[allow(dead_code)]
fn print_simple_grid(arr: &Array2<bool>) {
    for r in 0..arr.nrows() {
        for c in 0..arr.ncols() {
            print!("{}", if arr[(r, c)] { '#' } else { '.' });
        }
        println!();
    }
    println!();
}
#[allow(dead_code)]
fn print_grid(arr: &Array2<Option<Placement>>) {
    let wid = 10;
    for r in 0..wid * arr.nrows() {
        let int_r = r % 10;
        for c in 0..wid * arr.ncols() {
            let p = arr.get((r / 10, c / 10)).unwrap();
            let int_c = c % 10;
            if let Some(p) = p {
                //need to transform int_r and int_c.
                let (r2, c2) = match (p.rotations, p.flip) {
                    (0, false) => (int_r, int_c),
                    (3, false) => (9 - int_c, int_r),
                    (2, false) => (9 - int_r, 9 - int_c),
                    (1, false) => (int_c, 9 - int_r),
                    (0, true) => (int_r, 9 - int_c),
                    (3, true) => (9 - int_c, 9 - int_r),
                    (2, true) => (9 - int_r, int_c),
                    (1, true) => (int_c, int_r),
                    _ => panic!("bad rotation: {:?}", p),
                };
                print!("{}", if p.tile.map[(r2, c2)] { '#' } else { '.' });
            } else {
                print!("_");
            }
            if int_c == 9 {
                print!(" ");
            }
        }
        println!();
        if int_r == 9 {
            println!();
        }
    }
}
fn rotate_arr<T: Copy>(arr: &Array2<T>) -> Array2<T> {
    let nc = arr.ncols();
    Array2::from_shape_fn((arr.ncols(), arr.nrows()), |(r, c)| arr[(c, nc - 1 - r)])
}

fn flip_arr<T: Copy>(arr: &Array2<T>) -> Array2<T> {
    let nc = arr.ncols();
    Array2::from_shape_fn(arr.raw_dim(), |(r, c)| arr[(r, nc - 1 - c)])
}
fn mark_monster_tiles(marks: &mut Array2<bool>, arr: &Array2<bool>, monster: &Array2<bool>) {
    for r in 0..arr.nrows() - monster.nrows() {
        for c in 0..arr.ncols() - monster.ncols() {
            let mut found_monster = true;
            for ((mr, mc), x) in monster.indexed_iter() {
                if *x {
                    if let Some(wave) = arr.get((r + mr, c + mc)) {
                        if !*wave {
                            found_monster = false;
                            break;
                        }
                    } else {
                        //outside grid, fail this windo
                        found_monster = false;
                        break;
                    }
                }
            }
            if found_monster {
                for ((mr, mc), x) in monster.indexed_iter() {
                    if *x {
                        marks[(r + mr, c + mc)] = true;
                    }
                }
            }
        }
    }
}
fn p1(input: &str) -> usize {
    let tiles = input
        .trim()
        .replace("\r", "")
        .split("\n\n")
        .map(|x| x.parse::<Tile>().unwrap())
        .collect::<Vec<_>>();
    let connections: HashMap<usize, Vec<Placement>> = tiles.iter().flat_map(Tile::get_all_possible_ids).collect_lookup();

    let map = connections
        .values()
        .flat_map(move |v| v.iter().permutations(2).map(move |x| (x[0].tile.id, ())))
        .collect_lookup();
    let corners: Vec<usize> = map.iter().filter(|x| x.1.len() == 4).map(|x| *x.0).collect();
    corners.iter().product()
}

fn p2(input: &str) -> usize {
    let tiles = input
        .trim()
        .replace("\r", "")
        .split("\n\n")
        .map(|x| x.parse::<Tile>().unwrap())
        .collect::<Vec<_>>();
    let connections: HashMap<usize, Vec<Placement>> = tiles.iter().flat_map(Tile::get_all_possible_ids).collect_lookup();

    let map: HashMap<usize, Vec<(&Placement, &Placement)>> = connections
        .values()
        .flat_map(move |v| v.iter().permutations(2).map(move |x| (x[0].tile.id, (x[0], x[1]))))
        .collect_lookup();
    let corner_p = map.iter().find(|x| x.1.len() == 4).unwrap().1;
    let c = corner_p[0].0.tile;
    let wid = if tiles.len() > 10 { 12 } else { 3 };
    let mut array: Array2<Option<Placement>> = Array2::from_elem((wid, wid), None);
    //wlog, lets just stick one corner down. Which two edges are unused?
    let dirs: HashSet<u8> = map[&c.id].iter().filter(|x| !x.0.flip).map(|x| x.0.rotations).collect();
    let orientation = match (dirs.contains(&0), dirs.contains(&1), dirs.contains(&2), dirs.contains(&3)) {
        (false, false, true, true) => 1,
        (false, true, true, false) => 0,
        (true, true, false, false) => 3,
        (true, false, false, true) => 2,
        _ => panic!("This isn't a corner piece!"),
    };
    array[(0, 0)] = Some(Placement {
        tile: c,
        rotations: orientation,
        flip: false,
    });
    let mut fringe: Vec<(usize, usize)> = vec![(1, 0), (0, 1)].into_iter().collect();
    let mut used = HashSet::new();
    //println!("Started with {:?} at 0 0",array[(0,0)]);
    used.insert(c.id);
    while let Some((r, c)) = fringe.pop() {
        let a = array.get((r, c));
        if a.is_none() || a.unwrap().is_some() {
            //outside grid, or already set.
            continue;
        }
        let neighbours: Vec<(usize, usize)> = vec![(r.wrapping_sub(1), c), (r, c + 1), (r + 1, c), (r, c.wrapping_sub(1))];
        let mut found = false;
        for d in 0..4 {
            let adj_arr_ix = neighbours[d as usize];
            if let Some(Some(adj_placement)) = array.get(adj_arr_ix).copied() {
                let candidates = adj_placement.edge_matches((d + 2) % 4, &connections);
                let filtered = candidates.iter().filter(|p| !used.contains(&p.tile.id)).collect::<Vec<_>>();
                if filtered.len() == 1 {
                    let my_placement = *filtered[0];
                    *array.get_mut((r, c)).unwrap() = Some(my_placement);
                    used.insert(my_placement.tile.id);
                    //just placed at r,c. add neighbours to fringe.
                    fringe.extend(&neighbours);
                    found = true;
                    break;
                } else if filtered.is_empty() {
                    panic!("no possible matches for {:?}", (r, c));
                }
            }
        }
        if !found {
            panic!("Can't find piece for space");
        }
    }
    let a = flatten(&array);
    let monster = "                  # #    ##    ##    ### #  #  #  #  #  #   "
        .chars()
        .map(|x| x == '#')
        .collect::<Vec<_>>();
    let mut monster_arr = Array2::from_shape_vec((3, 20), monster).unwrap();
    let mut monster_marks = Array2::from_elem(a.raw_dim(), false);
    for _ in 0..4 {
        mark_monster_tiles(&mut monster_marks, &a, &monster_arr);
        let flipped = flip_arr(&monster_arr);
        mark_monster_tiles(&mut monster_marks, &a, &flipped);
        monster_arr = rotate_arr(&monster_arr);
    }
    let mm = monster_marks.iter().filter(|x| **x).count();
    let waves = a.iter().filter(|x| **x).count();
    waves - mm
}
