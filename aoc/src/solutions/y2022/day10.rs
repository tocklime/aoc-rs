use aoc_harness::*;
use utils::grid2d::Grid2d;

aoc_main!(2022 day 10, part1 [p1], part2 [p2], example part1 EG => 13140);

const EG : &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

fn p1(input: &str) -> i32 {
    // let mut time = 0;
    let mut x = 1;
    // let mut ip = 0;
    let mut x_over_time = vec![1];
    for l in input.lines() {
        match &l[0..4] {
            "noop" => {
                x_over_time.push(x);
            }
            "addx" => {
                let val : i32 = l[5..].parse().unwrap();
                x_over_time.push(x);
        println!("At end of cycle {}, x is {}", x_over_time.len(), x);
                x += val;
                x_over_time.push(x);
            }
            _ => panic!("{}", l)
        }
        println!("At end of cycle {}, x is {}", x_over_time.len(), x);
        assert_eq!(x_over_time[x_over_time.len()-1], x);
    }
    dbg!(x_over_time[220]);
    [20,60,100,140,180,220].map(|ix| dbg!((ix as i32) * dbg!(x_over_time[ix-1]))).into_iter().sum()
}

fn p2(input: &str) -> String {
    let mut x = 1;
    let mut x_over_time = vec![1];
    for l in input.lines() {
        match &l[0..4] {
            "noop" => {
                x_over_time.push(x);
            }
            "addx" => {
                let val : i32 = l[5..].parse().unwrap();
                x_over_time.push(x);
        println!("At end of cycle {}, x is {}", x_over_time.len(), x);
                x += val;
                x_over_time.push(x);
            }
            _ => panic!("{}", l)
        }
        println!("At end of cycle {}, x is {}", x_over_time.len(), x);
        assert_eq!(x_over_time[x_over_time.len()-1], x);
    }

    let mut screen = Grid2d::from_elem((6,40), '.');
    for (t, x) in x_over_time.iter().enumerate() {
        let ti = (t%40) as i32;
        if (x-ti).abs() < 2 {
            println!("# at time {}, x is {}", t, x);
            screen[(t/40, t%40)] = '#';
        } else {
            println!(". at time {}, x is {}", t, x);
        }
    }
    println!("{}", screen);
    screen.to_string()
}