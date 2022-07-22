use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.json").unwrap();
    println!("Day 12, part 1: {}", part1(&input));
    println!("Day 12, part 2: {}", part2(&input));
}

fn part1(input: &String) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }

    RE.find_iter(&input)
        .fold(0, |s, digits| s + digits.as_str().parse::<i32>().unwrap())
}

fn part2(input: &String) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r":red").unwrap();
    }
    let mut clean_input = input.clone();

    while RE.is_match(&clean_input) {
        let m = RE.find(&clean_input).unwrap();
        let (i, j) = (m.start(), m.end());

        let before = String::from(&clean_input[..i]);
        let mut n_open = 1;
        let mut n_to_skip = 0;
        for c in before.chars().rev() {
            n_open += match c {
                '{' => -1,
                '}' => 1,
                _ => 0,
            };
            n_to_skip += 1;
            if n_open == 0 {
                break;
            }
        }
        let mut before_string = before
            .chars()
            .take(before.len() - n_to_skip)
            .collect::<String>();

        let after = String::from(&clean_input[j..]);
        let mut n_open = 1;
        let mut n_to_skip = 0;
        for c in after.chars() {
            n_open += match c {
                '{' => 1,
                '}' => -1,
                _ => 0,
            };
            n_to_skip += 1;
            if n_open == 0 {
                break;
            }
        }
        let after_string = after.chars().skip(n_to_skip).collect::<String>();
        before_string.push_str(&after_string);
        clean_input = before_string;
    }

    part1(&clean_input)
}

#[test]
fn test_part2() {
    assert_eq!(part2(&String::from("[1,{\"c\":red,\"b\":2},3]")), 4);
    assert_eq!(
        part2(&String::from("{\"d\":red,\"e\":[1,2,3,4],\"f\":5}")),
        0
    );
    assert_eq!(part2(&String::from("[1,red,5]")), 6);
}
