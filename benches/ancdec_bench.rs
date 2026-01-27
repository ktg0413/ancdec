use ancdec::AncDec;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal as RustDecimal;

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");

    let a = AncDec {
        int: 12,
        frac: 345,
        scale: 3,
        neg: false,
    };
    let b = AncDec {
        int: 1,
        frac: 2,
        scale: 1,
        neg: false,
    };
    group.bench_function("ancdec", |bencher| {
        bencher.iter(|| black_box(black_box(a) + black_box(b)))
    });

    let a_rust = RustDecimal::new(12345, 3);
    let b_rust = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(a_rust) + black_box(b_rust)))
    });

    group.finish();
}

fn bench_sub(c: &mut Criterion) {
    let mut group = c.benchmark_group("sub");

    let a = AncDec {
        int: 12,
        frac: 345,
        scale: 3,
        neg: false,
    };
    let b = AncDec {
        int: 1,
        frac: 2,
        scale: 1,
        neg: false,
    };
    group.bench_function("ancdec", |bencher| {
        bencher.iter(|| black_box(black_box(a) - black_box(b)))
    });

    let a_rust = RustDecimal::new(12345, 3);
    let b_rust = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(a_rust) - black_box(b_rust)))
    });

    group.finish();
}

fn bench_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("mul");

    let a = AncDec {
        int: 12,
        frac: 345,
        scale: 3,
        neg: false,
    };
    let b = AncDec {
        int: 1,
        frac: 2,
        scale: 1,
        neg: false,
    };
    group.bench_function("ancdec", |bencher| {
        bencher.iter(|| black_box(black_box(a) * black_box(b)))
    });

    let a_rust = RustDecimal::new(12345, 3);
    let b_rust = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(a_rust) * black_box(b_rust)))
    });

    group.finish();
}

fn bench_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("div");

    let a = AncDec {
        int: 12,
        frac: 345,
        scale: 3,
        neg: false,
    };
    let b = AncDec {
        int: 1,
        frac: 2,
        scale: 1,
        neg: false,
    };
    group.bench_function("ancdec", |bencher| {
        bencher.iter(|| black_box(black_box(a) / black_box(b)))
    });

    let a_rust = RustDecimal::new(12345, 3);
    let b_rust = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(a_rust) / black_box(b_rust)))
    });

    group.finish();
}

fn bench_cmp(c: &mut Criterion) {
    let mut group = c.benchmark_group("cmp");

    let a = AncDec {
        int: 12,
        frac: 345,
        scale: 3,
        neg: false,
    };
    let b = AncDec {
        int: 1,
        frac: 2,
        scale: 1,
        neg: false,
    };
    group.bench_function("ancdec", |bencher| {
        bencher.iter(|| black_box(black_box(a) > black_box(b)))
    });

    let a_rust = RustDecimal::new(12345, 3);
    let b_rust = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(a_rust) > black_box(b_rust)))
    });

    group.finish();
}

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");

    group.bench_function("ancdec", |bencher| {
        bencher.iter(|| black_box(black_box("12345.6789").parse::<AncDec>().unwrap()))
    });

    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box("12345.6789").parse::<RustDecimal>().unwrap()))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_add,
    bench_sub,
    bench_mul,
    bench_div,
    bench_cmp,
    bench_parse
);
criterion_main!(benches);
