use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    /* Would be more efficient with a 2D array, but whatever */
    let mut on: HashSet<(i64, i64)> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                on.insert((x as i64, y as i64));
            }
        }
    }

    for _ in 0..100 {
        let tmp_on = on.clone();

        for i in 0..100 {
            for j in 0..100 {
                let neighbours = [
                    (i - 1, j - 1),
                    (i, j - 1),
                    (i + 1, j - 1),
                    (i - 1, j),
                    (i + 1, j),
                    (i - 1, j + 1),
                    (i, j + 1),
                    (i + 1, j + 1),
                ];
                let on_neighbours = neighbours
                    .iter()
                    .fold(0, |n_on, p| n_on + tmp_on.contains(p) as u64);

                if tmp_on.contains(&(i, j)) && (on_neighbours < 2 || on_neighbours > 3) {
                    on.remove(&(i, j));
                } else if !tmp_on.contains(&(i, j)) && on_neighbours == 3 {
                    on.insert((i, j));
                }
            }
        }
    }

    println!("Part 1: {}", on.len())
}
