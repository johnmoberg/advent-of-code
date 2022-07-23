use std::cmp;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    velocity: u64,
    run_time: u64,
    rest_time: u64,
}

enum ReindeerState {
    Moving,
    Resting,
}

struct MovingReindeer<'a> {
    reindeer: &'a Reindeer,
    state: ReindeerState,
    time_left_in_state: u64,
    current_position: u64,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").map(String::from).collect::<Vec<String>>();
    let reindeers = parse_input(&lines);
    let answer = distance_after_seconds(&reindeers, 2503);
    println!("Part 1: {}", answer);
}

fn distance_after_seconds(reindeers: &Vec<Reindeer>, time: u64) -> u64 {
    let mut v = Vec::new();
    for reindeer in reindeers {
        let r = MovingReindeer {
            reindeer: &reindeer,
            state: ReindeerState::Moving,
            time_left_in_state: reindeer.run_time,
            current_position: 0,
        };
        v.push(r);
    }

    for _ in 0..time {
        for r in v.iter_mut() {
            r.current_position += match r.state {
                ReindeerState::Moving => r.reindeer.velocity,
                ReindeerState::Resting => 0,
            };
            r.time_left_in_state -= 1;
            if r.time_left_in_state == 0 {
                r.state = match r.state {
                    ReindeerState::Moving => ReindeerState::Resting,
                    ReindeerState::Resting => ReindeerState::Moving,
                };
                r.time_left_in_state = match r.state {
                    ReindeerState::Moving => r.reindeer.run_time,
                    ReindeerState::Resting => r.reindeer.rest_time,
                }
            }
        }
    }

    v.iter()
        .fold(u64::MIN, |m, r| cmp::max(m, r.current_position))
}

#[test]
fn test_distance_after_seconds() {
    let reindeers = vec![
        Reindeer {
            name: String::from("Comet"),
            velocity: 14,
            run_time: 10,
            rest_time: 127,
        },
        Reindeer {
            name: String::from("Dancer"),
            velocity: 16,
            run_time: 11,
            rest_time: 162,
        },
    ];
    let d = distance_after_seconds(&reindeers, 1000);
    assert_eq!(d, 1120);
}

fn parse_input<'a>(lines: &'a Vec<String>) -> Vec<Reindeer> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"([a-zA-Z]+)? can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds."
        )
        .unwrap();
    }

    let mut reindeers = Vec::new();

    for line in lines {
        let caps = RE.captures(&line).unwrap();

        let reindeer = Reindeer {
            name: caps[1].parse::<String>().unwrap(),
            velocity: caps[2].parse::<u64>().unwrap(),
            run_time: caps[3].parse::<u64>().unwrap(),
            rest_time: caps[4].parse::<u64>().unwrap(),
        };
        reindeers.push(reindeer);
    }

    reindeers
}

#[test]
fn test_parse_input() {
    let lines = vec![
        String::from("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
        String::from("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."),
    ];
    let output = parse_input(&lines);
    let want = vec![
        Reindeer {
            name: String::from("Comet"),
            velocity: 14,
            run_time: 10,
            rest_time: 127,
        },
        Reindeer {
            name: String::from("Dancer"),
            velocity: 16,
            run_time: 11,
            rest_time: 162,
        },
    ];
    assert_eq!(output, want);
}
