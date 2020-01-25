use pathfinding::directed::dijkstra::{dijkstra, dijkstra_all};
use crypto::md5::Md5;
use crate::utils::cartesian::Point;
use crypto::digest::Digest;


type State = (Point<i32>, String);

fn neighbours(st: &State) -> Vec<(State, usize)> {
    if st.0 == Point::new(3,3) {
        return Vec::new();
    }
    let mut md5 = Md5::new();
    md5.input_str(&st.1);
    let digest = &md5.result_str()[0..4];
    digest.chars()//.map(|x| "bcdef".contains(x))
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

#[aoc(day17, part1)]
fn p1(input: &str) -> String {
    let input = input.trim();
    let start = Point::new(0, 0);
    let (path,_len) = dijkstra(
        &(start, input.to_string()),
        neighbours,
        |(a,_)| *a == Point::new(3,3)).expect("No path");
    path.last().unwrap().1[input.len()..].to_owned()
}
#[aoc(day17, part2)]
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