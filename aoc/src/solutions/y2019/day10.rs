use aoc_harness::aoc_main;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::convert::TryInto;
use std::f64::consts::FRAC_PI_2;
use utils::points::{Point, PolarCoord};

aoc_main!(2019 day 10, generator gen, part1 [p1a] => 288, part2 [p2a] => 616);
type AsteroidSet = HashSet<Point>;

pub fn gen(input: &str) -> AsteroidSet {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point(x.try_into().unwrap(), y.try_into().unwrap())),
                _ => None,
            })
        })
        .collect()
}

pub fn p1a(input: &AsteroidSet) -> usize {
    get_best_station(input).0
}
pub fn get_best_station(input: &AsteroidSet) -> (usize, Point) {
    input
        .iter()
        .map(|&p| {
            (
                input
                    .iter()
                    .filter_map(|&x| {
                        if x == p {
                            None
                        } else {
                            Some((x - p).simplest_direction())
                        }
                    })
                    .collect::<HashSet<_>>()
                    .len(),
                p,
            )
        })
        .max_by_key(|x| x.0)
        .unwrap()
}

pub fn p2a(input: &AsteroidSet) -> isize {
    p2(input, get_best_station(input).1, 200)
}

pub fn p2(input: &AsteroidSet, station: Point, nth: usize) -> isize {
    let mut map: HashMap<Point, BinaryHeap<Reverse<(isize, Point)>>> = input
        .iter()
        .filter(|&&x| x != station)
        .fold(HashMap::new(), |mut hm, &p| {
            let o = p - station;
            hm.entry(o.simplest_direction())
                .or_insert_with(BinaryHeap::new)
                .push(Reverse((o.size_squared(), p)));
            hm
        });
    let mut dir_list: Vec<_> = map.keys().cloned().collect();
    dir_list.sort_by(|&a, &b| {
        let a_pc = PolarCoord::from_point(a).rotate(FRAC_PI_2);
        let b_pc = PolarCoord::from_point(b).rotate(FRAC_PI_2);
        b_pc.theta.partial_cmp(&a_pc.theta).unwrap()
    });
    let mut list_ix = 0;
    let mut order = Vec::new();
    while !dir_list.is_empty() {
        list_ix %= dir_list.len();
        let heap = map.get_mut(&dir_list[list_ix]).unwrap();
        if let Some(Reverse((_, x))) = heap.pop() {
            order.push(x);
            list_ix += 1;
        } else {
            let _ = dir_list.remove(list_ix);
        }
    }
    let x = order[nth - 1];
    x.0 * 100 + x.1
}

#[cfg(test)]
const LARGE: &str = "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
#[test]
pub fn tests() {
    assert_eq!(
        get_best_station(&gen(".#..#\n.....\n#####\n....#\n...##")),
        (8, Point(3, 4))
    );
    assert_eq!(get_best_station(&gen("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n")),(33,Point(5,8)));
    assert_eq!(get_best_station(&gen("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.")),(35,Point(1,2)));
    assert_eq!(get_best_station(&gen(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..\n")),(41, Point(6,3)));
    assert_eq!(get_best_station(&gen(LARGE)), (210, Point(11, 13)));
}

#[test]
pub fn t2() {
    let e = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....#...###..\n..#.#.....#....##";
    assert_eq!(p2(&gen(e), Point(8, 3), 1), 801);
    assert_eq!(p2(&gen(e), Point(8, 3), 36), 1403);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 1), 1112);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 2), 1201);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 3), 1202);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 10), 1208);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 20), 1600);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 50), 1609);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 100), 1016);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 199), 906);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 200), 802);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 201), 1009);
    assert_eq!(p2(&gen(LARGE), Point(11, 13), 299), 1101);
}

/*           1111111
   01234567890123456
 0 .#....###24...#..
 1 ##...##.13#67..9#
 2 ##...#...5.8####.
 3 ..#.....X...###..
 4 ..#.#.....#....##

*/
