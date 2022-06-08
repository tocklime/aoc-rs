use std::collections::VecDeque;

fn solve(input: &str, last_marble_scale: usize) -> usize {
    let player_count: usize = input.split(' ').next().unwrap().parse().unwrap();
    let last_marble: usize = input.split(' ').nth(6).unwrap().parse().unwrap();
    let last_marble = last_marble * last_marble_scale;
    let mut scores = vec![0; player_count];
    let mut list = VecDeque::new();
    dbg!(player_count, last_marble);
    list.push_back(0);
    for turn in 0..last_marble {
        let marble = turn + 1;
        let elf = turn % player_count;
        if marble % 23 == 0 {
            list.rotate_right(7);
            *scores.get_mut(elf).unwrap() += marble + list.pop_back().unwrap();
            list.rotate_left(1);
        } else {
            list.rotate_left(1);
            list.push_back(marble);
        }
    }
    *scores.iter().max().unwrap()
}

fn p1(input: &str) -> usize {
    solve(input, 1)
}


fn p2(input: &str) -> usize {
    solve(input, 100)
}
