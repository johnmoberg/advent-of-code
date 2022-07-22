use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day13::{parse_input, part1};

fn criterion_benchmark(c: &mut Criterion) {
    let lines = vec![
        "Alice would gain 54 happiness units by sitting next to Bob.",
        "Alice would lose 79 happiness units by sitting next to Carol.",
        "Alice would lose 2 happiness units by sitting next to David.",
        "Bob would gain 83 happiness units by sitting next to Alice.",
        "Bob would lose 7 happiness units by sitting next to Carol.",
        "Bob would lose 63 happiness units by sitting next to David.",
        "Carol would lose 62 happiness units by sitting next to Alice.",
        "Carol would gain 60 happiness units by sitting next to Bob.",
        "Carol would gain 55 happiness units by sitting next to David.",
        "David would gain 46 happiness units by sitting next to Alice.",
        "David would lose 7 happiness units by sitting next to Bob.",
        "David would gain 41 happiness units by sitting next to Carol.",
    ];
    let (people, map) = parse_input(&lines);

    c.bench_function("part 1", |b| {
        b.iter(|| part1(black_box(&people), black_box(&map)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
