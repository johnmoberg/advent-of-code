use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.json").unwrap();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }

    let s = RE
        .find_iter(&input)
        .fold(0, |s, digits| s + digits.as_str().parse::<i32>().unwrap());

    println!("Day 12, part 1: {}", s);
}
