use aoc_harness::aoc_main;

aoc_main!(2015 day 21, part1 [p1], part2 [p2]);
use itertools::Itertools;

struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

fn weapons() -> Vec<Item> {
    vec![
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ]
}

fn armor() -> Vec<Item> {
    vec![
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ]
}

fn rings() -> Vec<Item> {
    vec![
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ]
}

/*
Hit Points: 103
Damage: 9
Armor: 2*/

struct Fighter {
    hit_points: usize,
    damage: usize,
    armor: usize,
}

fn hits_to_kill(hitter: &Fighter, hittee: &Fighter) -> usize {
    let per_hit = hitter.damage - hittee.armor;
    let base = hittee.hit_points / per_hit;
    if hittee.hit_points % per_hit == 0 {
        base
    } else {
        base + 1
    }
}

fn fight(me: &Fighter, boss: &Fighter) -> bool {
    if me.damage <= boss.armor {
        return false;
    }
    if boss.damage <= me.armor {
        return true;
    }
    hits_to_kill(me, boss) <= hits_to_kill(boss, me)
}

fn equip(items: &[&&Item]) -> Fighter {
    Fighter {
        hit_points: 100,
        damage: items.iter().map(|x| x.damage).sum(),
        armor: items.iter().map(|x| x.armor).sum(),
    }
}

fn p1(input: &str) -> usize {
    let data = input
        .lines()
        .map(|x| {
            x.split(':')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap()
        })
        .collect_vec();
    let boss = Fighter {
        hit_points: data[0],
        damage: data[1],
        armor: data[2],
    };
    let weapons = weapons();
    let armor = armor();
    let rings = rings();
    let weapon_choices: Vec<Vec<&Item>> = (1..=1)
        .flat_map(|x| weapons.iter().combinations(x))
        .collect_vec();
    let armor_choices: Vec<Vec<&Item>> = (0..=1)
        .flat_map(|c| armor.iter().combinations(c))
        .collect_vec();
    let ring_choices: Vec<Vec<&Item>> = (0..=2)
        .flat_map(|c| rings.iter().combinations(c))
        .collect_vec();

    [weapon_choices, armor_choices, ring_choices]
        .iter()
        .multi_cartesian_product()
        .map(|c: Vec<&Vec<&Item>>| c.iter().flat_map(|&x| x).collect_vec())
        .filter(|c| fight(&equip(c), &boss))
        .map(|c| c.iter().map(|i| i.cost).sum())
        .min()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let data = input
        .lines()
        .map(|x| {
            x.split(':')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap()
        })
        .collect_vec();
    let boss = Fighter {
        hit_points: data[0],
        damage: data[1],
        armor: data[2],
    };
    let weapons = weapons();
    let armor = armor();
    let rings = rings();
    let weapon_choices: Vec<Vec<&Item>> = (1..=1)
        .flat_map(|x| weapons.iter().combinations(x))
        .collect_vec();
    let armor_choices: Vec<Vec<&Item>> = (0..=1)
        .flat_map(|c| armor.iter().combinations(c))
        .collect_vec();
    let ring_choices: Vec<Vec<&Item>> = (0..=2)
        .flat_map(|c| rings.iter().combinations(c))
        .collect_vec();

    [weapon_choices, armor_choices, ring_choices]
        .iter()
        .multi_cartesian_product()
        .map(|c: Vec<&Vec<&Item>>| c.iter().flat_map(|&x| x).collect_vec())
        .filter(|c| !fight(&equip(c), &boss))
        .map(|c| c.iter().map(|i| i.cost).sum())
        .max()
        .unwrap()
}
