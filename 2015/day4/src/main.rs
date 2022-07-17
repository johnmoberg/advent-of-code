use md5;

fn main() {
    let input = &String::from("bgvyzdsv");
    println!("Part 1: n = {}", lowest_n(input));
}

fn lowest_n(key: &String) -> u64 {
    let mut i = 0u64;
    loop {
        let hash = format!("{:x}", md5::compute(key.to_owned() + &(i.to_string())[..]));

        if &hash[..5] == "00000" {
            return i;
        }

        i += 1;
    }
}

#[test]
fn test_lowest_n() {
    assert_eq!(lowest_n(&String::from("abcdef")), 609043);
    assert_eq!(lowest_n(&String::from("pqrstuv")), 1048970);
}
