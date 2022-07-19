use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut state = vec![vec![false; 1000]; 1000];
    for line in reader.lines() {
        let instruction = parse_instruction(line.unwrap());
        let p1: usize;
        let p2: usize;
        let p3: usize;
        let p4: usize;

        let f = match instruction {
            Instruction::TurnOn { x1, x2, x3, x4 } => {
                (p1, p2, p3, p4) = (x1, x2, x3, x4);
                |_: bool| true
            }
            Instruction::TurnOff { x1, x2, x3, x4 } => {
                (p1, p2, p3, p4) = (x1, x2, x3, x4);
                |_: bool| false
            }
            Instruction::Toggle { x1, x2, x3, x4 } => {
                (p1, p2, p3, p4) = (x1, x2, x3, x4);
                |v: bool| !v
            }
        };

        for i in p1..=p3 {
            for j in p2..=p4 {
                state[i][j] = f(state[i][j]);
            }
        }
    }

    let turned_on: usize = state.into_iter().flatten().filter(|b| *b).count();
    println!("Got {} turned on lights", turned_on);
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut state = vec![vec![0; 1000]; 1000];
    for line in reader.lines() {
        let instruction = parse_instruction(line.unwrap());
        let p1: usize;
        let p2: usize;
        let p3: usize;
        let p4: usize;

        let v: i8 = match instruction {
            Instruction::TurnOn { x1, x2, x3, x4 } => {
                (p1, p2, p3, p4) = (x1, x2, x3, x4);
                1
            }
            Instruction::TurnOff { x1, x2, x3, x4 } => {
                (p1, p2, p3, p4) = (x1, x2, x3, x4);
                -1
            }
            Instruction::Toggle { x1, x2, x3, x4 } => {
                (p1, p2, p3, p4) = (x1, x2, x3, x4);
                2
            }
        };

        for i in p1..=p3 {
            for j in p2..=p4 {
                state[i][j] = cmp::max(0, state[i][j] + v as i64);
            }
        }
    }

    let turned_on: i64 = state.into_iter().flatten().sum();
    println!("Got {} total brightness", turned_on);
}

#[derive(PartialEq, Debug)]
enum Instruction {
    TurnOn {
        x1: usize,
        x2: usize,
        x3: usize,
        x4: usize,
    },
    TurnOff {
        x1: usize,
        x2: usize,
        x3: usize,
        x4: usize,
    },
    Toggle {
        x1: usize,
        x2: usize,
        x3: usize,
        x4: usize,
    },
}

fn parse_instruction(s: String) -> Instruction {
    let re = Regex::new(r"(.+)? (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let caps = re.captures(&s).unwrap();

    let x1 = caps[2].parse::<usize>().unwrap();
    let x2 = caps[3].parse::<usize>().unwrap();
    let x3 = caps[4].parse::<usize>().unwrap();
    let x4 = caps[5].parse::<usize>().unwrap();

    assert!(x1 <= x3);
    assert!(x2 <= x4);

    match &caps[1] {
        "turn on" => Instruction::TurnOn { x1, x2, x3, x4 },
        "turn off" => Instruction::TurnOff { x1, x2, x3, x4 },
        "toggle" => Instruction::Toggle { x1, x2, x3, x4 },
        _ => panic!(),
    }
}

#[test]
fn test_parse_instruction_1() {
    let s = String::from("turn on 0,0 through 999,999");
    let want = Instruction::TurnOn {
        x1: 0,
        x2: 0,
        x3: 999,
        x4: 999,
    };

    assert_eq!(parse_instruction(s), want);
}

#[test]
fn test_parse_instruction_2() {
    let s = String::from("toggle 0,0 through 999,0");
    let want = Instruction::Toggle {
        x1: 0,
        x2: 0,
        x3: 999,
        x4: 0,
    };

    assert_eq!(parse_instruction(s), want);
}

#[test]
fn test_parse_instruction_3() {
    let s = String::from("turn off 343,232 through 444,998");
    let want = Instruction::TurnOff {
        x1: 343,
        x2: 232,
        x3: 444,
        x4: 998,
    };

    assert_eq!(parse_instruction(s), want);
}
