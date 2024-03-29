use std::cmp;
use std::collections::HashMap;
use std::fs;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

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

        map.entry(city_a.clone())
            .or_insert(HashMap::new())
            .insert(city_b.clone(), distance);
        map.entry(city_b.clone())
            .or_insert(HashMap::new())
            .insert(city_a.clone(), distance);
    }

    let mut shortest_route = usize::MAX;
    let mut longest_route = usize::MIN;

    'outer: for p in map.keys().permutations(map.len()) {
        let mut distance = 0;

        for slice in p.windows(2) {
            if let Some(d) = get_distance(&map, &slice[0], &slice[1]) {
                distance += d;
            } else {
                continue 'outer;
            }
        }
        shortest_route = cmp::min(shortest_route, distance);
        longest_route = cmp::max(longest_route, distance);
    }

    println!("Found shortest route with distance {}", shortest_route);
    println!("Found longest route with distance {}", longest_route);
}

fn get_distance<'a>(
    map: &'a HashMap<String, HashMap<String, usize>>,
    a: &'a String,
    b: &'a String,
) -> Option<&'a usize> {
    if let Some(h) = map.get(a) {
        h.get(b)
    } else {
        None
    }
}
