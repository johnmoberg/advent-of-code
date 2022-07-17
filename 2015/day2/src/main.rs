use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let total = reader.lines().fold((0, 0), |sum, val| {
        let v = val.unwrap();
        (sum.0 + required_wrapping(&v), sum.1 + required_ribbon(&v))
    });
    println!("Need {} wrapping and {} ribbon", total.0, total.1);
}

fn dimensions(line: &String) -> Option<(u64, u64, u64)> {
    let dims = line
        .split('x')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    if let [l, w, h] = dims[..] {
        Some((l, w, h))
    } else {
        None
    }
}

fn required_wrapping(line: &String) -> u64 {
    let (l, w, h) = dimensions(line).unwrap();
    let slack = *vec![l * w, w * h, h * l].iter().min().unwrap();

    2 * (l * w + w * h + h * l) + slack
}

fn required_ribbon(line: &String) -> u64 {
    let (l, w, h) = dimensions(line).unwrap();
    let min_perimeter = *vec![2 * (l + w), 2 * (l + h), 2 * (w + h)]
        .iter()
        .min()
        .unwrap();
    let volume = l * w * h;

    min_perimeter + volume
}

#[test]
fn test_p1() {
    assert_eq!(required_wrapping(&String::from("2x3x4")), 58);
    assert_eq!(required_wrapping(&String::from("1x1x10")), 43);
}

#[test]
fn test_p2() {
    assert_eq!(required_ribbon(&String::from("2x3x4")), 34);
    assert_eq!(required_ribbon(&String::from("1x1x10")), 14);
}
