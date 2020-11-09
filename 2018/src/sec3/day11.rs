

use itertools::Itertools;

fn level(sn: usize, x: usize, y: usize) -> isize {
    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + sn;
    let power_level = power_level * rack_id;
    let hundreds = (power_level / 100) % 10;
    (hundreds as isize) - 5
}

fn safe_get(grid : &[Vec<isize>],x : usize,y : usize) -> isize {
    grid.get(y).and_then(|r| r.get(x)).cloned().unwrap_or_default()
}

const SIZE : usize = 300;

#[aoc_generator(day11)]
fn gen(input: &str) -> Vec<Vec<isize>> {
    let n : usize = input.trim().parse().unwrap();
    let grid : Vec<Vec<isize>> = (1..=SIZE).map(|y1|
        (1..=SIZE).map(|x1| level(n,x1,y1)).collect_vec()
    ).collect_vec();
    let mut prefix_sum = vec![vec![0; SIZE]; SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let s= grid[y][x];
            let left = safe_get(&prefix_sum,x-1,y);
            let up = safe_get(&prefix_sum,x,y-1);
            let left_up = safe_get(&prefix_sum,x-1,y-1);
            prefix_sum[y][x] = s + left + up - left_up;
        }
    }
    prefix_sum
}

fn solve(input: &[Vec<isize>], window_size: usize) -> (isize,usize,usize,usize) {
    let pref_ref = &input;
    let foo = (0..SIZE - window_size).flat_map(|y| {
        (0..SIZE-window_size).map(move |x| {
            let tl = safe_get(pref_ref, x-1, y-1);
            let br = safe_get(pref_ref, x-1+window_size, y-1+window_size);
            let tr = safe_get(pref_ref, x-1+window_size, y-1);
            let bl = safe_get(pref_ref, x-1, y-1+window_size);
            let val = br + tl - bl - tr;
            (val,x+1,y+1,window_size)
        })
    });
    foo.max_by_key(|x| x.0).unwrap()
}

#[aoc(day11, part1)]
fn p1(input: &[Vec<isize>]) -> String {
    let ans = solve(input, 3);
    format!("{},{}",ans.1,ans.2)
}
#[aoc(day11, part2)]
fn p2(input: &[Vec<isize>]) -> String {
    let ans = (0..300).map(|s| solve(input,s)).max().unwrap();
    format!("{},{},{}",ans.1,ans.2,ans.3)
}