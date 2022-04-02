use criterion::{black_box, Criterion};
use factorial_zeros::{factorial_zeros, factorial_zeros_fast};
use num::BigInt;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fast", |b| b.iter(|| factorial_zeros_fast(black_box(123456789))));
    c.bench_function("big", |b| b.iter(|| factorial_zeros(black_box(BigInt::from(123456789)))));
}

pub fn benches() {
    let mut criterion: Criterion<_> = Criterion::default().configure_from_args();
    criterion_benchmark(&mut criterion);
}

fn main() {
    ::criterion::__warn_about_html_reports_feature();
    ::criterion::__warn_about_cargo_bench_support_feature();

    benches();

    Criterion::default().configure_from_args().final_summary();
}
