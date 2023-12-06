
use utils::{grid2d::Grid2d, ocr::OcrString};

aoc_harness::aoc_main!(2022 day 10, part1 [p1] => 14360, part2 [p2] => "BGKAEREZ", example both EG => (13140, "????????"), both [both]);

const EG: &str = "addx 15
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

fn handle_input<F>(input: &str, f: &mut F)
where
    F: FnMut(i32),
{
    let mut x = 1;
    for l in input.lines() {
        match &l[0..4] {
            "noop" => {
                f(x);
            }
            "addx" => {
                f(x);
                x += l[5..].parse::<i32>().unwrap();
                f(x);
            }
            _ => panic!("{}", l),
        }
    }
}

fn p1(input: &str) -> i32 {
    let mut time = 0;
    let mut ans: i32 = 0;
    let mut record = |val: i32| {
        time += 1;
        if ((time) % 40) == 20 {
            ans += (time) * val;
        }
    };
    record(1);
    handle_input(input, &mut record);
    ans
}

fn p2(input: &str) -> OcrString {
    let mut screen = Grid2d::from_elem((6, 40), ' ');
    let mut time = 0;
    let mut record = |val: i32| {
        if (time % 40 - val).abs() < 2 {
            screen[time as usize] = '#';
        }
        time += 1;
    };
    record(1);
    handle_input(input, &mut record);
    OcrString::from(screen.to_string())
}

fn both(input: &str) -> (i32, OcrString) {
    let mut screen = Grid2d::from_elem((6, 40), ' ');
    let mut time = 0;
    let mut ans: i32 = 0;
    let mut record = |val: i32| {
        if (time % 40 - val).abs() < 2 {
            screen[time as usize] = '#';
        }
        time += 1;
        if ((time) % 40) == 20 {
            ans += (time) * val;
        }
    };
    record(1);
    handle_input(input, &mut record);
    (ans, OcrString::from(screen.to_string()))
}
