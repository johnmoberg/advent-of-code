use std::fs;

use day13::{parse_input, part1};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let (mut people, mut map) = parse_input(&lines);

    let answer = part1(&people, &map);
    println!("Part 1: {}", answer);

    for person in &people {
        map.insert((String::from("John"), person.clone()), 0);
        map.insert((person.clone(), String::from("John")), 0);
    }
    people.push(String::from("John"));
    let answer = part1(&people, &map);
    println!("Part 2: {}", answer);
}
