use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;


pub fn part1(people: &Vec<String>, map: &HashMap<(String, String), i64>) -> i64 {
    let mut max_reward = i64::MIN;

    for p in people.iter().permutations(people.len()) {
        let mut p = p.clone();
        let first = String::from(*p.first().unwrap());
        p.push(&first);

        let mut reward = 0;

        for slice in p.windows(2) {
            let a = String::from(slice[0]);
            let b = String::from(slice[1]);
            reward += map[&(a.clone(), b.clone())];
            reward += map[&(b, a)];
        }
        max_reward = cmp::max(reward, max_reward);
    }

    max_reward
}

#[test]
fn test_part1() {
    let lines = vec![
        "Alice would gain 54 happiness units by sitting next to Bob.",
        "Alice would lose 79 happiness units by sitting next to Carol.",
        "Alice would lose 2 happiness units by sitting next to David.",
        "Bob would gain 83 happiness units by sitting next to Alice.",
        "Bob would lose 7 happiness units by sitting next to Carol.",
        "Bob would lose 63 happiness units by sitting next to David.",
        "Carol would lose 62 happiness units by sitting next to Alice.",
        "Carol would gain 60 happiness units by sitting next to Bob.",
        "Carol would gain 55 happiness units by sitting next to David.",
        "David would gain 46 happiness units by sitting next to Alice.",
        "David would lose 7 happiness units by sitting next to Bob.",
        "David would gain 41 happiness units by sitting next to Carol.",
    ];
    let (people, map) = parse_input(&lines);
    assert_eq!(part1(&people, &map), 330);
}


pub fn parse_input(lines: &Vec<&str>) -> (Vec<String>, HashMap<(String, String), i64>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"([a-zA-Z]+)? would (gain|lose) (\d+) happiness units by sitting next to ([a-zA-Z]+)?."
        )
        .unwrap();
    }

    let mut people = HashSet::new();
    let mut map = HashMap::new();

    for line in lines {
        let caps = RE.captures(&line).unwrap();

        let person_a = caps[1].parse::<String>().unwrap();
        let person_b = caps[4].parse::<String>().unwrap();
        let gain = match &caps[2] {
            "gain" => 1,
            "lose" => -1,
            _ => panic!(),
        };
        let points = gain * caps[3].parse::<i64>().unwrap();

        map.insert((person_a.clone(), person_b.clone()), points);
        people.insert(person_a);
        people.insert(person_b);
    }

    (people.into_iter().collect(), map)
}
