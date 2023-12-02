use std::collections::HashMap;

use aoc_harness::*;
use nom::{IResult, bytes::complete::tag, sequence::{preceded, separated_pair, terminated}, character::complete::{self, newline}, branch::alt, combinator::{value, all_consuming}, multi::{many1, separated_list1}};

aoc_main!(2023 day 2, part1 [p1], part2 [p2], example part1 EG => 8, example part2 EG =>2286);

#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone)]
enum Colour {
    Red,
    Green,
    Blue
}
#[derive(Debug)]
struct Game {
    id: u32,
    shows: Vec<HashMap<Colour,u32>>
}

fn parse_colour(input: &str) -> IResult<&str, Colour> {
    alt((
        value(Colour::Red, tag("red")),
        value(Colour::Blue, tag("blue")),
        value(Colour::Green, tag("green")),
    ))(input)
}
fn parse_show(input: &str) -> IResult<&str, HashMap<Colour,u32>> {
    let (input, shows) = nom::multi::separated_list1(tag(", "), separated_pair(complete::u32, tag(" "), parse_colour))(input)?;
    Ok((input, shows.into_iter().map(|(x,y)| (y,x)).collect()))
}
fn parse_line(input: &str) -> IResult<&str, Game> {
    let (input,id) = preceded(tag("Game "), complete::u32)(input)?;
    let (input, shows) = preceded(tag(": "), separated_list1(tag("; "), parse_show))(input)?;
    Ok((input,Game {id, shows}))
}

fn p1(input: &str) -> u32 {
    let (_, games) = all_consuming(many1(terminated(parse_line, newline)))(input).unwrap();
    let limits : HashMap<Colour, u32> = [(Colour::Red, 12), (Colour::Green, 13), (Colour::Blue, 14)].into_iter().collect();
    let mut total = 0;
    for game in games {
        let ok = game.shows.iter().all(|s| s.iter().all(|(c,count)| count <= &limits[c] ));
        if ok {
            total += game.id
        }
    }
    total
}

const EG: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

fn p2(input: &str) -> u32 {
    let (_, games) = all_consuming(many1(terminated(parse_line, newline)))(input).unwrap();
    let mut total = 0;
    for game in games {
        let minimum = game.shows.iter().fold(HashMap::new(), |mut map, s| {
            for (c,count) in s.iter() {
                let x : &mut u32 = map.entry(*c).or_default();
                *x = *count.max(x)
            }
            map
        });
        total += minimum.values().product::<u32>();
    }
    total
}