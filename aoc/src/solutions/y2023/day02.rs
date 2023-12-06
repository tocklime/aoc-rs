
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

aoc_harness::aoc_main!(2023 day 2, generator gen, part1 [p1] => 2771, part2 [p2] => 70924, example both EG => (8, 2286));

#[derive(Debug, Default, PartialEq)]
struct Colours {
    red: u32,
    green: u32,
    blue: u32,
}
impl PartialOrd for Colours {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            Some(std::cmp::Ordering::Less)
        } else if self.red >= other.red && self.green >= other.green && self.blue >= other.blue {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}
impl Colours {
    fn max(&self, other: &Colours) -> Colours {
        Colours {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
    fn product(&self) -> u32 {
        self.red * self.blue * self.green
    }
}
#[derive(Debug)]
struct Game {
    id: u32,
    shows: Vec<Colours>,
}

fn parse_show(input: &str) -> IResult<&str, Colours> {
    let mut ans = Colours::default();
    let (input, _) = nom::multi::separated_list1(tag(", "), |i| {
        let (i, count) = terminated(complete::u32, tag(" "))(i)?;
        let (i, _) = alt((
            map(tag("red"), |_| ans.red += count),
            map(tag("blue"), |_| ans.blue += count),
            map(tag("green"), |_| ans.green += count),
        ))(i)?;
        Ok((i, ()))
    })(input)?;
    Ok((input, ans))
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), complete::u32)(input)?;
    let (input, shows) = preceded(tag(": "), separated_list1(tag("; "), parse_show))(input)?;
    Ok((input, Game { id, shows }))
}

fn gen(input: &str) -> Vec<Game> {
    all_consuming(many1(terminated(parse_line, newline)))(input)
        .unwrap()
        .1
}
fn p1(games: &[Game]) -> u32 {
    let limits = Colours {
        red: 12,
        green: 13,
        blue: 14,
    };
    games
        .iter()
        .filter_map(|game| game.shows.iter().all(|s| s <= &limits).then_some(game.id))
        .sum()
}

fn p2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            game.shows
                .iter()
                .fold(Colours::default(), |max, s| max.max(s))
                .product()
        })
        .sum()
}

const EG: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
