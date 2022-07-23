use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Sue (\d+)?: (.+)?: (\d+), (.+)?: (\d+), (.+)?: (\d+)").unwrap();
    }

    'outer: for line in lines {
        let caps = RE.captures(&line).unwrap();
        let id = caps[1].parse::<u64>().unwrap();

        for i in 1..=3 {
            let t1 = caps[2 * i].parse::<String>().unwrap();
            let q1 = caps[2 * i + 1].parse::<u64>().unwrap();

            let could_be = match &t1[..] {
                "children" => q1 == 3,
                "cats" => q1 > 7,
                "samoyeds" => q1 == 2,
                "pomeranians" => q1 < 3,
                "akitas" => q1 == 0,
                "vizslas" => q1 == 0,
                "goldfish" => q1 < 5,
                "trees" => q1 > 3,
                "cars" => q1 == 2,
                "perfumes" => q1 == 1,
                _ => panic!(),
            };
            if !could_be {
                continue 'outer;
            }
        }

        println!("Hey, it could be Aunt {}!", id);
    }
}
