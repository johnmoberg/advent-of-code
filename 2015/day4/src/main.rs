use md5;

fn main() {
    let input = &String::from("bgvyzdsv");
    println!("Part 1: n = {}", lowest_n(input, 5));
    println!("Part 2: n = {}", lowest_n(input, 6));
}

fn lowest_n(key: &String, n_zeros: usize) -> u64 {
    let mut i = 0u64;
    loop {
        if i % 10000 == 0 {
            println!("Checked {} hashes", i);
        }

        let hash = format!("{:x}", md5::compute(key.to_owned() + &(i.to_string())[..]));

        if &hash[..n_zeros] == "0".repeat(n_zeros) {
            return i;
        }

        i += 1;
    }
}

#[test]
fn test_lowest_n() {
    assert_eq!(lowest_n(&String::from("abcdef"), 5), 609043);
    assert_eq!(lowest_n(&String::from("pqrstuv"), 5), 1048970);
}
