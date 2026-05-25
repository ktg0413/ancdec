use ancdec::{AncDec, AncDec128, AncDec32, AncDec8, RoundMode};
use bigdecimal::{BigDecimal, RoundingMode as BdRM};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastnum::{decimal::RoundingMode as FnRM, D256};
use fixed_num::{
    ops::{Abs, Ceil, Floor, RoundTo, UncheckedSqrt},
    Dec19x19,
};
use rust_decimal::prelude::*;
use rust_decimal::Decimal as RustDecimal;
use std::str::FromStr;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Helpers
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// Base values: a = 12.345, b = 1.2  (AncDec8: 12.34 due to u8 frac limit)
// All 8 competitors in every group:
//   ancdec8, ancdec32, ancdec, ancdec128
//   rust_decimal, fastnum_d256, fixed_num, bigdecimal

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// ADD
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8) + black_box(b8))));

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32) + black_box(b32))));

    let a = AncDec { int: 12, frac: 345, scale: 3, neg: false };
    let b = AncDec { int: 1, frac: 2, scale: 1, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a) + black_box(b))));

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) + black_box(b128))));

    let a_rd = RustDecimal::new(12345, 3);
    let b_rd = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) + black_box(b_rd))));

    let a_fn: D256 = "12.345".parse().unwrap();
    let b_fn: D256 = "1.2".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn) + black_box(b_fn))));

    let a_dec: Dec19x19 = "12.345".parse().unwrap();
    let b_dec: Dec19x19 = "1.2".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec) + black_box(b_dec))));

    let a_bd: BigDecimal = "12.345".parse().unwrap();
    let b_bd: BigDecimal = "1.2".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) + black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// SUB
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_sub(c: &mut Criterion) {
    let mut group = c.benchmark_group("sub");

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8) - black_box(b8))));

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32) - black_box(b32))));

    let a = AncDec { int: 12, frac: 345, scale: 3, neg: false };
    let b = AncDec { int: 1, frac: 2, scale: 1, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a) - black_box(b))));

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) - black_box(b128))));

    let a_rd = RustDecimal::new(12345, 3);
    let b_rd = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) - black_box(b_rd))));

    let a_fn: D256 = "12.345".parse().unwrap();
    let b_fn: D256 = "1.2".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn) - black_box(b_fn))));

    let a_dec: Dec19x19 = "12.345".parse().unwrap();
    let b_dec: Dec19x19 = "1.2".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec) - black_box(b_dec))));

    let a_bd: BigDecimal = "12.345".parse().unwrap();
    let b_bd: BigDecimal = "1.2".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) - black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// MUL
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("mul");

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8) * black_box(b8))));

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32) * black_box(b32))));

    let a = AncDec { int: 12, frac: 345, scale: 3, neg: false };
    let b = AncDec { int: 1, frac: 2, scale: 1, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a) * black_box(b))));

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) * black_box(b128))));

    let a_rd = RustDecimal::new(12345, 3);
    let b_rd = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) * black_box(b_rd))));

    let a_fn: D256 = "12.345".parse().unwrap();
    let b_fn: D256 = "1.2".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn) * black_box(b_fn))));

    let a_dec: Dec19x19 = "12.345".parse().unwrap();
    let b_dec: Dec19x19 = "1.2".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec) * black_box(b_dec))));

    let a_bd: BigDecimal = "12.345".parse().unwrap();
    let b_bd: BigDecimal = "1.2".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) * black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// DIV
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("div");

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8) / black_box(b8))));

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32) / black_box(b32))));

    let a = AncDec { int: 12, frac: 345, scale: 3, neg: false };
    let b = AncDec { int: 1, frac: 2, scale: 1, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a) / black_box(b))));

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) / black_box(b128))));

    let a_rd = RustDecimal::new(12345, 3);
    let b_rd = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) / black_box(b_rd))));

    let a_fn: D256 = "12.345".parse().unwrap();
    let b_fn: D256 = "1.2".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn) / black_box(b_fn))));

    let a_dec: Dec19x19 = "12.345".parse().unwrap();
    let b_dec: Dec19x19 = "1.2".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec) / black_box(b_dec))));

    let a_bd: BigDecimal = "12.345".parse().unwrap();
    let b_bd: BigDecimal = "1.2".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) / black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// REM
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_rem(c: &mut Criterion) {
    let mut group = c.benchmark_group("rem");

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8) % black_box(b8))));

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32) % black_box(b32))));

    let a = AncDec { int: 12, frac: 345, scale: 3, neg: false };
    let b = AncDec { int: 1, frac: 2, scale: 1, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a) % black_box(b))));

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) % black_box(b128))));

    let a_rd = RustDecimal::new(12345, 3);
    let b_rd = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) % black_box(b_rd))));

    let a_fn: D256 = "12.345".parse().unwrap();
    let b_fn: D256 = "1.2".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn) % black_box(b_fn))));

    let a_dec: Dec19x19 = "12.345".parse().unwrap();
    let b_dec: Dec19x19 = "1.2".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec) % black_box(b_dec))));

    let a_bd: BigDecimal = "12.345".parse().unwrap();
    let b_bd: BigDecimal = "1.2".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) % black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// NEG_ADD  (-12.345) + 1.2
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_neg_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("neg_add");

    let a8 = AncDec8::new(12, 34, 2, true);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8) + black_box(b8))));

    let a32 = AncDec32::new(12, 345, 3, true);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32) + black_box(b32))));

    let a = AncDec { int: 12, frac: 345, scale: 3, neg: true };
    let b = AncDec { int: 1, frac: 2, scale: 1, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a) + black_box(b))));

    let a128 = AncDec128::new(12, 345, 3, true);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) + black_box(b128))));

    let a_rd = RustDecimal::new(-12345, 3);
    let b_rd = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) + black_box(b_rd))));

    let pos_fn: D256 = "12.345".parse().unwrap();
    let neg_a_fn = -pos_fn;
    let b_fn: D256 = "1.2".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(neg_a_fn) + black_box(b_fn))));

    let pos_dec: Dec19x19 = "12.345".parse().unwrap();
    let neg_a_dec = -pos_dec;
    let b_dec: Dec19x19 = "1.2".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(neg_a_dec) + black_box(b_dec))));

    let a_bd: BigDecimal = "-12.345".parse().unwrap();
    let b_bd: BigDecimal = "1.2".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) + black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// ABS
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_abs(c: &mut Criterion) {
    let mut group = c.benchmark_group("abs");

    let a8 = AncDec8::new(12, 34, 2, true);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8).abs())));

    let a32 = AncDec32::new(12, 345, 3, true);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32).abs())));

    let a = AncDec { int: 12, frac: 345, scale: 3, neg: true };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a).abs())));

    let a128 = AncDec128::new(12, 345, 3, true);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128).abs())));

    let a_rd = RustDecimal::new(-12345, 3);
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd).abs())));

    let pos_fn: D256 = "12.345".parse().unwrap();
    let neg_fn = -pos_fn;
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(neg_fn).abs())));

    let pos_dec: Dec19x19 = "12.345".parse().unwrap();
    let neg_dec = -pos_dec;
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(neg_dec).abs())));

    let a_bd: BigDecimal = "-12.345".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd).abs())));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// CMP
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_cmp(c: &mut Criterion) {
    let mut group = c.benchmark_group("cmp");

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8) > black_box(b8))));

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32) > black_box(b32))));

    let a = AncDec { int: 12, frac: 345, scale: 3, neg: false };
    let b = AncDec { int: 1, frac: 2, scale: 1, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a) > black_box(b))));

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) > black_box(b128))));

    let a_rd = RustDecimal::new(12345, 3);
    let b_rd = RustDecimal::new(12, 1);
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) > black_box(b_rd))));

    let a_fn: D256 = "12.345".parse().unwrap();
    let b_fn: D256 = "1.2".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn) > black_box(b_fn))));

    let a_dec: Dec19x19 = "12.345".parse().unwrap();
    let b_dec: Dec19x19 = "1.2".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec) > black_box(b_dec))));

    let a_bd: BigDecimal = "12.345".parse().unwrap();
    let b_bd: BigDecimal = "1.2".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) > black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// PARSE
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");

    group.bench_function("ancdec8", |be| {
        be.iter(|| black_box(black_box("12.34").parse::<AncDec8>().unwrap()))
    });

    let s = "12345.6789";
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(s).parse::<AncDec32>().unwrap())));
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(s).parse::<AncDec>().unwrap())));
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(s).parse::<AncDec128>().unwrap())));
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(s).parse::<RustDecimal>().unwrap())));
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(s).parse::<D256>().unwrap())));
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(s).parse::<Dec19x19>().unwrap())));
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(s).parse::<BigDecimal>().unwrap())));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// PARSE HIGH PRECISION  (28+ sig digits)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_parse_high_precision(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_high_precision");

    // rust_decimal caps at 28 significant digits; all others handle more
    let s = "1234567890123456.789012345678";
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(s).parse::<AncDec128>().unwrap())));
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(s).parse::<RustDecimal>().unwrap())));
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(s).parse::<D256>().unwrap())));
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(s).parse::<Dec19x19>().unwrap())));
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(s).parse::<BigDecimal>().unwrap())));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// MUL HIGH PRECISION
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_mul_high_precision(c: &mut Criterion) {
    let mut group = c.benchmark_group("mul_high_precision");

    let a128: AncDec128 = "123456789.123456789".parse().unwrap();
    let b128: AncDec128 = "987654321.987654321".parse().unwrap();
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) * black_box(b128))));

    let a_rd = RustDecimal::from_str("123456789.123456789").unwrap();
    let b_rd = RustDecimal::from_str("987654321.987654321").unwrap();
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) * black_box(b_rd))));

    let a_fn: D256 = "123456789.123456789".parse().unwrap();
    let b_fn: D256 = "987654321.987654321".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn) * black_box(b_fn))));

    let a_dec: Dec19x19 = "123456789.123456789".parse().unwrap();
    let b_dec: Dec19x19 = "987654321.987654321".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec) * black_box(b_dec))));

    let a_bd: BigDecimal = "123456789.123456789".parse().unwrap();
    let b_bd: BigDecimal = "987654321.987654321".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) * black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// DIV HIGH PRECISION
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_div_high_precision(c: &mut Criterion) {
    let mut group = c.benchmark_group("div_high_precision");

    let a128: AncDec128 = "123456789.123456789".parse().unwrap();
    let b128: AncDec128 = "987654321.987654321".parse().unwrap();
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) / black_box(b128))));

    let a_rd = RustDecimal::from_str("123456789.123456789").unwrap();
    let b_rd = RustDecimal::from_str("987654321.987654321").unwrap();
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) / black_box(b_rd))));

    let a_fn: D256 = "123456789.123456789".parse().unwrap();
    let b_fn: D256 = "987654321.987654321".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn) / black_box(b_fn))));

    let a_dec: Dec19x19 = "123456789.123456789".parse().unwrap();
    let b_dec: Dec19x19 = "987654321.987654321".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec) / black_box(b_dec))));

    let a_bd: BigDecimal = "123456789.123456789".parse().unwrap();
    let b_bd: BigDecimal = "987654321.987654321".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) / black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// ROUND (to 2 dp, HalfUp)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_round(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");

    let a8: AncDec8 = "3.14".parse().unwrap();
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8).round(1, RoundMode::HalfUp))));

    let a32: AncDec32 = "3.14159".parse().unwrap();
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32).round(2, RoundMode::HalfUp))));

    let a: AncDec = "3.14159265358979".parse().unwrap();
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a).round(2, RoundMode::HalfUp))));

    let a128: AncDec128 = "3.14159265358979323846".parse().unwrap();
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128).round(2, RoundMode::HalfUp))));

    let a_rd = RustDecimal::from_str("3.14159265358979").unwrap();
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd).round_dp(2))));

    let a_fn: D256 = "3.14159265358979".parse().unwrap();
    group.bench_function("fastnum_d256", |be| {
        be.iter(|| black_box(black_box(a_fn).with_rounding_mode(FnRM::HalfUp).rescale(2)))
    });

    let a_dec: Dec19x19 = "3.14159265358979".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec).round_to(2))));

    let a_bd: BigDecimal = "3.14159265358979".parse().unwrap();
    group.bench_function("bigdecimal", |be| {
        be.iter(|| black_box(black_box(&a_bd).with_scale_round(2, BdRM::HalfUp)))
    });

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// FLOOR
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_floor(c: &mut Criterion) {
    let mut group = c.benchmark_group("floor");

    let a8: AncDec8 = "3.14".parse().unwrap();
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8).floor())));

    let a32: AncDec32 = "3.14159".parse().unwrap();
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32).floor())));

    let a: AncDec = "3.14159265358979".parse().unwrap();
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a).floor())));

    let a128: AncDec128 = "3.14159265358979323846".parse().unwrap();
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128).floor())));

    let a_rd = RustDecimal::from_str("3.14159265358979").unwrap();
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd).floor())));

    let a_fn: D256 = "3.14159265358979".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn).floor())));

    let a_dec: Dec19x19 = "3.14159265358979".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec).floor())));

    let a_bd: BigDecimal = "3.14159265358979".parse().unwrap();
    group.bench_function("bigdecimal", |be| {
        be.iter(|| black_box(black_box(&a_bd).with_scale_round(0, BdRM::Floor)))
    });

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// CEIL
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_ceil(c: &mut Criterion) {
    let mut group = c.benchmark_group("ceil");

    let a8: AncDec8 = "3.14".parse().unwrap();
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8).ceil())));

    let a32: AncDec32 = "3.14159".parse().unwrap();
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32).ceil())));

    let a: AncDec = "3.14159265358979".parse().unwrap();
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a).ceil())));

    let a128: AncDec128 = "3.14159265358979323846".parse().unwrap();
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128).ceil())));

    let a_rd = RustDecimal::from_str("3.14159265358979").unwrap();
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd).ceil())));

    let a_fn: D256 = "3.14159265358979".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn).ceil())));

    let a_dec: Dec19x19 = "3.14159265358979".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec).ceil())));

    let a_bd: BigDecimal = "3.14159265358979".parse().unwrap();
    group.bench_function("bigdecimal", |be| {
        be.iter(|| black_box(black_box(&a_bd).with_scale_round(0, BdRM::Ceiling)))
    });

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// SQRT
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_sqrt(c: &mut Criterion) {
    let mut group = c.benchmark_group("sqrt");

    let a8: AncDec8 = "4".parse().unwrap();
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8).sqrt())));

    let a32: AncDec32 = "2".parse().unwrap();
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32).sqrt())));

    let a: AncDec = "2".parse().unwrap();
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a).sqrt())));

    let a128: AncDec128 = "2".parse().unwrap();
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128).sqrt())));

    let a_rd = RustDecimal::from_str("2").unwrap();
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd).sqrt())));

    let a_fn: D256 = "2".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn).sqrt())));

    let a_dec: Dec19x19 = "2".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec).unchecked_sqrt())));

    let a_bd: BigDecimal = "2".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd).sqrt())));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// DISPLAY
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_display(c: &mut Criterion) {
    let mut group = c.benchmark_group("display");

    let a8 = AncDec8::new(12, 34, 2, false);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(format!("{}", black_box(a8)))));

    let a32 = AncDec32::new(12, 345, 3, false);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(format!("{}", black_box(a32)))));

    let a = AncDec { int: 12, frac: 345, scale: 3, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(format!("{}", black_box(a)))));

    let a128 = AncDec128::new(12, 345, 3, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(format!("{}", black_box(a128)))));

    let a_rd = RustDecimal::new(12345, 3);
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(format!("{}", black_box(a_rd)))));

    let a_fn: D256 = "12.345".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(format!("{}", black_box(a_fn)))));

    let a_dec: Dec19x19 = "12.345".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(format!("{}", black_box(a_dec)))));

    let a_bd: BigDecimal = "12.345".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(format!("{}", black_box(&a_bd)))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// CHAIN OPS  (a + b) * c
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_chain_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("chain_ops");

    let a8 = AncDec8::new(12, 34, 2, false);
    let b8 = AncDec8::new(1, 2, 1, false);
    let c8: AncDec8 = "3.14".parse().unwrap();
    group.bench_function("ancdec8", |be| {
        be.iter(|| black_box((black_box(a8) + black_box(b8)) * black_box(c8)))
    });

    let a32 = AncDec32::new(12, 345, 3, false);
    let b32 = AncDec32::new(1, 2, 1, false);
    let c32: AncDec32 = "3.14159".parse().unwrap();
    group.bench_function("ancdec32", |be| {
        be.iter(|| black_box((black_box(a32) + black_box(b32)) * black_box(c32)))
    });

    let a = AncDec { int: 12, frac: 345, scale: 3, neg: false };
    let b = AncDec { int: 1, frac: 2, scale: 1, neg: false };
    let c64: AncDec = "3.14159265358979".parse().unwrap();
    group.bench_function("ancdec", |be| {
        be.iter(|| black_box((black_box(a) + black_box(b)) * black_box(c64)))
    });

    let a128 = AncDec128::new(12, 345, 3, false);
    let b128 = AncDec128::new(1, 2, 1, false);
    let c128: AncDec128 = "3.14159265358979323846".parse().unwrap();
    group.bench_function("ancdec128", |be| {
        be.iter(|| black_box((black_box(a128) + black_box(b128)) * black_box(c128)))
    });

    let a_rd = RustDecimal::new(12345, 3);
    let b_rd = RustDecimal::new(12, 1);
    let c_rd = RustDecimal::from_str("3.14159265358979").unwrap();
    group.bench_function("rust_decimal", |be| {
        be.iter(|| black_box((black_box(a_rd) + black_box(b_rd)) * black_box(c_rd)))
    });

    let a_fn: D256 = "12.345".parse().unwrap();
    let b_fn: D256 = "1.2".parse().unwrap();
    let c_fn: D256 = "3.14159265358979".parse().unwrap();
    group.bench_function("fastnum_d256", |be| {
        be.iter(|| black_box((black_box(a_fn) + black_box(b_fn)) * black_box(c_fn)))
    });

    let a_dec: Dec19x19 = "12.345".parse().unwrap();
    let b_dec: Dec19x19 = "1.2".parse().unwrap();
    let c_dec: Dec19x19 = "3.14159265358979".parse().unwrap();
    group.bench_function("fixed_num", |be| {
        be.iter(|| black_box((black_box(a_dec) + black_box(b_dec)) * black_box(c_dec)))
    });

    let a_bd: BigDecimal = "12.345".parse().unwrap();
    let b_bd: BigDecimal = "1.2".parse().unwrap();
    let c_bd: BigDecimal = "3.14159265358979".parse().unwrap();
    group.bench_function("bigdecimal", |be| {
        be.iter(|| black_box((black_box(&a_bd) + black_box(&b_bd)) * black_box(&c_bd)))
    });

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// SUM 10  (accumulate 10 values)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_sum_10(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_10");

    // 1.1 + 1.2 + … + 1.9 + 2.0 = 16.5
    let vals8 = [
        AncDec8::new(1, 1, 1, false), AncDec8::new(1, 2, 1, false),
        AncDec8::new(1, 3, 1, false), AncDec8::new(1, 4, 1, false),
        AncDec8::new(1, 5, 1, false), AncDec8::new(1, 6, 1, false),
        AncDec8::new(1, 7, 1, false), AncDec8::new(1, 8, 1, false),
        AncDec8::new(1, 9, 1, false), AncDec8::new(2, 0, 0, false),
    ];
    group.bench_function("ancdec8", |be| {
        be.iter(|| black_box(black_box(&vals8).iter().copied().sum::<AncDec8>()))
    });

    let vals32 = [
        AncDec32::new(1, 100, 3, false), AncDec32::new(2, 200, 3, false),
        AncDec32::new(3, 300, 3, false), AncDec32::new(4, 400, 3, false),
        AncDec32::new(5, 500, 3, false), AncDec32::new(6, 600, 3, false),
        AncDec32::new(7, 700, 3, false), AncDec32::new(8, 800, 3, false),
        AncDec32::new(9, 900, 3, false), AncDec32::new(10, 0, 0, false),
    ];
    group.bench_function("ancdec32", |be| {
        be.iter(|| black_box(black_box(&vals32).iter().copied().sum::<AncDec32>()))
    });

    let vals64 = [
        AncDec { int: 1, frac: 100, scale: 3, neg: false },
        AncDec { int: 2, frac: 200, scale: 3, neg: false },
        AncDec { int: 3, frac: 300, scale: 3, neg: false },
        AncDec { int: 4, frac: 400, scale: 3, neg: false },
        AncDec { int: 5, frac: 500, scale: 3, neg: false },
        AncDec { int: 6, frac: 600, scale: 3, neg: false },
        AncDec { int: 7, frac: 700, scale: 3, neg: false },
        AncDec { int: 8, frac: 800, scale: 3, neg: false },
        AncDec { int: 9, frac: 900, scale: 3, neg: false },
        AncDec { int: 10, frac: 0, scale: 0, neg: false },
    ];
    group.bench_function("ancdec", |be| {
        be.iter(|| black_box(black_box(&vals64).iter().copied().sum::<AncDec>()))
    });

    let vals128 = [
        AncDec128::new(1, 100, 3, false), AncDec128::new(2, 200, 3, false),
        AncDec128::new(3, 300, 3, false), AncDec128::new(4, 400, 3, false),
        AncDec128::new(5, 500, 3, false), AncDec128::new(6, 600, 3, false),
        AncDec128::new(7, 700, 3, false), AncDec128::new(8, 800, 3, false),
        AncDec128::new(9, 900, 3, false), AncDec128::new(10, 0, 0, false),
    ];
    group.bench_function("ancdec128", |be| {
        be.iter(|| black_box(black_box(&vals128).iter().copied().sum::<AncDec128>()))
    });

    let vals_rd = [
        RustDecimal::new(1100, 3), RustDecimal::new(2200, 3),
        RustDecimal::new(3300, 3), RustDecimal::new(4400, 3),
        RustDecimal::new(5500, 3), RustDecimal::new(6600, 3),
        RustDecimal::new(7700, 3), RustDecimal::new(8800, 3),
        RustDecimal::new(9900, 3), RustDecimal::new(10000, 3),
    ];
    group.bench_function("rust_decimal", |be| {
        be.iter(|| black_box(black_box(&vals_rd).iter().copied().sum::<RustDecimal>()))
    });

    let vals_fn: [D256; 10] = [
        "1.1".parse().unwrap(), "1.2".parse().unwrap(),
        "1.3".parse().unwrap(), "1.4".parse().unwrap(),
        "1.5".parse().unwrap(), "1.6".parse().unwrap(),
        "1.7".parse().unwrap(), "1.8".parse().unwrap(),
        "1.9".parse().unwrap(), "2.0".parse().unwrap(),
    ];
    group.bench_function("fastnum_d256", |be| {
        be.iter(|| black_box(black_box(&vals_fn).iter().copied().sum::<D256>()))
    });

    // Dec19x19 has no Sum impl — use fold
    let vals_dec: [Dec19x19; 10] = [
        "1.1".parse().unwrap(), "1.2".parse().unwrap(),
        "1.3".parse().unwrap(), "1.4".parse().unwrap(),
        "1.5".parse().unwrap(), "1.6".parse().unwrap(),
        "1.7".parse().unwrap(), "1.8".parse().unwrap(),
        "1.9".parse().unwrap(), "2.0".parse().unwrap(),
    ];
    let zero_dec = Dec19x19::from_repr(0);
    group.bench_function("fixed_num", |be| {
        be.iter(|| {
            black_box(black_box(&vals_dec).iter().copied().fold(zero_dec, |acc, x| acc + x))
        })
    });

    let vals_bd: [BigDecimal; 10] = [
        "1.1".parse().unwrap(), "1.2".parse().unwrap(),
        "1.3".parse().unwrap(), "1.4".parse().unwrap(),
        "1.5".parse().unwrap(), "1.6".parse().unwrap(),
        "1.7".parse().unwrap(), "1.8".parse().unwrap(),
        "1.9".parse().unwrap(), "2.0".parse().unwrap(),
    ];
    group.bench_function("bigdecimal", |be| {
        be.iter(|| {
            black_box(
                black_box(&vals_bd)
                    .iter()
                    .fold(BigDecimal::from(0i32), |acc, x| acc + x),
            )
        })
    });

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// WORKFLOW  parse → add → mul → div → round  (all in one shot)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_workflow(c: &mut Criterion) {
    let mut group = c.benchmark_group("workflow");

    group.bench_function("ancdec8", |be| {
        be.iter(|| {
            let v: AncDec8 = black_box("12.34").parse().unwrap();
            let b = AncDec8::new(1, 2, 1, false);
            let m = AncDec8::new(3, 14, 2, false);
            let d = AncDec8::new(2, 0, 0, false);
            black_box(((v + b) * m / d).round(1, RoundMode::HalfUp))
        })
    });

    group.bench_function("ancdec32", |be| {
        be.iter(|| {
            let v: AncDec32 = black_box("12.345").parse().unwrap();
            let b = AncDec32::new(1, 2, 1, false);
            let m = AncDec32::new(3, 14159, 5, false);
            let d = AncDec32::new(2, 0, 0, false);
            black_box(((v + b) * m / d).round(2, RoundMode::HalfUp))
        })
    });

    group.bench_function("ancdec", |be| {
        be.iter(|| {
            let v: AncDec = black_box("12.345").parse().unwrap();
            let b = AncDec { int: 1, frac: 2, scale: 1, neg: false };
            let m = AncDec { int: 3, frac: 14159265358979, scale: 14, neg: false };
            let d = AncDec { int: 2, frac: 0, scale: 0, neg: false };
            black_box(((v + b) * m / d).round(2, RoundMode::HalfUp))
        })
    });

    group.bench_function("ancdec128", |be| {
        be.iter(|| {
            let v: AncDec128 = black_box("12.345").parse().unwrap();
            let b = AncDec128::new(1, 2, 1, false);
            let m = AncDec128::new(3, 14159265358979323846, 20, false);
            let d = AncDec128::new(2, 0, 0, false);
            black_box(((v + b) * m / d).round(2, RoundMode::HalfUp))
        })
    });

    group.bench_function("rust_decimal", |be| {
        be.iter(|| {
            let v: RustDecimal = black_box("12.345").parse().unwrap();
            let b = RustDecimal::new(12, 1);
            let m = RustDecimal::from_str("3.14159265358979").unwrap();
            let d = RustDecimal::new(2, 0);
            black_box(((v + b) * m / d).round_dp(2))
        })
    });

    group.bench_function("fastnum_d256", |be| {
        be.iter(|| {
            let v: D256 = black_box("12.345").parse().unwrap();
            let b: D256 = "1.2".parse().unwrap();
            let m: D256 = "3.14159265358979".parse().unwrap();
            let d: D256 = "2".parse().unwrap();
            black_box(((v + b) * m / d).with_rounding_mode(FnRM::HalfUp).rescale(2))
        })
    });

    group.bench_function("fixed_num", |be| {
        be.iter(|| {
            let v: Dec19x19 = black_box("12.345").parse().unwrap();
            let b: Dec19x19 = "1.2".parse().unwrap();
            let m: Dec19x19 = "3.14159265358979".parse().unwrap();
            let d: Dec19x19 = "2".parse().unwrap();
            black_box(((v + b) * m / d).round_to(2))
        })
    });

    group.bench_function("bigdecimal", |be| {
        be.iter(|| {
            let v: BigDecimal = black_box("12.345").parse().unwrap();
            let b: BigDecimal = "1.2".parse().unwrap();
            let m: BigDecimal = "3.14159265358979".parse().unwrap();
            let d: BigDecimal = "2".parse().unwrap();
            black_box(((v + b) * m / d).with_scale_round(2, BdRM::HalfUp))
        })
    });

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// EXTREME — large integer × large integer
// Values: 1234567890.123456789 × 9876543210.987654321
// Int part of result ≈ 1.22e19, fits in AncDec (u64 max ~9.2e18) — borderline;
// use smaller values that all types survive.
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_extreme_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("extreme_large");

    // AncDec32: u32 int, product must fit in u32 (max ~4.29e9); use 65530.9 * 65530.9 ≈ 4.29e9
    let a32 = AncDec32::new(65530, 9, 1, false); // 65530.9
    let b32 = AncDec32::new(65530, 9, 1, false); // 65530.9
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32) * black_box(b32))));

    // AncDec (u64): product ~1.5e17, fits
    let a = AncDec { int: 123456789, frac: 123456789, scale: 9, neg: false };
    let b = AncDec { int: 987654321, frac: 987654321, scale: 9, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a) * black_box(b))));

    let a128 = AncDec128::new(123456789, 123456789, 9, false);
    let b128 = AncDec128::new(987654321, 987654321, 9, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) * black_box(b128))));

    let a_rd = RustDecimal::from_str("123456789.123456789").unwrap();
    let b_rd = RustDecimal::from_str("987654321.987654321").unwrap();
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) * black_box(b_rd))));

    let a_fn: D256 = "123456789.123456789".parse().unwrap();
    let b_fn: D256 = "987654321.987654321".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn) * black_box(b_fn))));

    let a_dec: Dec19x19 = "123456789.123456789".parse().unwrap();
    let b_dec: Dec19x19 = "987654321.987654321".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec) * black_box(b_dec))));

    let a_bd: BigDecimal = "123456789.123456789".parse().unwrap();
    let b_bd: BigDecimal = "987654321.987654321".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) * black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// EXTREME — maximum fractional precision addition
// 0.1234567890123456789 + 0.9876543210987654321  (19 decimal places)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_extreme_precision(c: &mut Criterion) {
    let mut group = c.benchmark_group("extreme_precision");

    // AncDec8 can only hold scale 0-2, use 2-dp values
    let a8 = AncDec8::new(0, 99, 2, false);
    let b8 = AncDec8::new(0, 99, 2, false);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(a8) + black_box(b8))));

    let a32 = AncDec32::new(0, 123456789, 9, false);
    let b32 = AncDec32::new(0, 987654321, 9, false);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(a32) + black_box(b32))));

    let a = AncDec { int: 0, frac: 1234567890123456789, scale: 19, neg: false };
    let b = AncDec { int: 0, frac: 9876543210987654321, scale: 19, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(a) + black_box(b))));

    let a128 = AncDec128::new(0, 12345678901234567890123456789012345678, 38, false);
    let b128 = AncDec128::new(0, 98765432109876543210987654321098765432, 38, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(a128) + black_box(b128))));

    let a_rd = RustDecimal::from_str("0.1234567890123456789012345").unwrap();
    let b_rd = RustDecimal::from_str("0.8765432109876543210987654").unwrap();
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(a_rd) + black_box(b_rd))));

    let a_fn: D256 = "0.1234567890123456789012345678901234567".parse().unwrap();
    let b_fn: D256 = "0.8765432109876543210987654321098765432".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(a_fn) + black_box(b_fn))));

    let a_dec: Dec19x19 = "0.1234567890123456789".parse().unwrap();
    let b_dec: Dec19x19 = "0.8765432109876543210".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(a_dec) + black_box(b_dec))));

    let a_bd: BigDecimal = "0.1234567890123456789012345678901234567".parse().unwrap();
    let b_bd: BigDecimal = "0.8765432109876543210987654321098765432".parse().unwrap();
    group.bench_function("bigdecimal", |be| be.iter(|| black_box(black_box(&a_bd) + black_box(&b_bd))));

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// ACCURACY / RECURRING DECIMAL  — 1 / 3  (non-terminating)
// Measures how quickly each library handles a non-representable decimal.
// Result will be truncated at each type's precision limit.
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_accuracy_recurring(c: &mut Criterion) {
    let mut group = c.benchmark_group("accuracy_recurring");

    let one8 = AncDec8::new(1, 0, 0, false);
    let three8 = AncDec8::new(3, 0, 0, false);
    group.bench_function("ancdec8", |be| be.iter(|| black_box(black_box(one8) / black_box(three8))));

    let one32 = AncDec32::new(1, 0, 0, false);
    let three32 = AncDec32::new(3, 0, 0, false);
    group.bench_function("ancdec32", |be| be.iter(|| black_box(black_box(one32) / black_box(three32))));

    let one = AncDec { int: 1, frac: 0, scale: 0, neg: false };
    let three = AncDec { int: 3, frac: 0, scale: 0, neg: false };
    group.bench_function("ancdec", |be| be.iter(|| black_box(black_box(one) / black_box(three))));

    let one128 = AncDec128::new(1, 0, 0, false);
    let three128 = AncDec128::new(3, 0, 0, false);
    group.bench_function("ancdec128", |be| be.iter(|| black_box(black_box(one128) / black_box(three128))));

    let one_rd = RustDecimal::new(1, 0);
    let three_rd = RustDecimal::new(3, 0);
    group.bench_function("rust_decimal", |be| be.iter(|| black_box(black_box(one_rd) / black_box(three_rd))));

    let one_fn: D256 = "1".parse().unwrap();
    let three_fn: D256 = "3".parse().unwrap();
    group.bench_function("fastnum_d256", |be| be.iter(|| black_box(black_box(one_fn) / black_box(three_fn))));

    let one_dec: Dec19x19 = "1".parse().unwrap();
    let three_dec: Dec19x19 = "3".parse().unwrap();
    group.bench_function("fixed_num", |be| be.iter(|| black_box(black_box(one_dec) / black_box(three_dec))));

    let one_bd: BigDecimal = "1".parse().unwrap();
    let three_bd: BigDecimal = "3".parse().unwrap();
    group.bench_function("bigdecimal", |be| {
        be.iter(|| black_box(black_box(&one_bd) / black_box(&three_bd)))
    });

    group.finish();
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// ACCURACY — accumulated error:  (1/3 + 1/3 + 1/3) − 1  repeated ops
// Each type computes 1/3, adds three copies, subtracts 1.
// The closer to zero the result, the higher the precision.
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
fn bench_accuracy_accumulated(c: &mut Criterion) {
    let mut group = c.benchmark_group("accuracy_accumulated");

    group.bench_function("ancdec8", |be| {
        be.iter(|| {
            let one = AncDec8::new(1, 0, 0, false);
            let three = AncDec8::new(3, 0, 0, false);
            let t = one / three;
            black_box(t + t + t - one)
        })
    });

    group.bench_function("ancdec32", |be| {
        be.iter(|| {
            let one = AncDec32::new(1, 0, 0, false);
            let three = AncDec32::new(3, 0, 0, false);
            let t = one / three;
            black_box(t + t + t - one)
        })
    });

    group.bench_function("ancdec", |be| {
        be.iter(|| {
            let one = AncDec { int: 1, frac: 0, scale: 0, neg: false };
            let three = AncDec { int: 3, frac: 0, scale: 0, neg: false };
            let t = one / three;
            black_box(t + t + t - one)
        })
    });

    group.bench_function("ancdec128", |be| {
        be.iter(|| {
            let one = AncDec128::new(1, 0, 0, false);
            let three = AncDec128::new(3, 0, 0, false);
            let t = one / three;
            black_box(t + t + t - one)
        })
    });

    group.bench_function("rust_decimal", |be| {
        be.iter(|| {
            let one = RustDecimal::new(1, 0);
            let three = RustDecimal::new(3, 0);
            let t = one / three;
            black_box(t + t + t - one)
        })
    });

    group.bench_function("fastnum_d256", |be| {
        be.iter(|| {
            let one: D256 = "1".parse().unwrap();
            let three: D256 = "3".parse().unwrap();
            let t = one / three;
            black_box(t + t + t - one)
        })
    });

    group.bench_function("fixed_num", |be| {
        be.iter(|| {
            let one: Dec19x19 = "1".parse().unwrap();
            let three: Dec19x19 = "3".parse().unwrap();
            let t = one / three;
            black_box(t + t + t - one)
        })
    });

    group.bench_function("bigdecimal", |be| {
        be.iter(|| {
            let one: BigDecimal = "1".parse().unwrap();
            let three: BigDecimal = "3".parse().unwrap();
            let t = &one / &three;
            black_box(&t + &t + &t - &one)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_add,
    bench_sub,
    bench_mul,
    bench_div,
    bench_rem,
    bench_neg_add,
    bench_abs,
    bench_cmp,
    bench_parse,
    bench_parse_high_precision,
    bench_mul_high_precision,
    bench_div_high_precision,
    bench_round,
    bench_floor,
    bench_ceil,
    bench_sqrt,
    bench_display,
    bench_chain_ops,
    bench_sum_10,
    bench_workflow,
    bench_extreme_large,
    bench_extreme_precision,
    bench_accuracy_recurring,
    bench_accuracy_accumulated,
);
criterion_main!(benches);
