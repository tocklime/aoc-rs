use itertools::Itertools;
use num::{CheckedSub, Num};

#[derive(Debug)]
pub struct PolynomialDetector<T> {
    ns: Vec<Vec<T>>,
    capacity_hint: usize,
}
#[derive(Debug)]
pub struct PolynomialCertaintyAndPower {
    pub certainty: usize,
    pub power: usize,
}
#[derive(Debug)]
pub struct PolynomialResult<T> {
    a0_to_n: Vec<T>,
    certainty: usize,
}
impl<T> Default for PolynomialDetector<T> {
    fn default() -> Self {
        Self::with_capacity(8)
    }
}
impl<T: Num + Copy + std::fmt::Debug> PolynomialResult<T> {
    pub fn evaluate(&self, x: T) -> T {
        //newton form:
        //y = a0 + a1(x-1) + a2(x-1)(x-2) + a3(x-1)(x-2)(x-3)...
        self.a0_to_n
            .iter()
            .fold(
                (T::one(), T::zero(), T::one()),
                |(power_now, total, x_product), &a_n| {
                    assert_eq!((x_product * (x - power_now)) % power_now, T::zero());
                    let next_x_powers = x_product * (x - power_now) / power_now;
                    (power_now + T::one(), total + a_n * x_product, next_x_powers)
                },
            )
            .1
    }
    pub fn certainty(&self) -> usize {
        self.certainty
    }
}

impl<T> PolynomialDetector<T> {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            ns: Vec::with_capacity(n),
            capacity_hint: n,
        }
    }
}
impl<T: Num + Copy + std::fmt::Debug + CheckedSub> PolynomialDetector<T> {
    pub fn add(&mut self, n: T) {
        self.ns.push(Vec::with_capacity(self.capacity_hint));
        self.ns.last_mut().unwrap().push(n);
        let mut last = n;
        for r in (0..self.ns.len() - 1).rev() {
            let Some(diff) = last.checked_sub(self.ns[r].last().unwrap()) else {
                dbg!(self, last, r);
                panic!();
            };
            self.ns[r].push(diff);
            last = diff;
        }
    }
    pub fn get_certainty_and_power(&self) -> PolynomialCertaintyAndPower {
        let power = (0..self.ns.len())
            .find(|&p| {
                self.ns
                    .iter()
                    .take(self.ns.len() - p)
                    .map(|r| r[p])
                    .all_equal()
            })
            .unwrap();
        let certainty = self.ns.len() - power;
        PolynomialCertaintyAndPower { certainty, power }
    }
    pub fn get_equation(&self) -> PolynomialResult<T> {
        let cp = self.get_certainty_and_power();
        PolynomialResult {
            a0_to_n: self.ns[0][0..cp.power + 1].to_vec(),
            certainty: cp.certainty,
        }
    }
}

#[cfg(test)]
mod test {
    use super::PolynomialDetector;

    #[test]
    fn test_poly_detector() {
        let ns = [10i64, 13, 16, 21, 30, 45];
        let mut x = PolynomialDetector::default();
        for n in &ns {
            x.add(*n);
        }
        let r = x.get_equation();
        assert_eq!(r.certainty, 3);
        dbg!(x, &r);
        for (ix, x) in ns.iter().enumerate() {
            assert_eq!(r.evaluate(1 + ix as i64), *x, "Calculating {ix}th term.");
        }
        assert_eq!(r.evaluate(100), 314_005);
    }
    #[test]
    fn eg_from_2023d21() {
        let ns = [3943, 97407, 315263, 657511, 1124151, 1715183];
        let mut x = PolynomialDetector::default();
        for n in &ns {
            x.add(*n);
        }
        let cp = x.get_certainty_and_power();
        assert_eq!(cp.power, 2);
        assert_eq!(cp.certainty, 4);
        let r = x.get_equation();
        dbg!(&x, &r);
        for (ix, x) in ns.iter().enumerate() {
            assert_eq!(r.evaluate(1 + ix as i64), *x, "Calculating {ix}th term.");
        }
        assert_eq!(r.evaluate(101151), 636_350_496_972_143i64);
    }
}
