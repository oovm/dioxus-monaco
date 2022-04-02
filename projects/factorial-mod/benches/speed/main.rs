use criterion::{black_box, Criterion};
use factorial_mod::power_mod_fast;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fast", |b| b.iter(|| power_mod_fast(black_box(123456789), black_box(123456789), black_box(10086))));
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
