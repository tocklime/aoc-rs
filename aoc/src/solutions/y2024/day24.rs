use std::collections::HashMap;

aoc_harness::aoc_main!(2024 day 24, part1 [p1] => 43_559_017_878_162, example part1 EG => 4, example part1 EG2 => 2024);

fn p1(input: &str) -> u64 {
    let mut values : HashMap<&str, bool> = HashMap::new();
    let (inits, rules) = input.split_once("\n\n").unwrap();
    for l in inits.lines() {
        let (a,b) = l.split_once(": ").unwrap();
        let b = b == "1";
        values.insert(a, b);
    }
    let mut pending_rules : Vec<&str> = rules.lines().collect();
    while !pending_rules.is_empty() {
        pending_rules.retain(|r| {
            let (a, target) = r.split_once(" -> ").unwrap();
            let mut i = a.split(' ');
            let a = i.next().unwrap();
            let op = i.next().unwrap();
            let b = i.next().unwrap();
            match (values.get(a), values.get(b)) {
                (None, _) | (_, None) => true,
                (Some(&x), Some(&y)) => {
                    let v = match op {
                        "XOR" => x != y,
                        "OR" => x || y,
                        "AND" => x && y,
                        _ => unreachable!()
                    };
                    values.insert(target, v);
                    false
                }
            }
        });
    }
    let mut ans = 0;
    for (k,v) in values {
        if v && k.starts_with('z') {
            let n : usize = k[1..].parse().unwrap();
            ans |= 1 << n;
        }
    }
    ans
}

const EG: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";
const EG2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";
