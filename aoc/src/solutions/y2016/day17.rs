use aoc_harness::aoc_main;

aoc_main!(2016 day 17, part1 [p1], part2 [p2]);

use pathfinding::directed::dijkstra::{dijkstra, dijkstra_all};
use md5;
use utils::cartesian::Point;


type State = (Point<i32>, String);

fn neighbours(st: &State) -> Vec<(State, usize)> {
    if st.0 == Point::new(3,3) {
        return Vec::new();
    }
    let digest = format!("{:?}", md5::compute(&st.1));
    digest.chars()
        .zip("UDLR".chars())
        .filter_map(|(dig_char, dir)| {
            let new_p = st.0.follow_x("DULR", dir);
            if "bcdef".contains(dig_char)
                && new_p.x >= 0 && new_p.x < 4
                && new_p.y >= 0 && new_p.y < 4
            {
                Some(((new_p, format!("{}{}", st.1, dir)),1))
            } else { None }
        }).collect()
}


fn p1(input: &str) -> String {
    let input = input.trim();
    let start = Point::new(0, 0);
    let (path,_len) = dijkstra(
        &(start, input.to_string()),
        neighbours,
        |(a,_)| *a == Point::new(3,3)).expect("No path");
    path.last().unwrap().1[input.len()..].to_owned()
}

fn p2(input: &str) -> usize {
    let input = input.trim();
    let start = Point::new(0, 0);
    let m = dijkstra_all(
        &(start, input.to_string()),
        neighbours);
    let longest = m.iter()
        .filter(|(k,_)| k.0 == Point::new(3,3))
        .max_by_key(|x| (x.1).1).unwrap();
    (longest.1).1
}

#[test]
fn test17() {
    assert_eq!(p1("ihgpwlah"), "DDRRRD");
    assert_eq!(p1("kglvqrro"), "DDUDRLRRUDRD");
    assert_eq!(p1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    assert_eq!(p2("ihgpwlah"), 370);
    assert_eq!(p2("kglvqrro"), 492);
    assert_eq!(p2("ulqzkmiv"), 830);
}