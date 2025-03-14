use itertools::Itertools;

aoc_harness::aoc_main!(2024 day 25, part1 [p1] => 2691, example part1 EG => 3);

#[derive(Debug)]
struct Key([u8;5]);
#[derive(Debug)]
struct Lock([u8;5]);

#[derive(Debug)]
enum KeyLock {
    Key(Key),
    Lock(Lock),
}

fn parse(input: &str) -> KeyLock {
    let is_lock = input.starts_with('#');
    let mut heights = [0;5];
    for line in input.lines() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                heights[j] += 1;
            }
        }
    }
    if is_lock {
        KeyLock::Lock(Lock(heights))
    } else {
        KeyLock::Key(Key(heights))
    }
}

fn fits(key: &Key, lock: &Lock) -> bool {
    key.0.iter().zip(lock.0.iter()).all(|(k,l)| k + l <= 7)
}

fn p1(input: &str) -> usize {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for item in input.split("\n\n") {
        let p = parse(item);
        match p {
            KeyLock::Key(key) => keys.push(key),
            KeyLock::Lock(lock) => locks.push(lock)
        }
    }
    keys.iter().cartesian_product(&locks).filter(|(k,l)| fits(k,l)).count()
}

const EG: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";