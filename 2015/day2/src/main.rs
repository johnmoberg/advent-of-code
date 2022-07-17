use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let total = reader
        .lines()
        .fold(0, |sum, val| sum + required_wrapping(val.unwrap()));
    println!("Need {} wrapping", total);
}

fn required_wrapping(dimensions: String) -> u64 {
    let dims = dimensions
        .split('x')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    if let [l, w, h] = dims[..] {
        let slack = *vec![l * w, w * h, h * l].iter().min().unwrap();
        2 * (l * w + w * h + h * l) + slack
    } else {
        panic!("Unexpected input")
    }
}

#[test]
fn test_get_floor() {
    assert_eq!(required_wrapping(String::from("2x3x4")), 58);
    assert_eq!(required_wrapping(String::from("1x1x10")), 43);
}
