use std::collections::HashMap;

fn main() {
    let input = 34000000;
    println!("Part 1: {}", lowest_house_no(input));
    println!("Part 2: {}", lazy_elves(input));
}

fn lowest_house_no(presents: u64) -> u64 {
    (1..)
        .find(|house_no| num_presents(*house_no) >= presents)
        .unwrap()
}

fn num_presents(n: u64) -> u64 {
    10 + 10 * n
        + divisors::get_divisors(n)
            .iter()
            .map(|elf_no| 10 * elf_no)
            .sum::<u64>()
}

fn lazy_elves(presents: u64) -> u64 {
    let mut num_delivered = HashMap::new();

    for house_no in 1.. {
        let mut num_presents = 0;

        // The first elf will always only deliver to the first 50 houses.
        if house_no <= 50 {
            num_presents += 11;
        }

        // The elf with number = house_no will always deliver to the house with that number.
        num_presents += 11 * house_no;
        num_delivered.insert(house_no, 1);

        for elf_no in divisors::get_divisors(house_no) {
            let elf_delivered = *num_delivered.get(&elf_no).unwrap();

            if elf_delivered < 50 {
                num_presents += 11 * elf_no;
                num_delivered.insert(elf_no, elf_delivered + 1);
            }
        }

        if num_presents >= presents {
            return house_no;
        }
    }

    unreachable!()
}

#[test]
fn test_lowest_house_no() {
    assert_eq!(lowest_house_no(30), 2);
    assert_eq!(lowest_house_no(70), 4);
    assert_eq!(lowest_house_no(60), 4);
    assert_eq!(lowest_house_no(80), 6);
    assert_eq!(lowest_house_no(130), 8);
}

/*
Elf 3 will deliver to 3, 6, 9, 12, 15, ..., until it has delivered to 50 houses
Elf 11 will deliver to 11, 22, 33, ..., until it has delivered to 50 houses
*/
