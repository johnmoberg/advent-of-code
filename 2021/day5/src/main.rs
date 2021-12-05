use std::collections::HashMap;
use std::fs;
use std::cmp;
use regex::Regex;

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    direction: (i32, i32),
    length: i32,
}

fn count_intersections(lines: &Vec<Line>, count_diagonal: bool) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines.iter() {
        let (mut x, mut y) = (line.start.0, line.start.1);
        let (dx, dy) = (line.direction.0, line.direction.1);

        if !count_diagonal && dx.abs() + dy.abs() > 1 {
            continue;
        };

        for _ in 0..=line.length {
            *map.entry((x, y)).or_insert(0) += 1;

            x += dx;
            y += dy;
        };
    };

    map.retain(|_, v| *v > 1);
    map.len() as i32
}

fn main() {
    let contents = fs::read_to_string("../input/day5.txt").expect("Unable to read file");
    let re = Regex::new(r"\d+").unwrap();

    let mut lines: Vec<Line> = Vec::new();
    for line in contents.lines() {
        let coords: Vec<i32> = re.find_iter(line).filter_map(|d| d.as_str().parse().ok()).collect();
        let (x1, y1, x2, y2) = (coords[0], coords[1], coords[2], coords[3]);

        lines.push(Line {
            start: (x1, y1),
            direction: ((x2 - x1).signum(), (y2 - y1).signum()),
            length: cmp::max((x2 - x1).abs(), (y2 - y1).abs()),
        });
    };

    println!("Day 5, part 1: {}", count_intersections(&lines, false));
    println!("Day 5, part 2: {}", count_intersections(&lines, true));
}
