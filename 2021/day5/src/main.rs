use std::collections::HashMap;
use std::fs;
use std::cmp;

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    direction: (i32, i32),
    length: i32,
}

fn count_intersections(lines: &Vec<Line>, count_diagonal: bool) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    for line in lines.iter() {
        if !count_diagonal && line.direction.0.abs() + line.direction.1.abs() > 1 {
            continue;
        }

        let mut x = line.start.0;
        let mut y = line.start.1;

        for _ in 0..=line.length {
            let value = map.entry((x, y)).or_insert(0);
            *value += 1;

            x += line.direction.0;
            y += line.direction.1;
        };
    };

    let mut count = 0;
    for (_, value) in map.iter() {
        if *value > 1 {
            count += 1;
        };
    };

    return count;
}

fn main() {
    let contents = fs::read_to_string("../input/day5.txt").expect("Unable to read file");

    let mut lines: Vec<Line> = Vec::new();
    for line in contents.lines() {
        let mut parts = line.split(" -> ");
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();
        
        let start_parts: Vec<&str> = start.split(",").collect();
        let end_parts: Vec<&str> = end.split(",").collect();
        
        let x1 = start_parts[0].parse::<i32>().unwrap();
        let y1 = start_parts[1].parse::<i32>().unwrap();
        
        let x2 = end_parts[0].parse::<i32>().unwrap();
        let y2 = end_parts[1].parse::<i32>().unwrap();
        
        lines.push(Line {
            start: (x1, y1),
            direction: ((x2 - x1).signum(), (y2 - y1).signum()),
            length: cmp::max((x2 - x1).abs(), (y2 - y1).abs()),
        });
    };

    println!("Day 5, part 1: {}", count_intersections(&lines, false));
    println!("Day 5, part 2: {}", count_intersections(&lines, true));
}
