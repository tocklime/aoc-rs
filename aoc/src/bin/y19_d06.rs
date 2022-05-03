use aoc_harness::*;
use std::cell::RefCell;
use std::collections::HashMap;
aoc_main!(2019 day 6, part1 [p1] => 227612, part2 [p2] => 454, example part1 EG1 => 42, example part2 EG2 => 4);

struct OrbitalMap<'a> {
    map: HashMap<&'a str, &'a str>,
    depth_cache: RefCell<HashMap<&'a str, usize>>,
}

impl<'a> OrbitalMap<'a> {
    pub fn from_str(input: &'a str) -> OrbitalMap<'a> {
        OrbitalMap {
            map: input
                .lines()
                .map(|l| l.split(')').collect::<Vec<&str>>())
                .map(|a| (a[1], a[0]))
                .collect(),
            depth_cache: RefCell::new(HashMap::new()),
        }
    }
    fn get_chain_to_root(&self, obj: &'a str) -> Vec<&str> {
        let mut curr: Option<&&str> = Some(&obj);
        let mut vec: Vec<&str> = Vec::new();
        while let Some(x) = curr {
            vec.push(x);
            curr = self.map.get(x);
        }
        vec
    }
    pub fn get_depth(&'a self, obj: &'a str) -> usize {
        let mut dc = self.depth_cache.borrow_mut();
        dc.get(obj).cloned().unwrap_or_else(|| {
            let chain = self.get_chain_to_root(obj);
            for (ix, i) in chain.iter().enumerate() {
                if dc.contains_key(i) {
                    break;
                }
                dc.insert(i, chain.len() - ix);
            }
            chain.len()
        })
    }
}
pub fn p1(input: &str) -> usize {
    let ors = OrbitalMap::from_str(input);
    ors.map.values().map(|x| ors.get_depth(x)).sum()
}
pub fn p2(input: &str) -> usize {
    let ors = OrbitalMap::from_str(input);
    let my_chain = ors.get_chain_to_root("YOU");
    let san_chain = ors.get_chain_to_root("SAN");
    let common_prefix_len = my_chain
        .iter()
        .rev()
        .zip(san_chain.iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    my_chain.len() + san_chain.len() - 2 * (common_prefix_len + 1)
}

const EG1: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
const EG2: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
