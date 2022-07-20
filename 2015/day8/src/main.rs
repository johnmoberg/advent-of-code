use lazy_static::lazy_static;
use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(strings: &Vec<&str>) -> usize {
    strings.iter().fold(0, |sum, line| {
        let (code_len, deescaped_len) = char_count(line);
        sum + (code_len - deescaped_len)
    })
}

fn part2(strings: &Vec<&str>) -> usize {
    strings.iter().fold(0, |sum, line| {
        let (code_len, escaped_len) = char_count_2(line);
        sum + (escaped_len - code_len)
    })
}

fn char_count(s: &str) -> (usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\\x([0-9a-f]){2}").unwrap();
    }

    let deescaped = RE.replace_all(&s, "a");
    let deescaped = deescaped.replace("\\\"", "b");
    let deescaped = deescaped.replace("\\\\", "c");

    (s.len(), deescaped.len() - 2)
}

fn char_count_2(s: &str) -> (usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\\x([0-9a-f]){2}").unwrap();
    }

    let escaped = RE.replace_all(&s, "aaaaa");
    let escaped = escaped.replace("\"", "aaa");
    let escaped = escaped.replace("\\\"", "aaaa");
    let escaped = escaped.replace("\\\\", "aaaa");

    (s.len(), escaped.len())
}

#[test]
fn test_char_count() {
    assert_eq!(char_count("\"\""), (2, 0));
    assert_eq!(char_count("\"abc\""), (5, 3));
    assert_eq!(char_count("\"aaa\\\"aaa\""), (10, 7));
    assert_eq!(char_count("\"\\x27\""), (6, 1));
}

#[test]
fn test_char_count_2() {
    assert_eq!(char_count_2("\"\""), (2, 6));
    assert_eq!(char_count_2("\"abc\""), (5, 9));
    assert_eq!(char_count_2("\"aaa\\\"aaa\""), (10, 16));
    assert_eq!(char_count_2("\"\\x27\""), (6, 11));
}

#[test]
fn test_part1() {
    let v = vec!["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""];
    assert_eq!(part1(&v), 12);
}

#[test]
fn test_part2() {
    let v = vec!["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""];
    assert_eq!(part2(&v), 19);
}
