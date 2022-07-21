use std::collections::HashSet;

fn main() {
    let input = String::from("hepxcrrq");
    let output = next_password(&input);
    println!("Part 1: {}", output);

    let input = String::from("hepxxzaa");
    let output = next_password(&input);
    println!("Part 2: {}", output);
}

fn next_password(password: &String) -> String {
    let mut v = password.chars().collect::<Vec<char>>();
    let n = v.len();

    while !is_valid(&(v.iter().collect())) {
        let mut i = n - 1;
        loop {
            if v[i] == 'z' {
                v[i] = 'a';
                i -= 1;
            } else {
                v[i] = char::from_u32(v[i] as u32 + 1).unwrap();
                break;
            }
        }
    }

    v.iter().collect()
}

fn is_valid(password: &String) -> bool {
    first_req(password) && second_req(password) && third_req(password)
}

fn first_req(password: &String) -> bool {
    let v = password.chars().collect::<Vec<char>>();

    for slice in v.windows(3) {
        let a = slice[0] as u32;
        let b = slice[1] as u32;
        let c = slice[2] as u32;

        if c - b == 1 && b - a == 1 {
            return true;
        }
    }

    false
}

fn second_req(password: &String) -> bool {
    !password.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
}

fn third_req(password: &String) -> bool {
    let mut set = HashSet::new();
    let v = password.chars().collect::<Vec<char>>();

    for slice in v.windows(2) {
        if slice[0] == slice[1] {
            set.insert(slice[0]);
        }
    }

    set.len() >= 2
}

#[test]
fn test_next_password() {
    assert_eq!(
        next_password(&String::from("abcdefgh")),
        String::from("abcdffaa")
    );
    assert_eq!(
        next_password(&String::from("ghijklmn")),
        String::from("ghjaabcc")
    );
}

#[test]
fn test_first_req() {
    assert_eq!(first_req(&String::from("hijklmmn")), true);
    assert_eq!(first_req(&String::from("abbceffg")), false);
}

#[test]
fn test_second_req() {
    assert_eq!(second_req(&String::from("hijklmmn")), false);
}

#[test]
fn test_third_req() {
    assert_eq!(third_req(&String::from("abbceffg")), true);
    assert_eq!(third_req(&String::from("abbcegjk")), false);
}
