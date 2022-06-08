use aoc_harness::aoc_main;

aoc_main!(2015 day 12, part1 [p1], part2 [p2]);
use json::JsonValue;

fn num_val(j: &JsonValue) -> f64 {
    match j {
        JsonValue::Number(n) => (*n).into(),
        JsonValue::Array(a) => a.iter().map(num_val).sum(),
        JsonValue::Object(o) => o.iter().map(|(_, x)| num_val(x)).sum(),
        _ => 0.0,
    }
}

fn num_val_2(j: &JsonValue) -> f64 {
    match j {
        JsonValue::Number(n) => (*n).into(),
        JsonValue::Array(a) => a.iter().map(num_val_2).sum(),
        JsonValue::Object(o) => {
            let red = o.iter().any(|(_,v)| v.as_str() == Some("red"));
            if red {0.0} else {o.iter().map(|(_, x)| num_val_2(x)).sum()}
        },
        _ => 0.0,
    }
}


fn p1(input: &str) -> i64 {
    num_val(&json::parse(input).expect("Bad JSON")) as i64
}


fn p2(input: &str) -> i64 {
    num_val_2(&json::parse(input).expect("Bad JSON")) as i64
}
