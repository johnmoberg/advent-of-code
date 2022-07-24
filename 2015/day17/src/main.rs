use itertools::Itertools;
use std::cmp;

fn main() {
    let containers: Vec<u64> = vec![
        43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38,
    ];
    let target = 150u64;

    let (min_containers, max_containers) = (1, containers.len() as u64);
    let (n_combinations, min_needed) = part1(target, min_containers, max_containers, &containers);
    println!("Part 1: {}", n_combinations);

    let (n_combinations, _) = part1(target, min_needed, min_needed, &containers);
    println!("Part 2: {}", n_combinations);
}

fn part1(
    target: u64,
    min_containers: u64,
    max_containers: u64,
    containers: &Vec<u64>,
) -> (u64, u64) {
    let mut n_combinations = 0;
    let mut min_needed = u64::MAX;

    for n_containers in min_containers..=max_containers {
        for combination in containers
            .iter()
            .combinations(n_containers.try_into().unwrap())
        {
            if combination.iter().copied().sum::<u64>() == target {
                n_combinations += 1;
                min_needed = cmp::min(min_needed, n_containers);
            }
        }
    }

    (n_combinations, min_needed)
}
