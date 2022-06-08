use aoc_harness::aoc_main;

aoc_main!(2015 day 22, generator gen, part1 [p1], part2 [p2]);
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct State {
    my_health: isize,
    boss_health: isize,
    boss_damage: isize,
    shield_turns: usize,
    poison_turns: usize,
    recharge_turns: usize,
    my_mana: isize,
    boss_armor: usize,
    my_armor: isize,
    is_hard_mode: bool,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn all() -> &'static [Self; 5] {
        static SPELLS: [Spell; 5] = [
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ];
        &SPELLS
    }
    fn mana_cost(&self) -> isize {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

impl State {
    fn effect_is_recastable(&self, sp: Spell) -> bool {
        match sp {
            Spell::Shield => self.shield_turns <= 1,
            Spell::Poison => self.poison_turns <= 1,
            Spell::Recharge => self.recharge_turns <= 1,
            _ => true,
        }
    }

    fn apply_effects(&self) -> Self {
        let mut t = *self;
        if t.recharge_turns > 0 {
            t.my_mana += 101;
            t.recharge_turns -= 1;
        }
        t.my_armor = if t.shield_turns > 0 {
            t.shield_turns -= 1;
            7
        } else {
            0
        };
        if t.poison_turns > 0 {
            t.boss_health -= 3;
            t.poison_turns -= 1;
        }
        t
    }
    fn cast_spell(&self, sp: Spell) -> Self {
        let mut t = self.apply_effects();
        t.my_mana -= sp.mana_cost();
        if t.is_hard_mode {
            t.my_health -= 1;
        }
        match sp {
            Spell::MagicMissile => {
                t.boss_health -= 4;
            }
            Spell::Drain => {
                t.boss_health -= 2;
                t.my_health += 2;
            }
            Spell::Shield => {
                t.shield_turns = 6;
            }
            Spell::Poison => {
                t.poison_turns = 6;
            }
            Spell::Recharge => {
                t.recharge_turns = 5;
            }
        }
        t
    }
    fn is_loss(&self) -> bool {
        self.my_health <= 0 || self.my_mana <= 0
    }
    fn is_win(&self) -> bool {
        self.boss_health <= 0
    }
    fn boss_turn(&self) -> Self {
        let mut t = self.apply_effects();
        if t.boss_damage > t.my_armor {
            t.my_health -= t.boss_damage - t.my_armor;
        } else {
            t.my_health -= 1;
        }
        t
    }
    fn neighbours(&self) -> Vec<(State, isize)> {
        Spell::all()
            .iter()
            .filter_map(|&sp| {
                if self.effect_is_recastable(sp) {
                    let t = self.cast_spell(sp);
                    if t.is_loss() {
                        None
                    } else if t.is_win() {
                        Some((t, sp.mana_cost()))
                    } else {
                        let t2 = t.boss_turn();
                        if t2.is_loss() {
                            None
                        } else {
                            Some((t2, sp.mana_cost()))
                        }
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

fn gen(input: &str) -> State {
    let data = input
        .lines()
        .map(|x| {
            x.split(':')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<isize>()
                .unwrap()
        })
        .collect_vec();
    State {
        my_health: 50,
        boss_health: data[0],
        boss_damage: data[1],
        shield_turns: 0,
        poison_turns: 0,
        recharge_turns: 0,
        my_mana: 500,
        boss_armor: 0,
        my_armor: 0,
        is_hard_mode: false,
    }
}

fn p1(init: &State) -> isize {
    dijkstra(init, |s| s.neighbours(), |s| s.is_win())
        .unwrap()
        .1
}

fn p2(init: &State) -> isize {
    let mut hard = *init;
    hard.is_hard_mode = true;
    dijkstra(&hard, |s| s.neighbours(), |s| s.is_win())
        .unwrap()
        .1
}
//1408 too high
//1295 too high
