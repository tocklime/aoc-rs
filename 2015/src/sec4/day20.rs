
fn presents(n: u32) -> u32 {
    let x : u32 = (1_u32..=n).filter(|e| n % e == 0).sum();
    x * 10
}

#[aoc(day20,part1)]
fn p1(input: &str) -> u32 {
    let target = input.parse::<u32>().unwrap();
    (1..).find(|&h| presents(h) >= target).unwrap()
}

#[test]
fn day20p1tests(){
    assert_eq!(presents(1), 10);
    assert_eq!(presents(2), 30);
    assert_eq!(presents(3), 40);
    assert_eq!(presents(4), 70);
    assert_eq!(presents(5), 60);
    assert_eq!(presents(6), 120);
    assert_eq!(presents(7), 80);
    assert_eq!(presents(8), 150);
    assert_eq!(presents(9), 130);
    assert_eq!(presents(210), 130);
    assert_eq!(presents(1274999), 14_361_600);

}