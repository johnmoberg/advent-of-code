use std::cmp;

#[derive(Debug)]
struct Ingredient(i64, i64, i64, i64, i64);

const N: i64 = 100;

fn main() {
    let cookies = vec![
        Ingredient(4, -2, 0, 0, 5),
        Ingredient(0, 5, -1, 0, 8),
        Ingredient(-1, 0, 5, 0, 6),
        Ingredient(0, 0, -2, 2, 1),
    ];

    let mut max_score = 0;
    for i in 0..N {
        for j in 0..N - i {
            for k in 0..N - i - j {
                let h = 100 - i - j - k;

                let cal = i * cookies[0].4 + j * cookies[1].4 + k * cookies[2].4 + h * cookies[3].4;
                if cal != 500 {
                    continue;
                }

                let cap = i * cookies[0].0 + j * cookies[1].0 + k * cookies[2].0 + h * cookies[3].0;
                let dur = i * cookies[0].1 + j * cookies[1].1 + k * cookies[2].1 + h * cookies[3].1;
                let fla = i * cookies[0].2 + j * cookies[1].2 + k * cookies[2].2 + h * cookies[3].2;
                let tex = i * cookies[0].3 + j * cookies[1].3 + k * cookies[2].3 + h * cookies[3].3;

                let score =
                    cmp::max(cap, 0) * cmp::max(dur, 0) * cmp::max(fla, 0) * cmp::max(tex, 0);
                max_score = cmp::max(max_score, score);
            }
        }
    }

    println!("Part 1: {}", max_score);
}
