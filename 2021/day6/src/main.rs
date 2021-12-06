use std::fs;

fn simulate_fish(input: &Vec<i8>, n_days: i32) -> i64 {
    let input = input.clone();

    let mut lanternfish: [i64; 9] = [0; 9];
    for life in input.iter() {
        lanternfish[*life as usize] += 1;
    }

    for _ in 0..n_days {
        let zeroes = lanternfish[0];
        
        // Every fish moves forward a step.
        for i in 0..8 {
            lanternfish[i] = lanternfish[i+1];
        }

        // The ones at zero are reborn and reproduce.
        lanternfish[6] += zeroes;
        lanternfish[8] = zeroes;
    }

    lanternfish.iter().sum()
}

fn main() {
    let contents = fs::read_to_string("../input/day6.txt").expect("Unable to read file");
    let input: Vec<i8> = contents.split(",").map(|x| x.parse::<i8>().unwrap()).collect();

    println!("Day 6, part 1: {}", simulate_fish(&input, 80));
    println!("Day 6, part 1: {}", simulate_fish(&input, 256));
}
