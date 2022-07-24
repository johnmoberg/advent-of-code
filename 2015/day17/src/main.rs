use itertools::Itertools;

fn main() {
    let target = 150u64;
    let containers: Vec<u64> = vec![
        43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38,
    ];

    let mut n_combinations = 0;

    for n_containers in 1..=containers.len() {
        for combination in containers.iter().combinations(n_containers) {
            if combination.iter().copied().sum::<u64>() == target {
                n_combinations += 1;
            }
        }
    }

    println!("Part 1: {}", n_combinations);
}
