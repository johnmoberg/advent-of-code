fn main() {
    let input = String::from("1113222113");
    let output = (0..40).fold(input, |x, _| look_and_say(&x));
    println!("Part 1: {}", output.len());
    
    let input = String::from("1113222113");
    let output = (0..50).fold(input, |x, _| look_and_say(&x));
    println!("Part 2: {}", output.len());
}

fn look_and_say(s: &String) -> String {
    let mut output = String::from("");
    let mut last_char = s.chars().next().unwrap();
    let mut count = 1;

    for c in s.chars().skip(1) {
        if c == last_char {
            count += 1;
        } else {
            output.push_str(&format!("{}{}", count, last_char));
            (last_char, count) = (c, 1);
        }
    }

    if count > 0 {
        output.push_str(&format!("{}{}", count, last_char));
    }

    output
}

#[test]
fn test_look_and_say() {
    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("1211"), "111221");
    assert_eq!(look_and_say("111221"), "312211");
}
