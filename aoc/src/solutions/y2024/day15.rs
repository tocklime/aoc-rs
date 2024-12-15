use itertools::Itertools;
use utils::{cartesian::{Dir, Point}, grid2d::Grid2d};

aoc_harness::aoc_main!(2024 day 15, part1 [p1] => 1_495_147, part2 [p2] => 1_524_905, example both EG => (10_092,9_021));

fn p1(input: &str) -> usize {
    let (input, instr) = input.split_once("\n\n").unwrap();
    let mut g = Grid2d::from_str_as_char(input);
    let mut pos = g.find(|x| x == &'@').unwrap().0;
    g[pos] = '.';

    for i in instr.chars() {
        if i == '\n' {
            continue;
        }
        let dir : Point<isize> = Dir::from_x("v^<>", i).as_point_step();
        let target = pos+dir;
        //we're at pos, trying to move dir. can we find a gap before we find a wall?
        // println!("{g}\n\n");
        let gap = g.values_in_direction(target, dir).find_map(|(p,c)| match c {
            '.' => Some(Some(p)),
            '#' => Some(None),
            'O' => None,
            _ => unreachable!()
        }).unwrap();
        match gap {
            None => (), //can't move, do nothing.
            Some(p) => {
                //we're at pos, trying to move to pos+dir. the first open spot is at p, and everything between here and there is 'O'.
                g[p] = 'O';
                g[target] = '.';
                pos = target;
            }
        }
    }
    // println!("{g}");
    g.indexed_iter().filter_map(|(p,c)| {
        (c == &'O').then_some(p.y*100+p.x)
    }).sum()
}

/// Can a push here move? if it did, what is the list of cells that would move?
fn find_movers(g: &Grid2d<char>, pos: Point<usize>, dir: Point<isize>) -> Option<Vec<Point<usize>>> {
    let next = pos + dir;
    // println!("Trying to move in dir {dir:?} from {pos:?}. Next step is {next:?}. Char here is {}", g[pos]);
    match (g[pos], dir.x == 0) {
        ('.',_) => Some(vec![]),
        ('#',_) => None,
        ('[',true) => {
            let mut left = find_movers(g, next, dir)?;
            let right = find_movers(g, next.right(), dir)?;
            left.extend(right);
            left.push(pos);
            left.push(pos.right());
            // println!("  movers from here are {left:?}");
            Some(left)
        }
        (']',true) => {
            let mut left = find_movers(g, next.left(), dir)?;
            let right = find_movers(g, next, dir)?;
            left.extend(right);
            left.push(pos);
            left.push(pos.left());
            // println!("  movers from here are {left:?}");
            Some(left)
        }
        ('[' | ']', false) => {
            let mut sub = find_movers(g, next, dir)?;
            sub.push(pos);
            Some(sub)
        }
        _ => unreachable!()
    }
}
fn p2(input: &str) -> usize {
    let (input, instr) = input.split_once("\n\n").unwrap();
    let g = Grid2d::from_str_as_char(input);
    let dim = g.dim();
    let mut g = Grid2d::from_fn((dim.y,dim.x*2), |p| {
        let base = g[(p.y,p.x/2)];
        let is_right = p.x%2 == 1;
        match (base,is_right) {
            ('#',_) => '#',
            ('O',false) => '[',
            ('O',true) => ']',
            ('@',false) => '@',
            _ => '.'
        }
    });
    let mut pos = g.find(|x| x == &'@').unwrap().0;
    g[pos] = '.';

    for i in instr.chars() {
        if i == '\n' {
            continue;
        }
        let dir : Point<isize> = Dir::from_x("v^<>", i).as_point_step();
        let target = pos+dir;
        //we're at pos, trying to move dir. can we find a gap before we find a wall?
        if let Some(v) = find_movers(&g, target, dir) {
            // println!("From {pos:?} To move in {dir:?}: {v:?}");
            // println!("{g}\n\n");
            for p in v.into_iter().unique() {
                assert_eq!(g[p+dir], '.');
                g[p+dir] = g[p];
                g[p] = '.';
            }
            pos = target;
        }
    }
    // println!("{g}");
    g.indexed_iter().filter_map(|(p,c)| {
        (c == &'[').then_some(p.y*100+p.x)
    }).sum()
}


const EG: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";