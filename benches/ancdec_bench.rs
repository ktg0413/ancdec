use ancdec::{AncDec, AncDec128, AncDec32, AncDec8, RoundMode};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_decimal::prelude::*;
use rust_decimal::Decimal as RustDecimal;
use std::str::FromStr;

// Common values for basic benchmarks:
//   a = 12.345, b = 1.2
//   AncDec8 uses a = 12.34 (u8 frac can't hold 345 at scale=3)

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |bencher| {
        bencher.iter(|| black_box(black_box(a8) + black_box(b8)))
    });

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |bencher| {
        bencher.iter(|| black_box(black_box(a32) + black_box(b32)))
    });

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

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(a128) + black_box(b128)))
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

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |bencher| {
        bencher.iter(|| black_box(black_box(a8) - black_box(b8)))
    });

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |bencher| {
        bencher.iter(|| black_box(black_box(a32) - black_box(b32)))
    });

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

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(a128) - black_box(b128)))
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

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |bencher| {
        bencher.iter(|| black_box(black_box(a8) * black_box(b8)))
    });

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |bencher| {
        bencher.iter(|| black_box(black_box(a32) * black_box(b32)))
    });

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

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(a128) * black_box(b128)))
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

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |bencher| {
        bencher.iter(|| black_box(black_box(a8) / black_box(b8)))
    });

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |bencher| {
        bencher.iter(|| black_box(black_box(a32) / black_box(b32)))
    });

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

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(a128) / black_box(b128)))
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

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |bencher| {
        bencher.iter(|| black_box(black_box(a8) > black_box(b8)))
    });

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |bencher| {
        bencher.iter(|| black_box(black_box(a32) > black_box(b32)))
    });

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

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(a128) > black_box(b128)))
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

    // AncDec8 uses shorter input due to u8 integer range (max 255)
    group.bench_function("ancdec8", |bencher| {
        bencher.iter(|| black_box(black_box("12.34").parse::<AncDec8>().unwrap()))
    });

    let input = "12345.6789";
    group.bench_function("ancdec32", |bencher| {
        bencher.iter(|| black_box(black_box(input).parse::<AncDec32>().unwrap()))
    });

    group.bench_function("ancdec", |bencher| {
        bencher.iter(|| black_box(black_box(input).parse::<AncDec>().unwrap()))
    });

    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(input).parse::<AncDec128>().unwrap()))
    });

    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(input).parse::<RustDecimal>().unwrap()))
    });

    group.finish();
}

fn bench_parse_high_precision(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_high_precision");

    // rust_decimal supports up to 28-29 significant digits;
    // use a common 28-digit string that both libraries can parse
    let input = "1234567890123456.789012345678";

    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(input).parse::<AncDec128>().unwrap()))
    });

    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(input).parse::<RustDecimal>().unwrap()))
    });

    group.finish();
}

fn bench_mul_high_precision(c: &mut Criterion) {
    let mut group = c.benchmark_group("mul_high_precision");

    // Same value for both: 123456789.123456789 and 987654321.987654321
    let a128: AncDec128 = "123456789.123456789".parse().unwrap();
    let b128: AncDec128 = "987654321.987654321".parse().unwrap();
    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(a128) * black_box(b128)))
    });

    let a_rust = RustDecimal::from_str("123456789.123456789").unwrap();
    let b_rust = RustDecimal::from_str("987654321.987654321").unwrap();
    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(a_rust) * black_box(b_rust)))
    });

    group.finish();
}

fn bench_div_high_precision(c: &mut Criterion) {
    let mut group = c.benchmark_group("div_high_precision");

    // Same value for both: 123456789.123456789 / 987654321.987654321
    let a128: AncDec128 = "123456789.123456789".parse().unwrap();
    let b128: AncDec128 = "987654321.987654321".parse().unwrap();
    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(a128) / black_box(b128)))
    });

    let a_rust = RustDecimal::from_str("123456789.123456789").unwrap();
    let b_rust = RustDecimal::from_str("987654321.987654321").unwrap();
    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(a_rust) / black_box(b_rust)))
    });

    group.finish();
}

fn bench_round(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");

    let a8: AncDec8 = "3.14".parse().unwrap();
    group.bench_function("ancdec8", |bencher| {
        bencher.iter(|| black_box(black_box(a8).round(1, RoundMode::HalfUp)))
    });

    let a32: AncDec32 = "3.14159".parse().unwrap();
    group.bench_function("ancdec32", |bencher| {
        bencher.iter(|| black_box(black_box(a32).round(2, RoundMode::HalfUp)))
    });

    let a: AncDec = "3.14159265358979".parse().unwrap();
    group.bench_function("ancdec", |bencher| {
        bencher.iter(|| black_box(black_box(a).round(2, RoundMode::HalfUp)))
    });

    let a128: AncDec128 = "3.14159265358979323846".parse().unwrap();
    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(a128).round(2, RoundMode::HalfUp)))
    });

    let a_rust = RustDecimal::from_str("3.14159265358979").unwrap();
    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(a_rust).round_dp(2)))
    });

    group.finish();
}

fn bench_sqrt(c: &mut Criterion) {
    let mut group = c.benchmark_group("sqrt");

    let a8: AncDec8 = "4".parse().unwrap();
    group.bench_function("ancdec8", |bencher| {
        bencher.iter(|| black_box(black_box(a8).sqrt()))
    });

    let a32: AncDec32 = "2".parse().unwrap();
    group.bench_function("ancdec32", |bencher| {
        bencher.iter(|| black_box(black_box(a32).sqrt()))
    });

    let a: AncDec = "2".parse().unwrap();
    group.bench_function("ancdec", |bencher| {
        bencher.iter(|| black_box(black_box(a).sqrt()))
    });

    let a128: AncDec128 = "2".parse().unwrap();
    group.bench_function("ancdec128", |bencher| {
        bencher.iter(|| black_box(black_box(a128).sqrt()))
    });

    let a_rust = RustDecimal::from_str("2").unwrap();
    group.bench_function("rust_decimal", |bencher| {
        bencher.iter(|| black_box(black_box(a_rust).sqrt()))
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
    bench_parse,
    bench_parse_high_precision,
    bench_mul_high_precision,
    bench_div_high_precision,
    bench_round,
    bench_sqrt,
);
criterion_main!(benches);
