use std::{fmt::Display, ops::Add, str::FromStr};

use aoc_harness::*;

aoc_main!(2021 day 18, generator lines::<SnailNumber>, part1 [p1], part2 [p2],  example part1 EG => 4140);

const EG: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

#[derive(Debug, Clone)]
struct SnailNumber {
    body: Body,
}
#[derive(Debug, Clone)]
enum Body {
    Leaf(usize),
    Tree(Children),
}
#[derive(Debug, Clone)]
struct Children {
    left: Box<SnailNumber>,
    right: Box<SnailNumber>,
}
fn parse_snail(s: &str) -> (SnailNumber, &str) {
    match s.chars().next().unwrap() {
        '[' => {
            let (left, s) = parse_snail(&s[1..]);
            assert_eq!(s.as_bytes().get(0), Some(&b','));
            let (right, s) = parse_snail(&s[1..]);
            assert_eq!(s.as_bytes().get(0), Some(&b']'));

            let ans = SnailNumber {
                body: Body::Tree(Children {
                    left: Box::new(left),
                    right: Box::new(right),
                }),
            };
            // match &mut ans.body {
            //     Body::Leaf(_) => unreachable!(),
            //     Body::Tree(c) => unsafe {
            //         std::ptr::write(c.left.parent, ans.as_ptr());
            //         std::ptr::write(c.right.parent, ans);
            //     },
            // }

            (ans, &s[1..])
        }
        c => (
            SnailNumber {
                body: Body::Leaf(usize::try_from(c.to_digit(16).unwrap()).unwrap()),
            },
            &s[1..],
        ),
    }
}

impl FromStr for SnailNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = parse_snail(s);
        assert_eq!(b, "");
        Ok(a)
    }
}
impl Add for SnailNumber {
    type Output = SnailNumber;
    fn add(self, rhs: Self) -> Self::Output {
        let mut a = SnailNumber {
            body: Body::Tree(Children {
                left: Box::new(self),
                right: Box::new(rhs),
            }),
        };
        a.reduce();
        a
    }
}
impl Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.body {
            Body::Leaf(n) => f.write_fmt(format_args!("{}", n))?,
            Body::Tree(c) => f.write_fmt(format_args!("[{},{}]", c.left, c.right))?,
        }
        Ok(())
    }
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Dir {
    Left,
    Right,
}
impl SnailNumber {
    fn find_leftmost_at_depth(&self, n: usize) -> Option<SnailNumberView> {
        match &self.body {
            Body::Leaf(_) => None,
            Body::Tree(c) => {
                let view = SnailNumberView {
                    current_node: self,
                    i_am_of_parent: None,
                    parent: None,
                };
                if n == 0 {
                    return Some(view);
                }
                match c.left.find_leftmost_at_depth(n - 1) {
                    Some(mut p) => {
                        p.i_am_of_parent = Some(Dir::Left);
                        p.parent = Some(Box::new(view));
                        Some(p)
                    }
                    None => match c.right.find_leftmost_at_depth(n - 1) {
                        Some(mut p) => {
                            p.i_am_of_parent = Some(Dir::Right);
                            p.parent = Some(Box::new(view));
                            Some(p)
                        }
                        None => None,
                    },
                }
            }
        }
    }
    fn try_split(&mut self) -> bool {
        match &mut self.body {
            Body::Leaf(x) if *x >= 10 => {
                self.body = Body::Tree(Children {
                    left: Box::new(SnailNumber {
                        body: Body::Leaf(*x / 2),
                    }),
                    right: Box::new(SnailNumber {
                        body: Body::Leaf(*x / 2 + *x % 2),
                    }),
                });
                true
            }
            Body::Leaf(_) => false,
            Body::Tree(c) => c.left.try_split() || c.right.try_split(),
        }
    }
    fn try_explode(&mut self, depth: usize) -> Option<(bool, Option<usize>, Option<usize>)> {
        match &mut self.body {
            Body::Leaf(_) => None,
            Body::Tree(c) => {
                if depth == 0 {
                    //explode here.
                    let l = c.left.get_value().unwrap();
                    let r = c.right.get_value().unwrap();
                    return Some((true, Some(l), Some(r)));
                }
                let act = c
                    .left
                    .try_explode(depth - 1)
                    .map(|x| (x, Dir::Left))
                    .or_else(|| c.right.try_explode(depth - 1).map(|x| (x, Dir::Right)));
                match act {
                    Some(((reset, l, r), d)) => {
                        if reset {
                            match d {
                                Dir::Left => c.left.body = Body::Leaf(0),
                                Dir::Right => c.right.body = Body::Leaf(0),
                            }
                        }
                        match &mut self.body {
                            Body::Leaf(_) => unreachable!(),
                            Body::Tree(c) => {
                                if d == Dir::Left && r.is_some() {
                                    c.right.add_to_edge(Dir::Left, r.unwrap());
                                    Some((false, l, None))
                                } else if d == Dir::Right && l.is_some() {
                                    c.left.add_to_edge(Dir::Right, l.unwrap());
                                    Some((false, None, r))
                                } else {
                                    Some((false, l, r))
                                }
                            }
                        }
                    }
                    None => None,
                }
            }
        }
    }
    fn add_to_edge(&mut self, dir: Dir, val: usize) {
        match &mut self.body {
            Body::Leaf(n) => {
                *n += val;
            }
            Body::Tree(c) => match dir {
                Dir::Left => c.left.add_to_edge(dir, val),
                Dir::Right => c.right.add_to_edge(dir, val),
            },
        }
    }
    fn reduce(&mut self) {
        loop {
            // println!("{}", self);
            if self.try_explode(4).is_none() && !self.try_split() {
                break;
            }
        }
    }
    fn get_children(&self) -> Option<(&Self, &Self)> {
        match &self.body {
            Body::Leaf(_) => None,
            Body::Tree(c) => Some((&c.left, &c.right)),
        }
    }
    fn get_value(&self) -> Option<usize> {
        match &self.body {
            Body::Leaf(l) => Some(*l),
            Body::Tree(_) => None,
        }
    }
    fn magnitude(&self) -> usize {
        match &self.body {
            Body::Leaf(a) => *a,
            Body::Tree(c) => 3 * c.left.magnitude() + 2 * c.right.magnitude(),
        }
    }
}
#[derive(Debug)]
struct SnailNumberView<'a> {
    current_node: &'a SnailNumber,
    i_am_of_parent: Option<Dir>,
    parent: Option<Box<SnailNumberView<'a>>>,
}

fn p1(input: &[SnailNumber]) -> usize {
    let mut ans = input[0].clone();
    for i in &input[1..] {
        ans = ans + i.clone();
    }
    ans.magnitude()
}
fn p2(input: &[SnailNumber]) -> usize {
    input
        .iter()
        .permutations(2)
        .map(|v| {
            let a = v[0].clone() + v[1].clone();
            a.magnitude()
        })
        .max()
        .unwrap()
}
#[cfg(test)]
mod snailtest {
    use std::str::FromStr;

    use super::*;

    fn exp(input: &str) -> String {
        let mut p: SnailNumber = input.parse().unwrap();
        p.try_explode(4);
        return format!("{}", p);
    }
    fn spl(input: &str) -> String {
        let mut p: SnailNumber = input.parse().unwrap();
        p.try_split();
        return format!("{}", p);
    }
    fn add(a: &str, b: &str) -> String {
        let mut a: SnailNumber = a.parse().unwrap();
        let b: SnailNumber = b.parse().unwrap();
        let c = a + b;
        return format!("{}", c);
    }
    #[test]
    fn test_explodes() {
        assert_eq!(exp("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");
        assert_eq!(exp("[7,[6,[5,[4,[3,2]]]]]"), "[7,[6,[5,[7,0]]]]");
        assert_eq!(exp("[[6,[5,[4,[3,2]]]],1]"), "[[6,[5,[7,0]]],3]");
        assert_eq!(
            exp("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
        );
        assert_eq!(
            exp("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        );
    }
    #[test]
    fn test_split() {
        assert_eq!(
            spl("[[[[0,7],4],[F,[0,D]]],[1,1]]"),
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
        );
    }
    #[test]
    fn test_addition() {
        const SCRIPT: &str = "
  [[[[4,3],4],4],[7,[[8,4],9]]]
+ [1,1]
= [[[[0,7],4],[[7,8],[6,0]]],[8,1]]

  [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
+ [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
= [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]

  [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
+ [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
= [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]

  [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
+ [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
= [[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]

  [[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
+ [7,[5,[[3,8],[1,4]]]]
= [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]

  [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]
+ [[2,[2,2]],[8,[8,1]]]
= [[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]

  [[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]
+ [2,9]
= [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]

  [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
+ [1,[[[9,3],9],[[9,0],[0,7]]]]
= [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]

  [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]
+ [[[5,[7,4]],7],1]
= [[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]

  [[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]
+ [[[[4,2],2],6],[8,7]]
= [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

        for sum in SCRIPT.trim_matches('\n').split("\n\n") {
            let mut i = sum.lines();
            let a = &i.next().unwrap()[2..];
            let b = &i.next().unwrap()[2..];
            let c = &i.next().unwrap()[2..];
            assert_eq!(add(a, b), c);
        }
    }
}
