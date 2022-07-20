use regex::Captures;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

/*
Quite hacky solution and clearly not the best (we should probably build a DAG), but works :-)
*/

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    let assign_re = Regex::new(r"(.+)? -> (.+)").unwrap();
    let and_re = Regex::new(r"(.+)? AND (.+)? -> (.+)").unwrap();
    let or_re = Regex::new(r"(.+)? OR (.+)? -> (.+)").unwrap();
    let not_re = Regex::new(r"NOT (.+)? -> (.+)").unwrap();
    let rshift_re = Regex::new(r"(.+)? RSHIFT (.+)? -> (.+)").unwrap();
    let lshift_re = Regex::new(r"(.+)? LSHIFT (.+)? -> (.+)").unwrap();

    let mut map = HashMap::new();

    while let None = get_value(&String::from("a"), &map) {
        for s in &lines {
            if let Some(s) = and_re.captures(&s) {
                let (a, b, c) = parse_three(&s);
                let mb_a_val = get_value(&a, &map);
                let mb_b_val = get_value(&b, &map);

                if let (Some(a_val), Some(b_val)) = (mb_a_val, mb_b_val) {
                    map.insert(c, a_val & b_val);
                }
            } else if let Some(s) = or_re.captures(&s) {
                let (a, b, c) = parse_three(&s);
                let mb_a_val = get_value(&a, &map);
                let mb_b_val = get_value(&b, &map);

                if let (Some(a_val), Some(b_val)) = (mb_a_val, mb_b_val) {
                    map.insert(c, a_val | b_val);
                }
            } else if let Some(s) = not_re.captures(&s) {
                let (a, b) = parse_two(&s);
                let mb_a_val = get_value(&a, &map);

                if let Some(a_val) = mb_a_val {
                    map.insert(b, !a_val);
                }
            } else if let Some(s) = rshift_re.captures(&s) {
                let (a, b, c) = parse_three(&s);
                let mb_a_val = get_value(&a, &map);
                let mb_b_val = get_value(&b, &map);

                if let (Some(a_val), Some(b_val)) = (mb_a_val, mb_b_val) {
                    map.insert(c, a_val >> b_val);
                }
            } else if let Some(s) = lshift_re.captures(&s) {
                let (a, b, c) = parse_three(&s);
                let mb_a_val = get_value(&a, &map);
                let mb_b_val = get_value(&b, &map);

                if let (Some(a_val), Some(b_val)) = (mb_a_val, mb_b_val) {
                    map.insert(c, a_val << b_val);
                }
            } else if let Some(s) = assign_re.captures(&s) {
                let (a, b) = parse_two(&s);
                let mb_a_val = get_value(&a, &map);

                if let Some(a_val) = mb_a_val {
                    map.insert(b, a_val);
                }
            } else {
                panic!();
            }
        }
    }

    let a = get_value(&String::from("a"), &map).ok_or("oh no").unwrap();
    println!("a = {}", a);
}

fn parse_two(s: &Captures) -> (String, String) {
    /* Hacky but hey..... */
    let a = s[1].parse::<String>().unwrap();
    let b = s[2].parse::<String>().unwrap();
    (a, b)
}

fn parse_three(s: &Captures) -> (String, String, String) {
    let a = s[1].parse::<String>().unwrap();
    let b = s[2].parse::<String>().unwrap();
    let c = s[3].parse::<String>().unwrap();
    (a, b, c)
}

fn get_value(s: &String, map: &HashMap<String, i64>) -> Option<i64> {
    let maybe_int = s.parse::<i64>();
    if maybe_int.is_ok() {
        return Some(maybe_int.unwrap());
    }

    // Part 2 stuff 🤷‍♂️
    if s == "b" {
        return Some(956);
    }

    match map.get(s) {
        Some(i) => Some(*i),
        None => None,
    }
}
