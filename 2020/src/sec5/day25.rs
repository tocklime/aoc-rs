use crate::utils::nums::mod_pow;

#[aoc(day25,part1)]
pub fn p1(input: &str) -> usize {
    let keys = input.lines().map(str::parse).collect::<Result<Vec<usize>,_>>().unwrap();
    let mut loop_sizes = Vec::new();
    for &k in &keys {
        for loop_size in 0.. {
            let mut subject_number = 7;
            subject_number = mod_pow(subject_number, loop_size, 20201227);
            if subject_number == k {
                println!("Found loop_size for {}: {}",k,loop_size);
                loop_sizes.push(loop_size);
                break;
            }
        }
    }
    mod_pow(keys[0],loop_sizes[1],20201227)
}