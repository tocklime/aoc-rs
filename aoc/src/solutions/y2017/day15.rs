use itertools::Itertools;

#[derive(Copy, Clone)]
struct Gen {
    curr: u64,
    factor: u64,
    require_mod: u64,
}

impl Gen {
    fn new(curr: u64, factor: u64,require_mod: u64) -> Self {
        Self { curr, factor, require_mod }
    }
    fn step(&mut self) -> u64 {
        loop {
            self.curr = (self.curr * self.factor) % 2147483647; //2^31
            if self.curr % self.require_mod == 0 {
                break;
            }
        }
        self.curr
    }
}



fn p1(input: &str) -> usize {
    /*Generator A starts with 618\nGenerator B starts with 814*/
    let (a, b) = input.lines().map(|x| x[24..].parse::<u64>().unwrap()).next_tuple().unwrap();
    let mut a = Gen::new(a, 16807,1);
    let mut b = Gen::new(b, 48271,1);
    let mut matches = 0;
    for _ in 0..40_000_000 {
        let x = a.step();
        let y = b.step();
        if (x as u16) == (y as u16) {
            matches += 1;
        }
    }
    matches
}

fn p2(input: &str) -> usize {
    /*Generator A starts with 618\nGenerator B starts with 814*/
    let (a, b) = input.lines().map(|x| x[24..].parse::<u64>().unwrap()).next_tuple().unwrap();
    let mut a = Gen::new(a, 16807,4);
    let mut b = Gen::new(b, 48271,8);
    let mut matches = 0;
    for _ in 0..5_000_000 {
        let x = a.step();
        let y = b.step();
        if (x as u16) == (y as u16) {
            matches += 1;
        }
    }
    matches
}
