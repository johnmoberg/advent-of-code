use std::fs;

fn main() {
    println!("Ended up on floor {}", floor(fs::read_to_string("input.txt").unwrap()));

}

fn floor(instructions: String) -> i64 {
    instructions.chars().enumerate().fold(0, |f, (i, p)| {
        if f == -1 {
            println!("In basement at position {}", i)
        }

        f + match p {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    })
}

#[test]
fn test_get_floor() {
    assert_eq!(floor(String::from("(())")), 0);
    assert_eq!(floor(String::from("(((")), 3);
    assert_eq!(floor(String::from("())")), -1);
    assert_eq!(floor(String::from(")())())")), -3);
}
