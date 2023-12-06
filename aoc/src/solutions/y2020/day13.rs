

aoc_harness::aoc_main!(2020 day 13, generator gen, part1 [p1], part2 [p2]);
use utils::nums::chinese_remainder_theorem;
pub struct Bus {
    id: i64,
    index: i64,
}

impl Bus {
    #[allow(clippy::missing_const_for_fn)] //rem_euclid is not stable as a const fn yet.
    fn departure_after(&self, t: i64) -> i64 {
        (-t).rem_euclid(self.id)
    }
    fn target_remainder(&self) -> i64 {
        self.departure_after(self.index)
    }
    fn parse(input: (&str, i64)) -> Option<Self> {
        input.0.parse().ok().map(|id| Self { id, index: input.1 })
    }
}

fn gen(input: &str) -> (i64, Vec<Bus>) {
    let mut l = input.lines();
    let t = l.next().unwrap().parse().unwrap();
    let ns = l.next().unwrap().split(',').zip(0..).filter_map(Bus::parse).collect();
    (t, ns)
}

fn p1(input: &(i64, Vec<Bus>)) -> i64 {
    let b = input.1.iter().map(|b| (b.departure_after(input.0), b.id)).min().unwrap();
    b.0 * b.1
}

fn p2(input: &(i64, Vec<Bus>)) -> i64 {
    chinese_remainder_theorem(&input.1.iter().map(|b| (b.target_remainder(), b.id)).collect::<Vec<_>>())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &str) -> i64 {
        let p = format!("0\n{input}");
        p2(&gen(&p))
    }
    #[test]

    fn examples() {
        assert_eq!(test("17,x,13,19"), 3417);
        assert_eq!(test("67,7,59,61"), 754_018);
        assert_eq!(test("67,x,7,59,61"), 779_210);
        assert_eq!(test("67,7,x,59,61"), 1_261_476);
        assert_eq!(test("1789,37,47,1889"), 1_202_161_486);
    }
}
