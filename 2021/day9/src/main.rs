use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Unable to read file");
    let risk_sum = compute_risk_sum(&contents);
    println!("Day 9, part 1: {}", risk_sum);
}

fn compute_risk_sum(input: &str) -> i32 {
    let mut rows: Vec<Vec<i8>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<i8> = Vec::new();

        for c in line.chars() {
            row.push(c as i8 - '0' as i8);
        }
        rows.push(row);
    };

    let mut sum: i32 = 0;
    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            let cur = rows[i][j];

            if i > 0 && cur >= rows[i-1][j] {
                continue;
            }
            if j > 0 && cur >= rows[i][j-1] {
                continue;
            }
            if i < rows.len() - 1 && cur >= rows[i+1][j] {
                continue;
            }
            if j < rows[i].len() - 1 && cur >= rows[i][j+1] {
                continue;
            }
            sum += cur as i32 + 1;
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::compute_risk_sum;

    #[test]
    fn small_case() {
        let test_input: String = String::from("2199943210
3987894921
9856789892
8767896789
9899965678");
        let risk_sum = compute_risk_sum(&test_input);
        assert_eq!(risk_sum, 15);
    }
}