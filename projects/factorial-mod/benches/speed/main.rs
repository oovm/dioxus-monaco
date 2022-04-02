use criterion::{black_box, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
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
