aoc_harness::aoc_main!(2018 day 11, generator gen, part1 [p1], part2 [p2]);
use utils::grid2d::Grid2d;

fn level(sn: usize, x: usize, y: usize) -> isize {
    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + sn;
    let power_level = power_level * rack_id;
    let hundreds = (power_level / 100) % 10;
    (hundreds as isize) - 5
}

const SIZE: usize = 300;

fn gen(input: &str) -> Grid2d<isize> {
    let n: usize = input.trim().parse().unwrap();
    let g = Grid2d::from_fn((SIZE, SIZE), |(x, y)| level(n, x + 1, y + 1));
    let mut prefix_grid = Grid2d::from_elem((SIZE, SIZE), 0);
    for y in 0..(SIZE as isize) {
        for x in 0..(SIZE as isize) {
            let s = g.get_i((x, y)).unwrap();
            let left = prefix_grid.get_i((x - 1, y)).copied().unwrap_or_default();
            let up = prefix_grid.get_i((x, y - 1)).copied().unwrap_or_default();
            let left_up = prefix_grid
                .get_i((x - 1, y - 1))
                .copied()
                .unwrap_or_default();
            prefix_grid[(x as usize, y as usize)] = s + left + up - left_up;
        }
    }
    prefix_grid
}
fn window_sum_at(grid: &Grid2d<isize>, window_size: isize, x: isize, y: isize) -> isize {
    let tl = grid.get_i((x-1, y-1)).copied().unwrap_or_default();
    let br = grid
        .get_i((x-1 + window_size, y-1 + window_size))
        .copied()
        .unwrap_or_default();
    let tr = grid
        .get_i((x-1 + window_size, y-1))
        .copied()
        .unwrap_or_default();
    let bl = grid
        .get_i((x-1, y-1 + window_size))
        .copied()
        .unwrap_or_default();
    br + tl - bl - tr
}
fn solve(input: &Grid2d<isize>, window_size: isize) -> (isize, isize, isize, isize) {
    (0..(SIZE as isize) - window_size)
        .flat_map(|y| {
            (0..(SIZE as isize) - window_size).map(move |x| {
                let val = window_sum_at(input, window_size, x, y);
                (val, x + 1, y + 1, window_size)
            })
        })
        .max_by_key(|x| x.0)
        .unwrap()
}

fn p1(input: &Grid2d<isize>) -> String {
    let ans = solve(input, 3);
    format!("{},{}", ans.1, ans.2)
}

fn p2(input: &Grid2d<isize>) -> String {
    let ans = (0..300).map(|s| solve(input, s)).max().unwrap();
    format!("{},{},{}", ans.1, ans.2, ans.3)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_examples() {
        assert_eq!(level(8, 3, 5), 4);
        assert_eq!(level(57, 122, 79), -5);
        assert_eq!(level(39, 217, 196), 0);
        assert_eq!(level(71, 101, 153), 4);
        assert_eq!(level(18, 33, 45), 4);
        let g = gen("18");
        assert_eq!(window_sum_at(&g, 3, 32, 44), 29);
        assert_eq!(p1(&gen("18")), "33,45");
    }
}
