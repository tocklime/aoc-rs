use utils::{cartesian::Point, grid2d::Grid2d};

aoc_harness::aoc_main!(2024 day 18, generator gen_, part1 [p1] => 356, part2 [p2_binsearch, p2_pathinfo] => "22,33", example both EG => (22,"6,1"));

struct Memory {
    g: Grid2d<Option<usize>>,
    corruptions: Vec<Point<usize>>,
    is_test: bool,
}

fn gen_(input: &str) -> Memory {
    let is_test = input == EG;
    let s = if is_test { 7 } else { 71 };
    let mut g = Grid2d::from_elem((s, s), None);
    let mut corruptions = Vec::new();
    for (t, l) in input.lines().enumerate() {
        let (x, y) = l.split_once(',').unwrap();
        let p = Point::new(x.parse().unwrap(), y.parse().unwrap());
        g[p] = Some(t);
        corruptions.push(p);
    }
    Memory {
        g,
        corruptions,
        is_test,
    }
}
impl Memory {
    fn try_solve(&self, at_time: usize) -> Option<(Vec<Point<usize>>, usize)> {
        let g = &self.g;
        let target = g.dim() - Point::new(1, 1);
        pathfinding::directed::astar::astar(
            &Point::new(0, 0),
            |&p| {
                g.neighbours(p).filter_map(|x| match g[x] {
                    Some(x) if x < at_time => None,
                    _ => Some((x, 1)),
                })
            },
            |&p| p.manhattan_unsigned(&target),
            |&p| p == target,
        )
    }
}
fn p1(input: &Memory) -> usize {
    let t_limit = if input.is_test { 12 } else { 1024 };
    input.try_solve(t_limit).unwrap().1
}
fn p2_binsearch(input: &Memory) -> String {
    let t_limit = if input.is_test { 12 } else { 1024 };
    let t = utils::nums::bin_search(
        |x| input.try_solve(x).is_none(),
        true,
        t_limit,
        input.corruptions.len(),
    );
    let p = input.corruptions[t];
    format!("{},{}", p.x, p.y)
}
fn p2_pathinfo(input: &Memory) -> String {
    let mut t = if input.is_test { 12 } else { 1024 };
    while let Some(p) = input.try_solve(t) {
        //success at time t. what time is the earliest block on the path?
        t = 1 + p.0.iter().filter_map(|x| input.g[*x]).min().unwrap();
        // println!("trying at time {t}");
    }
    let p = input.corruptions[t - 1];
    format!("{},{}", p.x, p.y)
}

const EG: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
