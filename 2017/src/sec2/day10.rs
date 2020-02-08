use itertools::Itertools;

#[derive(Debug)]
struct State {
    pos: usize,
    skip_size: usize,
    data: Vec<usize>
}
impl State {
    fn new(size: usize) -> Self {
        State {
            pos: 0,
            skip_size: 0,
            data: (0..size).collect()
        }
    }
    fn step(&mut self, input: &[usize]) {
        let size = self.data.len();
        for &n in input {
            for i in 0..n / 2 {
                self.data.swap((self.pos + i) % size, (self.pos + n - i - 1) % size);
            }
            self.pos = (self.pos + n + self.skip_size) % size;
            self.skip_size += 1;
        }
    }
    fn dense_hash(&self) -> String {
        self.data.chunks(16).map(|ch| {
            let n = ch.iter().fold(0_usize,|a,&b| a ^ b);
            format!("{:x}",n)
        }).collect()
    }
}

#[aoc(day10,part1)]
fn p1(input:&str) -> usize {
    let ns = input.split(',').map(|n| n.parse::<usize>().unwrap()).collect_vec();
    let mut st = State::new(256);
    st.step(&ns);
    st.data[0] * st.data[1]
}


#[aoc(day10,part2)]
fn p2(input: &str) -> String {
    let suffix: Vec<u8> = vec![17,31,73,47,23];
    let ns : Vec<usize> = input.bytes().chain(suffix).map(|x| x.into()).collect();
    let mut st = State::new(256);
    for _ in 0..64 {
        st.step(&ns);
    }
    st.dense_hash()
}

