use std::{
    error::Error,
    fmt::{Display, Write},
    str::FromStr,
};

use aoc_harness::*;

aoc_main!(2022 day 25, generator lines::<Snafu>, part1 [p1] => "2=0--0---11--01=-100", example part1 EG => "2=-1=0");

const EG: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

#[derive(Copy, Clone)]
struct Snafu(i64);

impl Snafu {
    fn char_value(c: char) -> Result<i64, NotASnafuCharError> {
        match c {
            '2' => Ok(2),
            '1' => Ok(1),
            '0' => Ok(0),
            '-' => Ok(-1),
            '=' => Ok(-2),
            c => Err(NotASnafuCharError(c)),
        }
    }
}

#[derive(Debug)]
struct NotASnafuCharError(char);
impl Error for NotASnafuCharError {}
impl Display for NotASnafuCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Not a snafu digit: '{}'", self.0))
    }
}
impl FromStr for Snafu {
    type Err = NotASnafuCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .fold(Ok(0), |acc, c| {
                acc.and_then(|a| Ok(a * 5 + Self::char_value(c)?))
            })
            .map(Snafu)
    }
}
impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n = self.0;
        let mut ans: Vec<char> = vec![];
        while n > 0 {
            let (value_used, c) = match n % 5 {
                0 => (0, '0'),
                1 => (1, '1'),
                2 => (2, '2'),
                3 => (-2, '='),
                4 => (-1, '-'),
                _ => panic!(),
            };
            ans.push(c);
            n = (n - value_used) / 5;
        }
        for c in ans.into_iter().rev() {
            f.write_char(c)?;
        }
        Ok(())
    }
}
impl<'a> std::iter::Sum<&'a Snafu> for Snafu {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Snafu(iter.map(|x| x.0).sum())
    }
}

fn p1(input: &[Snafu]) -> String {
    format!("{}", input.iter().sum::<Snafu>())
}
