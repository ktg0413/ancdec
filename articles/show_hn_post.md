# Show HN: AncDec – a no_std Rust decimal library that stores integer and fraction in separate fields

**Title (250 chars max):**
Show HN: Rust decimal where fraction precision never shrinks

---

**Body (plain text, no markdown tables):**

With most Rust decimal libraries, precision is a shared budget. rust_decimal gives you 28 significant digits total — so a 20-digit integer leaves only 8 for the fraction. As your integers grow, your fraction precision shrinks.

AncDec doesn't share. Integer and fraction each get their own full-width field. In AncDec (u64), you always have 19 integer digits and 19 fraction digits simultaneously, regardless of how large either gets. 9999999999999999999.999999999999999999 is representable. In rust_decimal it isn't.

The library ships four types:

  AncDec8   (u8,   4 bytes)  — embedded/IoT, 2+2 digit precision
  AncDec32  (u32, 12 bytes)  — general purpose, 9+9 digits
  AncDec    (u64, 24 bytes)  — financial, 19+19 digits
  AncDec128 (u128, 40 bytes) — institutional, 38+38 digits

All four are no_std with zero heap allocation and zero non-optional dependencies. Cross-type arithmetic is built in (AncDec8 + AncDec32 → AncDec32), and wide arithmetic (u256/u512) handles mul/div overflow internally.

I benchmarked all four against rust_decimal, fastnum (D256), fixed-num (Dec19x19), and bigdecimal across 24 groups. AncDec32 wins 10 of 24 groups including div (12 ns vs rust_decimal's 27 ns), floor (7.3 ns vs rust_decimal's 35 ns), and end-to-end workflow (21 ns vs rust_decimal's 115 ns).

The most counterintuitive finding: fixed-num wins 8 individual groups (add, sub, cmp — pure scalar ops), but its parse is 407 ns against ancdec8's 9.4 ns (43x slower). A parse→add→mul→div→round workflow: 1,503 ns vs ancdec32's 21 ns — 71x slower. The scalar wins disappear the moment string boundaries appear.

GitHub: https://github.com/ktg0413/ancdec  
crates.io: https://crates.io/crates/ancdec  
Benchmark code: https://github.com/ktg0413/ancdec/blob/main/benches/ancdec_bench.rs

Happy to answer questions about the architecture, the wide arithmetic implementation, or the benchmark methodology.
