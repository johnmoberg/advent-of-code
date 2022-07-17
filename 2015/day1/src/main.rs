use std::fs;

fn main() {
    println!("{}", get_floor(fs::read_to_string("input.txt").unwrap()));

}

fn get_floor(instructions: String) -> i64 {
    instructions.chars().fold(0, |f, p| {
        f + match p {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    })
}

#[test]
fn test_get_floor() {
    assert_eq!(get_floor(String::from("(())")), 0);
    assert_eq!(get_floor(String::from("(((")), 3);
    assert_eq!(get_floor(String::from("())")), -1);
    assert_eq!(get_floor(String::from(")())())")), -3);
}
