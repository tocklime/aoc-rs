use nom::IResult;
use nom::character::complete::{alpha1, digit1};
use nom::bytes::complete::tag;
use nom::combinator::{opt};
use nom::sequence::tuple;
use nom::multi::separated_list;
use itertools::Itertools;
use nom::lib::std::collections::HashMap;

#[derive(Debug)]
struct Program<'a> {
    name: &'a str,
    size: i32,
    holding: Vec<&'a str>,
}
impl<'a> Program<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        let (i, (name,_,size,_,children)) = tuple((alpha1,tag(" ("),digit1,tag(")"),opt(tag(" -> "))))(input)?;
        let (i,holding) = if children.is_some() {
            separated_list(tag(", "),alpha1)(i)?
        } else { (i, Vec::new())};
        Ok((i,Self { name, size: size.parse().unwrap(), holding }))
    }
    fn total_weight(&self, lu: &'a HashMap<&'a str,&Self>) -> i32 {
        self.size + self.holding.iter().map(|n| lu[n].total_weight(lu)).sum::<i32>()
    }
    fn needed_weight_edit(&self, lu: &'a HashMap<&'a str,&Self>) -> Option<(&str,i32)> {
        let mut seen = HashMap::new();
        for h in &self.holding {
            let w = lu[h].total_weight(lu);
            let e = seen.entry(w).or_insert((h,0));
            e.1 += 1;
        }
        let good = seen.iter().find(|(_,b)| b.1 > 1).unwrap().0;
        seen.iter().find_map(|(a,b)| if b.1 == 1 {Some((*b.0,*good - *a)) } else {None})
    }
}


fn p1(input: &str) -> String {
    let progs = input.lines().map(|l| Program::parse(l).unwrap().1).collect_vec();
    let parents = progs.iter().flat_map(|i| i.holding.iter().map(move|c| (c,i.name))).collect::<HashMap<_,_>>();
    let mut n = progs[0].name;
    while parents.contains_key(&n) {
        n = parents[&n];
    }
    n.to_string()
}


fn p2(input: &str) -> i32 {
    let progs = input.lines().map(|l| Program::parse(l).unwrap().1).collect_vec();
    let lookup = progs.iter().map(|p| (p.name,p)).collect::<HashMap<_,_>>();
    let parents = progs.iter().flat_map(|i| i.holding.iter().map(move|c| (c,i.name))).collect::<HashMap<_,_>>();
    let mut root = progs[0].name;
    while parents.contains_key(&root) {
        root = parents[&root];
    }
    let mut problem = lookup[&root];
    let mut offsize_by = 0;
    loop {
        let n = problem.needed_weight_edit(&lookup);
        match n {
            None => {return problem.size + offsize_by;}
            Some(c) => {
                offsize_by = c.1;
                problem = lookup[c.0];
            }
        }
    }
}
//8094 too high
//15 wrong