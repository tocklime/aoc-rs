use itertools::Itertools;
use lazy_static::lazy_static;
use utils::{
    cartesian::{Dir, Point},
    grid2d::Grid2d,
};

aoc_harness::aoc_main!(2024 day 21, part1 [p1],
    example part1 "456A" => 64 * 456,
    example part1 "379A" => 64 * 379,
    example part1 "029A" => 68 * 29,
    example part1 "980A" => 60 * 980,
    example part1 "179A" => 68 * 179,
    example part1 EG => 126_384,
);

#[derive(Clone)]
struct ButtonPad {
    grid: Grid2d<char>,
    position: Point<usize>,
}
const DIR_PRIORITY : [Dir;4] = [Dir::Down, Dir::Left, Dir::Right, Dir::Up];
impl ButtonPad {
    fn from_str(input: &str) -> Self {
        let grid = Grid2d::from_str_as_char(input);
        let grid = grid.flip_y();
        let position = grid.find(|&x| x == 'A').unwrap().0;
        Self { grid, position }
    }
    fn press(&mut self, c: char) -> String {
        //need to press c, we're at `self.position`.
        let c_pos = self.grid.find(|&x| x == c).unwrap().0;
        let (path, _cost) = pathfinding::directed::astar::astar_bag(
            &self.position,
            |&p| {
                DIR_PRIORITY.iter()
                .map(move |d| p.step(*d))
                .filter(|&p| self.grid.get(p).is_some())
                .filter(|&x| self.grid[x] != ' ')
                .map(|x| (x, 1))
            },
            |x| x.manhattan_unsigned(&c_pos),
            |x| *x == c_pos,
        ).expect("no path");

        let mut all_ans = Vec::new();
        for s in path {
            let mut ans: String = s
                .into_iter()
                .tuple_windows()
                .map(|(a, b)| {
                    let d = b.as_i().unwrap() - a.as_i().unwrap();
                    Dir::try_from(d).unwrap().to_x(['^', 'v', '<', '>'])
                })
                .collect();
            self.position = c_pos;
            ans.push('A');
            all_ans.push((ARROW_PAD.estimate_cost(&ans), ans));
        }
        all_ans.sort();
        println!("Paths from {:?} to {c_pos:?} is with cost {_cost}:", self.position);
        for (cost, a) in &all_ans {
            println!("  {a} - {cost}");
        }

        all_ans.into_iter().min().unwrap().1
    }
    fn estimate_cost(&self, input: &str) -> usize {
        // input.chars().fold(('A',0), |(a,cost),b|{
        //     let pos_a = self.grid.find(|x| *x == a).unwrap().0;
        //     let pos_b = self.grid.find(|x| *x == b).unwrap().0;
        //     let cost = cost + pos_a.manhattan_unsigned(&pos_b);
        //     (b, cost)
        // }).1
        input.chars().tuple_windows().map(|(a,b)| usize::from(a != b)).sum()
    }
    fn press_sequence(&mut self, s: &str) -> String {
        // println!("\n\nTyping out {s}");
        s.chars().map(|x| self.press(x)).collect()
    }
}
// 212128 is too high

lazy_static! {
    ///+---+---+---+
    ///| 7 | 8 | 9 |
    ///+---+---+---+
    ///| 4 | 5 | 6 |
    ///+---+---+---+
    ///| 1 | 2 | 3 |
    ///+---+---+---+
    ///    | 0 | A |
    ///    +---+---+
    static ref NUM_PAD : ButtonPad = ButtonPad::from_str("789\n456\n123\n 0A");

    ///     +---+---+
    ///     | ^ | A |
    /// +---+---+---+
    /// | < | v | > |
    /// +---+---+---+
    static ref ARROW_PAD : ButtonPad = ButtonPad::from_str(" ^A\n<v>");
}

fn p1(input: &str) -> usize {
    let mut machines = [NUM_PAD.clone(), ARROW_PAD.clone(), ARROW_PAD.clone()];
    let mut ans = 0;
    for l in input.lines() {
        let seq = machines
            .iter_mut()
            .fold(l.to_owned(), |acc, m| m.press_sequence(&acc));
        let num: usize = l[0..l.len() - 1].parse().unwrap();
        // println!("ans is {seq} * {num}");
        ans += seq.len() * num;
    }
    ans
}

//v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>A<vA>^AA<A>A<vA<A>>^AAA<A>vA^A
//v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A^>AA<A>Av<A<A>>^AAA<A>vA^A

// Correct for 379A:
// <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//    <   A > A  v <<   AA >  ^ AA > A  v  AA ^ A   < v  AAA >  ^ A
// <A>Av<<AA>^AA>AvAA^A<vAAA>^A
//  ^ A   <<  ^^ A >> A  vvv  A
// ^A<<^^A>>AvvvA
//  3    7  9   A
// 379A

// Me for 379A:
// v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A^>AA<A>Av<A<A>>^AAA<A>vA^A
//    <   A > A   <   AA  v <   AA >>  ^ A  v  AA ^ A  v <   AAA ^  > A
// <A>A<AAv<AA>>^AvAA^Av<AAA^>A
//  ^ A ^^  <<   A >> A  vvv  A
// ^A^^<<A>>AvvvA
// 379A

// v<<A>>^AvA^Av<<A>>^AA<vA<A>>^AAvAA<^A>A<vA^>AA<A>Av<<A>A>^AAA<A>vA^A
// <A>A<AAv<AA>>^AvAA^A<vAAA^>A
// ^A^^<<A>>AvvvA
// 379A


// Correct for 029A
// <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
// v<<A>>^A<A>AvA<^AA>A<vAAA>^A
// <A^A>^^AvvvA
// 029A

// Me for 029A
// v<A<AA>^>AvA<^Av>A^Av<<A>^>AvA^Av<<A>^>AAv<A>A^A<A>Av<A<A>^>AAA<Av>A^A * 29   
// v<<A>^>A<A>A<AAv>A^Av<AAA^>A                                                                                                          ▐
// <A^A^^>AvvvA                                                                                                                          ▐
// 029A                                                                                                                                  ▐

const EG: &str = "029A
980A
179A
456A
379A
";


#[cfg(test)]
mod test {
    const EG1: &str = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
v<<A>>^A<A>AvA<^AA>A<vAAA>^A
<A^A>^^AvvvA
029A";
    #[test]
    fn example() {
        let lines: Vec<&str> = EG1.lines().collect();
        let mut my_num = super::NUM_PAD.clone();
        let a: String = my_num.press_sequence(lines[3]);
        assert_eq!(a.len(), lines[2].len());
        println!("****");
        let mut my_arr1 = super::ARROW_PAD.clone();
        assert_eq!(my_arr1.press_sequence(lines[2]).len(), lines[1].len());
        println!("****");
        let mut my_arr2 = super::ARROW_PAD.clone();
        assert_eq!(my_arr2.press_sequence(lines[1]).len(), lines[0].len());
    }
}
