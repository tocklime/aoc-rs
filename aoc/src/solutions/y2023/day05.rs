use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::Parser;
use nom::combinator::all_consuming;
use nom::{
    character::complete::{self, alpha1, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
};
use utils::{nom::IResult, span::Span};

aoc_harness::aoc_main!(2023 day 5, generator gen_, part1 [p1] => 289_863_851, part2 [p2] => 60_568_880, example both EG => (35,46));

#[derive(Debug)]
struct Map {
    _from: String,
    _to: String,
    ranges: Vec<(Span<i64>, i64)>,
}
#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

impl Map {
    fn parse(input: &str) -> IResult<'_, Self> {
        let (input, (from, to)) = separated_pair(alpha1::<&str, _>, tag("-to-"), alpha1)
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .parse(input)?;
        let (input, _) = tag(" map:\n").parse(input)?;
        let (input, mut ranges) = nom::multi::separated_list1(
            newline,
            (complete::i64, space1, complete::i64, space1, complete::i64)
                .map(|(t, _, f, _, size)| (Span::new(f, f + size), t - f)),
        )
        .parse(input)?;
        ranges.sort();
        Ok((
            input,
            Self {
                _from: from,
                _to: to,
                ranges,
            },
        ))
    }
}
impl Almanac {
    fn parse(input: &str) -> IResult<'_, Self> {
        let (input, seeds) = (tag("seeds: "), separated_list1(space1, complete::i64))
            .map(|x| x.1)
            .parse(input)?;
        let (input, _) = tag("\n\n")(input)?;
        let (input, maps) = separated_list1((newline, newline), Map::parse).parse(input)?;
        let (input, _) = newline(input)?;
        Ok((input, Self { seeds, maps }))
    }
    fn convert_range_and_minimise(&self, from: i64, size: i64) -> i64 {
        let init = Span::new(from, from + size);
        self.maps
            .iter()
            .fold(vec![init], |spans, m| {
                spans
                    .iter()
                    .flat_map(|span| {
                        let (mut ans, last_overlap_end) = m.ranges.iter().fold(
                            (Vec::new(), span.start),
                            |(mut ans, start), (from, delta)| {
                                if let Some(i) = span.intersection(from) {
                                    if start < i.start {
                                        ans.push(Span::new(start, i.start));
                                    }
                                    ans.push(i + *delta);
                                    (ans, i.end)
                                } else {
                                    (ans, start)
                                }
                            },
                        );
                        if last_overlap_end < span.end {
                            ans.push(Span::new(last_overlap_end, span.end));
                        }
                        ans
                    })
                    .collect()
            })
            .into_iter()
            .map(|x| x.start)
            .min()
            .unwrap()
    }
    fn convert(&self, input: i64) -> i64 {
        self.maps.iter().fold(input, |x, m| {
            let output = m
                .ranges
                .iter()
                .find_map(|&(from, delta)| from.contains(x).then_some(x + delta))
                .unwrap_or(x);
            output
        })
    }
}

fn gen_(input: &str) -> Almanac {
    use nom::Parser;
    all_consuming(Almanac::parse)
        .parse(input)
        .expect("Parse")
        .1
}

fn p1(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .iter()
        .map(|&s| almanac.convert(s))
        .min()
        .unwrap()
}

fn p2(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .iter()
        .tuples()
        .map(|(&a, &b)| almanac.convert_range_and_minimise(a, b))
        .min()
        .unwrap()
}
const EG: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
