use aoc2019::utils::points::as_point_map;
use aoc2019::utils::prelude::*;

//#[aoc(day17, part1)]
pub fn p1(input: &str) -> isize {
    let mut c: Computer = input.parse().unwrap();
    let output = c.run().output_as_string();
    let g = as_point_map(&output);
    g.iter()
        .filter_map(|(p, c)| {
            if c == &'#' {
                let nc = p.neighbours().iter().filter(|pn| g.get(pn) == Some(&'#')).count();
                if nc == 4 {
                    Some(p.0 * p.1)
                } else { None }
            } else { None }
        })
        .sum()
}

//#[aoc(day17, part2)]
pub fn p2(input: &str) -> i32 {
    let mut c: Computer = input.parse().unwrap();
    c.abs_store(0, 2);
    let icode: &str = "
A,B,A,C,A,B,C,A,B,C
R,8,R,10,R,10
R,4,R,8,R,10,R,12
R,12,R,4,L,12,L,12
n
";
    c.with_string_input(icode.trim_start())
        .run()
        .get_last_output()
}
