use std::collections::HashSet;
use std::fs;

fn main() {
    let n = houses_visited(fs::read_to_string("input.txt").unwrap());
    println!("Visited {} unique houses", n);
}

fn houses_visited(input: String) -> usize {
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

#[test]
fn test_houses_visited() {
    assert_eq!(houses_visited(String::from(">")), 2);
    assert_eq!(houses_visited(String::from("^>v<")), 4);
    assert_eq!(houses_visited(String::from("^v^v^v^v^v")), 2);
}
