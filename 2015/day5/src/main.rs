use std::fs::File;
use std::io::{BufRead, BufReader};

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let n_nice = reader
        .lines()
        .fold(0, |n, s| n + (nice_string(&s.unwrap()[..]) as i64));
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

    return vowels >= 3 && twice_in_a_row;
}

#[test]
fn test_nice_string() {
    assert_eq!(nice_string("ugknbfddgicrmopn"), true);
    assert_eq!(nice_string("aaa"), true);
    assert_eq!(nice_string("jchzalrnumimnmhp"), false);
    assert_eq!(nice_string("haegwjzuvuyypxyu"), false);
    assert_eq!(nice_string("dvszwmarrgswjxmb"), false);
}
