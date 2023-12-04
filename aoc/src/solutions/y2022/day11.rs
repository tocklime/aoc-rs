use aoc_harness::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};

aoc_main!(2022 day 11, generator gen, part1 [solve::<3, 20>] => 110885, part2 [solve::<1, 10000>] => 25272176808, example both EG => (10605, 2713310158));

const EG: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

#[derive(Debug, Clone)]
enum Operation {
    AddI(u64),
    MulI(u64),
    Double,
    Square,
}
#[derive(Debug, Clone)]
struct Monkey {
    held: Vec<u64>,
    alter: Operation,
    test_div: u64,
    targets: [usize; 2],
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("  Operation: new = old ")(input)?;
    let (input, op_str) = terminated(alt((tag("+"), tag("*"))), tag(" "))(input)?;
    let (input, val) = alt((value(None, tag("old")), map(u64, Some)))(input)?;
    let (input, _) = newline(input)?;
    let ans = match (op_str, val) {
        ("+", None) => Operation::Double,
        ("+", Some(i)) => Operation::AddI(i),
        ("*", None) => Operation::Square,
        ("*", Some(i)) => Operation::MulI(i),
        a => panic!("bad match in parse operation: {a:?}"),
    };
    Ok((input, ans))
}
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _monkey_ix) = delimited(tag("Monkey "), u64, tag(":\n"))(input)?;
    let (input, held) = delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), u64),
        newline,
    )(input)?;
    let (input, alter) = parse_operation(input)?;
    let (input, test_div) = delimited(tag("  Test: divisible by "), u64, newline)(input)?;
    let (input, true_target) =
        delimited(tag("    If true: throw to monkey "), u64, newline)(input)?;
    let (input, false_target) =
        delimited(tag("    If false: throw to monkey "), u64, newline)(input)?;
    let true_target = true_target as usize;
    let false_target = false_target as usize;
    let monkey = Monkey {
        held,
        alter,
        test_div,
        targets: [false_target, true_target],
    };
    Ok((input, monkey))
}

fn gen(input: &str) -> Vec<Monkey> {
    all_consuming(separated_list1(newline, parse_monkey))(input)
        .unwrap()
        .1
}

fn solve<const DIV: u64, const ROUNDS: usize>(input: &[Monkey]) -> usize {
    let mut monkeys: Vec<Monkey> = input.to_vec();
    let monkey_count = monkeys.len();
    let mut inspection_counts = vec![0; monkey_count];
    let big_modulo: u64 = monkeys.iter().map(|m| m.test_div).product();
    for _round in 0..ROUNDS {
        for m_ix in 0..monkeys.len() {
            let item_count = monkeys[m_ix].held.len();
            inspection_counts[m_ix] += item_count;
            for ix in 0..item_count {
                let me = &monkeys[m_ix];
                let worry = me.held[ix];
                let mut new_worry = match me.alter {
                    Operation::AddI(x) => worry + x,
                    Operation::MulI(x) => worry * x,
                    Operation::Double => 2 * worry,
                    Operation::Square => worry * worry,
                };
                new_worry /= DIV;
                if new_worry > big_modulo {
                    new_worry %= big_modulo;
                }
                let new_t = me.targets[usize::from(new_worry % me.test_div == 0)];
                monkeys[new_t].held.push(new_worry);
            }
            monkeys[m_ix].held.clear();
        }
    }
    inspection_counts.sort();
    inspection_counts[monkey_count - 1] * inspection_counts[monkey_count - 2]
}
