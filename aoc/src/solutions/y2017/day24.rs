use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra_all;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Part {
    a: usize,
    b: usize,
}

impl Part {
    fn from_str(input: &str) -> Self {
        let a = input.split('/').map(|a| a.parse::<usize>().unwrap()).collect_vec();
        Part { a: a[0], b: a[1] }
    }
}

fn get_all_bridges(input: &str) -> HashMap<(usize,Vec<Part>),((usize,Vec<Part>),usize)>
{
    let parts = input.lines().map(Part::from_str).collect_vec();
    dijkstra_all(
        &(0, parts),
        |(open_port, bits)| {
            bits.iter().filter_map(move |c|
                if c.a == *open_port {
                    let left = bits.iter().filter(|&x| x != c).cloned().collect_vec();
                    Some(((c.b, left), c.a + c.b))
                } else if c.b == *open_port {
                    let left = bits.iter().filter(|&x| x != c).cloned().collect_vec();
                    Some(((c.a, left), c.a + c.b))
                } else {
                    None
                }
            ).collect_vec()
        },
    )
}


fn p1(input: &str) -> usize {
    get_all_bridges(input).values().map(|x| x.1).max().unwrap()
}


fn p2(input: &str) -> usize {
    let n = input.lines().count();
    let x = get_all_bridges(input);
    *x.values().map(|((_,rem),str)| (n - rem.len(),str)).max().unwrap().1
}
