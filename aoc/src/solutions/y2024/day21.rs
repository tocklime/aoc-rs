use std::collections::HashMap;

use lazy_static::lazy_static;
use utils::{
    cartesian::{Dir, Point}, collections::VecLookup, grid2d::Grid2d
};

aoc_harness::aoc_main!(2024 day 21, 
    part1 [p1::<3>] => 202_648, 
    // part2 [p1::<16>, p1_shorter::<16>],
    // part2 [p1_shorter::<26>],
    part2 [p1::<26>] => 248_919_739_734_728,
    example part1 "029A" => 68 * 29,
    example part1 "379A" => 64 * 379,
    example part1 "456A" => 64 * 456,
    example part1 "980A" => 60 * 980,
    example part1 "179A" => 68 * 179,
    example part1 EG => 126_384,
);

struct ButtonPad {
    grid: Grid2d<char>,
}

type Cache = VecLookup<HashMap<(char,char),usize>>;

#[allow(dead_code)]
fn summarise_cache(cache: &Cache) {
    print!("\r");
    for i in 1..=26 {
        print!("{:02} ", cache.get(i).map(|x| x.len()).unwrap_or_default());
    }
}
impl ButtonPad {
    fn from_str(input: &str) -> Self {
        let grid = Grid2d::from_str_as_char(input);
        let grid = grid.flip_y();
        Self { grid }
    }
    fn lookup(&self, c: char) -> Point<usize> {
        self.grid.find(|x| x == &c).unwrap().0
    }
    
    fn type_string(&self, cache: &mut Cache, str: &str, robot_count: usize) -> usize {
        let mut last_char = 'A';
        let mut ans_cost = 0;
        for c in str.chars() {
            let cost = self.type_char(cache, last_char, c, robot_count);
            ans_cost += cost;
            last_char = c;
        }
        ans_cost
    }
    #[allow(dead_code)]
    fn evaluate(&self, input: &str, with_spaces: bool) -> String {
        let mut pos = self.lookup('A');
        let mut ans = String::new();
        for c in input.chars() {
            let x = match c {
                '<' => (' ', pos.left()),
                '>' => (' ', pos.right()),
                '^' => (' ', pos.up()),
                'v' => (' ', pos.down()),
                'A' => (self.grid[pos], pos),
                ' ' => (' ', pos),
                _ => unreachable!(),
            };
            pos = x.1;
            if with_spaces || x.0 != ' ' {
                ans.push(x.0);
            }
            assert!(self.grid.get(pos).is_some());
            assert!(self.grid[pos] != ' ');
        }
        assert!(self.grid[pos] == 'A');
        ans
    }
    fn type_char(&self, cache: &mut Cache, from: char, c: char, robot_count: usize) -> usize {
        if let Some(a) = cache.get(robot_count).and_then(|m| m.get(&(from,c))) {
            return *a;
        }
        if robot_count == 0 {
            //this is a human. they can just type.
            return 1;
        }
        if from == c {
            //is same character. can just hit 'A' all the way up.
            return 1;
        }
        // println!("Trying to press {c} from {from} at level {robot_count}");
        // need path to get next robot to move me from from to c.
        // next robot starts at A, needs to finish at 'A' and then press. 
        let (_path, cost) = pathfinding::directed::dijkstra::dijkstra(
            &(from, 'A'),
            |(prev_button, prev_dir)| {
                const DIRS: [char; 4] = ['<', 'v', '>', '^'];
                let p = self.lookup(*prev_button);
                let foo: Vec<_> = DIRS
                    .iter()
                    .filter_map(move |d| {
                        let x = self
                            .grid
                            .get(p.step(Dir::from_x("^v<>", *d)))
                            .filter(|p| **p != ' ');
                        x.map(|c| (d, c))
                    })
                    .map(|(dir, new_sym)| {
                        let mut cost = ARROW_PAD.type_char(cache, *prev_dir, *dir, robot_count - 1);
                        if *new_sym == c {
                            //this is the target. add on the keypress cost.
                            let  p_cost = ARROW_PAD.type_char(cache, *dir, 'A',robot_count-1);
                            cost += p_cost;
                        }
                        ((*new_sym, *dir), cost)
                    })
                    .collect();
                foo
            },
            |(ch, _)| *ch == c ,
        )
        .expect("No path found");

        cache.entry(robot_count).or_default().insert((from,c), cost);
        // summarise_cache(cache);
        cost
    }

}

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

#[allow(dead_code)]
fn print_ans(a: &[String], target: &str) {
    let mut a_pos: Vec<usize> = Vec::new();
    for l in a.iter().rev().map(|x| &x[..]).chain([target]) {
        let mut next_a = a_pos.into_iter();
        let mut new_as = Vec::new();
        let mut cs = l.chars();
        let mut pos = 0;
        while let Some(a) = next_a.next() {
            let c = cs.next().unwrap();
            while pos < a {
                print!(" ");
                pos += 1;
            }
            print!("{c}");
            if c == 'A' {
                new_as.push(pos);
            }
            pos += 1;
        }
        //any left?
        while let Some(c) = cs.next() {
            print!("{c}");
            if c == 'A' {
                new_as.push(pos);
            }
            pos += 1;
        }
        a_pos = new_as;
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn breakdown(line: &str, depth: usize) {
    let mut foo = line.to_string();
    println!("{foo}");
    for l in (0..depth).rev() {
        if l > 0 {
            foo = ARROW_PAD.evaluate(&foo, true);
        } else {
            foo = NUM_PAD.evaluate(&foo, true);
        }
        println!("{foo}");
    }
}
fn p1<const ROBOTS: usize>(input: &str) -> usize {
    let mut ans = 0;
    let mut cache = Cache::default();
    for l in input.lines() {
        let x = NUM_PAD.type_string(&mut cache, l, ROBOTS);
        let num: usize = l[0..l.len() - 1].parse().unwrap();
        ans += x * num;
    }
    ans
}

const EG: &str = "029A
980A
179A
456A
379A
";
