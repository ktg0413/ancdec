# The decimal benchmark that exposes a hidden cost: parse

*How fixed_num wins 8 of 24 individual benchmark groups — then loses the end-to-end workflow by 71×*

---

## Background

I've been working on [ancdec](https://crates.io/crates/ancdec), a `no_std` fixed-point decimal library with a split int/frac storage model. While tuning it, I ran an 8-way benchmark against the most commonly recommended Rust decimal crates:

- `ancdec` (all 4 sizes: u8/u32/u64/u128)
- `rust_decimal`
- `fastnum` (D256)
- `fixed-num` (Dec19x19)
- `bigdecimal`

24 benchmark groups, all on identical inputs. The results had one surprise I wasn't expecting.

---

## The surprising leader: fixed_num

`fixed_num` (Dec19x19) dominates the scalar operation groups:

| Operation | fixed_num | ancdec32 | rust_decimal |
|-----------|-----------|----------|--------------|
| add | **1.9 ns** 🥇 | 7.9 ns | 12.5 ns |
| sub | **1.9 ns** 🥇 | 8.5 ns | 13.8 ns |
| neg_add | **1.3 ns** 🥇 | 5.3 ns | 10.9 ns |
| abs | **1.1 ns** 🥇 | 2.7 ns | 2.1 ns |
| cmp | **0.97 ns** 🥇 | 6.8 ns | 9.7 ns |

That's 8 first-place finishes total. The reason makes sense: `fixed_num` uses a compile-time-fixed scale (`Dec19x19` means exactly 19 integer + 19 fraction digits, always). Operations on two values with the same fixed scale reduce to plain integer arithmetic with no alignment step — effectively a newtype wrapper around multiplication.

---

## The twist: parse

Then I measured parse:

| Operation | ancdec8 | ancdec32 | rust_decimal | fastnum | **fixed_num** | bigdecimal |
|-----------|---------|----------|--------------|---------|---------------|------------|
| parse | **9.4 ns** 🥇 | 14.3 ns | 12.8 ns | 21.6 ns | **407 ns** | 202 ns |

`fixed_num` is **43× slower** than `ancdec8` at parsing a decimal string. And it's not close to any other library either — the next slowest is `bigdecimal` at 202 ns, still 2× faster.

The reason follows from its design: because the scale is fixed at compile time, parsing a string like `"3.14"` into `Dec19x19` means computing a precise integer representation at exactly 19 decimal places — which requires full-precision integer scaling on every parse call.

Display has the same problem:

| Operation | ancdec32 | rust_decimal | **fixed_num** |
|-----------|----------|--------------|---------------|
| display | **93 ns** 🥇 | 120 ns | 313 ns |

---

## The collapse: end-to-end workflow

I added a workflow benchmark: `parse → add → mul → div → round`, all in one call — a realistic unit of work for any program that reads decimal input and produces decimal output.

| Workflow | ancdec32 | ancdec8 | rust_decimal | fastnum | **fixed_num** | bigdecimal |
|----------|----------|---------|--------------|---------|---------------|------------|
| time | **21.1 ns** 🥇 | 30.0 ns | 115.4 ns | 305.9 ns | **1,503 ns** | 1,048 ns |

`fixed_num`: 1,503 ns. `ancdec32`: 21.1 ns. That's **71× slower**.

`bigdecimal`, which allocates on the heap for every operation, actually beats `fixed_num` here at 1,048 ns.

The scalar wins evaporate completely the moment the benchmark includes any string boundary.

---

## Win count across all 24 groups

| Library | Wins |
|---------|------|
| **ancdec32** | 10 |
| **fixed_num** | 8 |
| **ancdec8** | 4 |
| **ancdec128** | 2 |
| rust_decimal | 0 |
| fastnum | 0 |
| bigdecimal | 0 |
| ancdec (u64) | 0 |

`fixed_num` wins 8 groups. All 8 are pure in-memory scalar ops on pre-constructed values. It wins zero groups that touch strings.

---

## What this means

`fixed_num` is the right choice when:
- Values are constructed once (e.g., from a compile-time constant or a single-time conversion)
- All subsequent work is arithmetic in a tight loop
- You never format output or parse input at runtime

It is a poor fit when:
- You read decimal input from users, files, or network
- You log or serialize output
- Your "hot path" includes any string operation

This is the hidden cost of compile-time-fixed scale: the conversion burden moves entirely to the parse/display boundary, and that boundary is almost always on the critical path.

---

## The broader lesson

**Benchmark the workflow, not just the operation.**

A library that wins `add` by 4× and loses `parse` by 43× is likely slower in production. Any benchmark suite that only measures isolated arithmetic is measuring a workload that rarely exists outside synthetic benchmarks.

The benchmark code is in [`benches/ancdec_bench.rs`](https://github.com/ktg0413/ancdec/blob/main/benches/ancdec_bench.rs) — 24 groups, 8 libraries, identical inputs. Reproducible with `cargo bench`.

*Benchmarked on Intel Core i7-10750H @ 2.60GHz, Rust 1.87.0 nightly, release mode.*
