use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chemical_elements::isotopic_pattern::{poisson_approximation, TheoreticalIsotopicPattern};

fn make_tid() -> TheoreticalIsotopicPattern {
    TheoreticalIsotopicPattern::new(poisson_approximation(1200.0, 8, 2), 1200.0)
}

fn tid_filter(c: &mut Criterion) {
    let tid = make_tid();
    c.bench_function("combined", |b| {
        b.iter(|| {
            black_box(
                tid.clone()
                    .truncate_after_ignore_below_shift_normalize(0.95, 0.001, 10.0),
            )
        })
    });
    c.bench_function("step_wise", |b| {
        b.iter(|| {
            black_box(
                tid.clone()
                    .truncate_after(0.95)
                    .ignore_below(0.001)
                    .shift(10.0),
            )
        })
    });
}

criterion_group!(benches, tid_filter);
criterion_main!(benches);
