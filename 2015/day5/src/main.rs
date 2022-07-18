use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let n_nice = reader
        .lines()
        .fold(0, |n, s| n + (nice_string_2(&s.unwrap()[..]) as i64));
    println!("Got {} nice strings", n_nice);
}

fn nice_string(input: &str) -> bool {
    let vec_input = input.chars().collect::<Vec<char>>();
    let vowels = vec_input
        .iter()
        .fold(0, |cnt, ch| cnt + VOWELS.contains(ch) as u64);

    let mut twice_in_a_row = false;

    for slice in vec_input.windows(2) {
        let (a, b) = (slice[0], slice[1]);

        twice_in_a_row = a == b || twice_in_a_row;

        let s = [a, b].iter().collect::<String>();
        if vec![
            String::from("ab"),
            String::from("cd"),
            String::from("pq"),
            String::from("xy"),
        ]
        .contains(&s)
        {
            return false;
        }
    }

    vowels >= 3 && twice_in_a_row
}

fn nice_string_2(input: &str) -> bool {
    let vec_input = input.chars().collect::<Vec<char>>();

    let mut map = HashMap::new();
    let mut maybe_last_s: Option<String> = None;

    for slice in vec_input.windows(2) {
        let (a, b) = (slice[0], slice[1]);
        let s = [a, b].iter().collect::<String>();

        if let Some(last_s) = maybe_last_s {
            if last_s == s {
                maybe_last_s = None;
                continue;
            } else {
                maybe_last_s = Some(s.clone());
                *map.entry(s).or_insert(0) += 1;
            }
        } else {
            maybe_last_s = Some(s.clone());
            *map.entry(s).or_insert(0) += 1;
        }
    }

    let pairs = map.into_iter().any(|(_, v)| v >= 2);
    let repeat = vec_input.windows(3).any(|s| s[0] == s[2]);

    pairs && repeat
}

#[test]
fn test_nice_string() {
    assert_eq!(nice_string("ugknbfddgicrmopn"), true);
    assert_eq!(nice_string("aaa"), true);
    assert_eq!(nice_string("jchzalrnumimnmhp"), false);
    assert_eq!(nice_string("haegwjzuvuyypxyu"), false);
    assert_eq!(nice_string("dvszwmarrgswjxmb"), false);
}

#[test]
fn test_nice_string_2() {
    assert_eq!(nice_string_2("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(nice_string_2("xxyxx"), true);
    assert_eq!(nice_string_2("uurcxstgmygtbstg"), false);
    assert_eq!(nice_string_2("ieodomkazucvgmuy"), false);
}
