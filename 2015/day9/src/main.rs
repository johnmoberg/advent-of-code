use std::fs;
use std::cmp;
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+)? to (.+)? = (\d+)").unwrap();
    }

    let mut map: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in lines {
        let caps = RE.captures(&line).unwrap();

        let city_a = caps[1].parse::<String>().unwrap();
        let city_b = caps[2].parse::<String>().unwrap();
        let distance = caps[3].parse::<usize>().unwrap();

        map.entry(city_a).or_insert(HashMap::new()).insert(city_b, distance);
    }

    let mut shortest_route = usize::MAX;

    for p in map.keys().permutations(map.len()) {
        let mut distance = 0;
        for slice in p.windows(2) {
            println!("{} {}", slice[0], slice[1]);
            distance += map.get(slice[0]).unwrap().get(slice[1]).unwrap();
        }

        shortest_route = cmp::min(shortest_route, distance);
    }

    println!("Found shortest route {}", shortest_route);
}
