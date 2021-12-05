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
