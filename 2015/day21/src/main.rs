use itertools::Itertools;
use std::cmp;

const PLAYER_HP: i32 = 100;

const BOSS_HP: i32 = 100;
const BOSS_DAMAGE: i32 = 8;
const BOSS_ARMOR: i32 = 2;

struct Stats {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Stats {
    pub fn from(hp: i32, items: &Vec<&Item>) -> Stats {
        let armor = items.iter().map(|item| item.armor).sum();
        let damage = items.iter().map(|item| item.damage).sum();

        Stats { hp, armor, damage }
    }
}

#[derive(Debug, Clone)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    pub fn new(cost: i32, damage: i32, armor: i32) -> Item {
        Item {
            cost,
            damage,
            armor,
        }
    }
}

fn main() {
    let weapons: [Item; 5] = [
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ];
    let armor: [Option<Item>; 6] = [
        Some(Item::new(13, 0, 1)),
        Some(Item::new(31, 0, 2)),
        Some(Item::new(53, 0, 3)),
        Some(Item::new(75, 0, 4)),
        Some(Item::new(102, 0, 5)),
        None,
    ];
    let rings: [Option<Item>; 8] = [
        Some(Item::new(25, 1, 0)),
        Some(Item::new(50, 2, 0)),
        Some(Item::new(100, 3, 0)),
        Some(Item::new(20, 0, 1)),
        Some(Item::new(40, 0, 2)),
        Some(Item::new(80, 0, 3)),
        None,
        None,
    ];

    let boss_stats = Stats {
        hp: BOSS_HP,
        damage: BOSS_DAMAGE,
        armor: BOSS_ARMOR,
    };

    let mut min_win_cost = std::i32::MAX;
    let mut max_loss_cost = std::i32::MIN;

    for (w, maybe_a) in weapons.iter().cartesian_product(armor.iter()) {
        for ring_slice in rings.iter().combinations(2) {
            let mut items = vec![w];
            if let Some(a) = maybe_a {
                items.push(a);
            }

            for maybe_ring in ring_slice {
                if let Some(ring) = maybe_ring {
                    items.push(ring);
                }
            }

            let player_stats = Stats::from(PLAYER_HP, &items);
            let items_cost = items.iter().map(|item| item.cost).sum();

            if player_wins(&player_stats, &boss_stats) {
                min_win_cost = cmp::min(min_win_cost, items_cost);
            } else {
                max_loss_cost = cmp::max(max_loss_cost, items_cost);
            }
        }
    }

    println!("Part 1: {}", min_win_cost);
    println!("Part 2: {}", max_loss_cost);
}

fn player_wins(player: &Stats, boss: &Stats) -> bool {
    let mut player_hp = player.hp;
    let mut boss_hp = boss.hp;

    while player_hp > 0 && boss_hp > 0 {
        boss_hp -= cmp::max(1, player.damage - boss.armor);
        player_hp -= cmp::max(1, boss.damage - player.armor);
    }

    player_hp > 0 || (player_hp <= 0 && boss_hp <= 0)
}

#[test]
fn test_player_wins() {
    assert_eq!(
        player_wins(
            &Stats {
                hp: 8,
                damage: 5,
                armor: 5
            },
            &Stats {
                hp: 12,
                damage: 7,
                armor: 2
            }
        ),
        true
    );
    assert_eq!(
        player_wins(
            &Stats {
                hp: 8,
                damage: 4,
                armor: 5
            },
            &Stats {
                hp: 12,
                damage: 7,
                armor: 2
            }
        ),
        false
    );
}
