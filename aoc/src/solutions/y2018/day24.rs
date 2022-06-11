aoc_harness::aoc_main!(2018 day 24, part1 [p1], part2 [p2]);
use nom::bytes::complete::*;
use nom::IResult;
use nom::sequence::tuple;
use nom::character::complete::{digit1, alpha1};
use nom::multi::separated_list0;
use nom::branch::alt;
use itertools::Itertools;
use nom::combinator::opt;
use std::cmp::min;
use std::collections::HashSet;
use std::cell::Cell;


#[derive(Debug, PartialEq, Eq)]
struct AttackGroup {
    full_size: u32,
    size: Cell<u32>,
    hp: u32,
    immunities: Vec<String>,
    weaknesses: Vec<String>,
    attack_strength: u32,
    attack_type: String,
    initiative: u32,
}

impl AttackGroup {
    fn parse_a_status_set<'a>(i: &'a str, name: &'a str) -> IResult<&'a str, Vec<String>> {
        let (i, _) = tag(name)(i)?;
        let (i, _) = tag(" to ")(i)?;
        let (i, ns) = separated_list0(tag(", "), alpha1)(i)?;
        let (i, _) = alt((tag("; "), tag(") ")))(i)?;
        let our_names = ns.iter().map(|x| x.to_string()).collect_vec();
        Ok((i, our_names))
    }
    fn parse_statuses(i: &str) -> IResult<&str, (Vec<String>, Vec<String>)> {
        let (i, _) = tag("(")(i)?;
        let (i, immune) = Self::parse_a_status_set(i, "immune").unwrap_or((i, Vec::new()));
        let (i, weak) = Self::parse_a_status_set(i, "weak").unwrap_or((i, Vec::new()));
        let (i, immune2) = Self::parse_a_status_set(i, "immune").unwrap_or((i, Vec::new()));
        Ok((i, (if immune.is_empty() { immune2 } else { immune }, weak)))
    }

    fn effective_power(&self, boost: u32) -> u32 {
        self.size.get() * (self.attack_strength + boost)
    }
    fn calc_damage(&self, amount: u32, attack_type: &str) -> u32 {
        if self.immunities.iter().any(|x| x == attack_type) {
            0
        } else if self.weaknesses.iter().any(|x| x == attack_type) {
            amount * 2
        } else {
            amount
        }
    }
    fn inflict_damage(&self, amount: u32) -> u32 {
        let killed = min(self.size.get(), amount / self.hp);
        let new_sz = self.size.get() - killed;
        self.size.set(new_sz);
        killed
    }

    fn parse(i: &str) -> IResult<&str, AttackGroup> {
        tuple((
            digit1,
            tag(" units each with "),
            digit1,
            tag(" hit points "),
            opt(Self::parse_statuses),
            tag("with an attack that does "),
            digit1,
            tag(" "),
            alpha1,
            tag(" damage at initiative "),
            digit1
        ))(i)
            .map(|(i, (s, _, h, _, st, _, a, _, at, _, init))| {
                let st = st.unwrap_or((Vec::new(), Vec::new()));
                let size = s.parse().unwrap();
                (i, Self {
                    full_size: size,
                    size: Cell::new(size),
                    hp: h.parse().unwrap(),
                    immunities: st.0,
                    weaknesses: st.1,
                    attack_strength: a.parse().unwrap(),
                    attack_type: at.to_string(),
                    initiative: init.parse().unwrap(),
                })
            })
    }
}

fn assign_targets<'a>(attackers: &'a [AttackGroup], defenders: &'a [AttackGroup],boost: u32,defenders_boost: u32) -> Vec<(usize, usize)> {
    let mut available: HashSet<usize> = (0..defenders.len()).filter(|&x| defenders[x].size.get() > 0).collect();
    let mut targets = Vec::new();
    for (ix, (power,attacker)) in attackers.iter().map(|x| (x.effective_power(boost),x)).enumerate().sorted_by_key(|(_, x)| (x.0,x.1.initiative)).rev() {
        let best_t = available.iter().map(|&d_ix| {
            let d = &defenders[d_ix];
            let dam = d.calc_damage(power,&attacker.attack_type);
            let def_pow = d.effective_power(defenders_boost);
            let def_init = d.initiative;
            (dam,def_pow,def_init,d_ix)
        }).max();
        if let Some((dam,_,_,d_ix)) = best_t {
            if dam > 0 {
                available.remove(&d_ix);
                targets.push((ix, d_ix));
            }
        }
    }
    targets
}

fn run(immune: &[AttackGroup], infection: &[AttackGroup], boost: u32) -> (u32,u32) {
    let teams = vec![immune, infection];
    for i in immune {
        i.size.set(i.full_size);
    }
    for i in infection {
        i.size.set(i.full_size);
    }
    loop {
        //targeting
        let immune_targets = assign_targets(&immune, &infection,boost,0);
        let infection_targets = assign_targets(&infection, &immune,0,boost);
        //attacking
        let mut died = 0;
        for (&(a, b), team) in immune_targets.iter().map(|x| (x, 0)).chain(infection_targets.iter().map(|x| (x, 1))).sorted_by_key(|(x, t)| teams[*t][x.0].initiative).rev() {
            let attacker = &teams[team][a];
            let defender = &teams[1 - team][b];
            let boost = if team == 0 {boost} else {0};
            let damage = defender.calc_damage(attacker.effective_power(boost), &attacker.attack_type);
            let this_time = defender.inflict_damage(damage);
            //println!("T{}: {} -> {} dam: {}, kills: {}",team,a+1,b+1,damage,this_time);
            died += this_time;
        }
        //println!("");
        let imm_score: u32 = immune.iter().map(|x| x.size.get()).sum();
        let inf_score: u32 = infection.iter().map(|x| x.size.get()).sum();
        if imm_score == 0 || inf_score == 0 || died == 0 {
            break;
        }
    }
    let imm_score: u32 = immune.iter().map(|x| x.size.get()).sum();
    let inf_score: u32 = infection.iter().map(|x| x.size.get()).sum();
    (imm_score,inf_score)
}


fn p1(input: &str) -> u32 {
    let input = input.replace("\r", "");
    let groups = input.split("\n\n").collect_vec();
    let immune = groups[0].lines().skip(1).map(|l| AttackGroup::parse(l).unwrap().1).collect_vec();
    let infection = groups[1].lines().skip(1).map(|l| AttackGroup::parse(l).unwrap().1).collect_vec();
    run(&immune,&infection,0).1
}

fn p2(input: &str) -> u32 {
    let input = input.replace("\r", "");
    let groups = input.split("\n\n").collect_vec();
    let immune = groups[0].lines().skip(1).map(|l| AttackGroup::parse(l).unwrap().1).collect_vec();
    let infection = groups[1].lines().skip(1).map(|l| AttackGroup::parse(l).unwrap().1).collect_vec();
    (0..).map(|x| run(&immune,&infection,x)).find(|&(_,b)| b == 0).unwrap().0
}
//1627 too low. 11 wrong.