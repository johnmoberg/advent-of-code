use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let n = houses_visited(&input);
    let robo_n = robo_houses_visited(&input);
    println!("Visited {} unique houses", n);
    println!("Visited {} unique houses with the robot", robo_n);
}

fn houses_visited(input: &String) -> usize {
    let mut pos = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(pos);

    for c in input.chars() {
        let d = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, 1),
            'v' => (0, -1),
            _ => (0, 0),
        };
        pos.0 += d.0;
        pos.1 += d.1;
        visited.insert(pos);
    }

    visited.len()
}

fn robo_houses_visited(input: &String) -> usize {
    let mut pos = [(0, 0), (0, 0)];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for (i, c) in input.chars().enumerate() {
        let d = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, 1),
            'v' => (0, -1),
            _ => (0, 0),
        };
        pos[i % 2].0 += d.0;
        pos[i % 2].1 += d.1;
        visited.insert(pos[i % 2]);
    }

    visited.len()
}

#[test]
fn test_houses_visited() {
    assert_eq!(houses_visited(&String::from(">")), 2);
    assert_eq!(houses_visited(&String::from("^>v<")), 4);
    assert_eq!(houses_visited(&String::from("^v^v^v^v^v")), 2);
}

#[test]
fn test_robo_houses_visited() {
    assert_eq!(robo_houses_visited(&String::from(">v")), 3);
    assert_eq!(robo_houses_visited(&String::from("^>v<")), 3);
    assert_eq!(robo_houses_visited(&String::from("^v^v^v^v^v")), 11);
}
