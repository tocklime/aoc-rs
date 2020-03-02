use nom::bytes::complete::*;
use nom::IResult;
use nom::sequence::tuple;
use nom::character::complete::{digit1, alpha1};
use nom::multi::separated_list;
use nom::branch::alt;
use itertools::Itertools;
use nom::combinator::opt;


#[derive(Debug)]
struct AttackGroup {
    size: u32,
    hp: u32,
    immunities: Vec<String>,
    weaknesses: Vec<String>,
    attack_strength: u32,
    attack_type: String,
    initiative: u32
}

impl AttackGroup {
    fn parse_a_status_set<'a>(i: &'a str, name: &'a str) -> IResult<&'a str, Vec<String>> {
        let (i, _) = tag(name)(i)?;
        let (i, _) = tag(" to ")(i)?;
        let (i,ns) = separated_list(tag(", "),alpha1)(i)?;
        let (i,_) = alt((tag("; "),tag(") ")))(i)?;
        let our_names = ns.iter().map(|x| x.to_string()).collect_vec();
        Ok((i,our_names))
    }
    fn parse_statuses(i: &str) -> IResult<&str, (Vec<String>,Vec<String>)> {
        let (i,_) = tag("(")(i)?;
        let (i, immune) = Self::parse_a_status_set(i,"immune").unwrap_or((i,Vec::new()));
        let (i, weak) = Self::parse_a_status_set(i,"weak").unwrap_or((i,Vec::new()));
        let (i, immune2) = Self::parse_a_status_set(i,"immune").unwrap_or((i,Vec::new()));
        dbg!(i,&immune,&weak);
        Ok((i,(if immune.is_empty() {immune2} else {immune},weak)))
    }

    fn parse(i: &str) -> IResult<&str, AttackGroup> {
        tuple((
            digit1,
            tag(" units each with "),
            digit1,
            tag(" hit points "),
            opt(Self::parse_statuses),
            tag ("with an attack that does "),
            digit1,
            tag(" "),
            alpha1,
            tag(" damage at initiative "),
            digit1
        ))(i)
            .map(|(i,(s,_,h,_,st,_,a,_,at,_,init))| {
                let st = st.unwrap_or((Vec::new(),Vec::new()));
                (i,Self {
                    size: s.parse().unwrap(),
                    hp: h.parse().unwrap(),
                    immunities: st.0,
                    weaknesses: st.1,
                    attack_strength: a.parse().unwrap(),
                    attack_type: at.to_string(),
                    initiative: init.parse().unwrap()
                })
            })
    }
}

#[aoc(day24,part1)]
fn p1(input: &str) -> usize {
    let input = input.replace("\r","");
    let groups = input.split("\n\n").collect_vec();
    let immune = groups[0].lines().skip(1).map(|l| AttackGroup::parse(l).unwrap()).collect_vec();
    let infection = groups[1].lines().skip(1).map(|l| AttackGroup::parse(l).unwrap()).collect_vec();
    dbg!(immune,infection);
    0
}