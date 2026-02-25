# AncDec

## Anchored Decimal

A fast, precise fixed-point decimal type for `no_std` environments with **independent** integer and fractional parts.

- **AncDec8** (u8): 2-digit integer + 2-digit fraction, 4 bytes — embedded/IoT
- **AncDec32** (u32): 9-digit integer + 9-digit fraction, 12 bytes — general purpose
- **AncDec** (u64): 19-digit integer + 19-digit fraction, 24 bytes — financial
- **AncDec128** (u128): 38-digit integer + 38-digit fraction, 40 bytes — institutional

## Why AncDec?

- **Independent storage**: Integer and fraction stored separately (not shared)
- **Exact arithmetic**: No floating-point rounding errors
- **Overflow-safe**: Wide arithmetic (u256/u512) for mul/div prevents overflow
- **Fast**: Competitive with rust_decimal across all operations
- **no_std**: Zero heap allocation, embedded-friendly
- **Zero dependencies**: No external crates required (serde, sqlx optional)
- **Safe**: All public APIs return `Result`, internal panics are unreachable by design

## Why AncDec128?

AncDec128 was introduced to handle **institutional-scale financial data** (e.g., BlackRock fund data) where integer parts regularly exceed `u64::MAX` (~1.8 x 10^19). When processing total asset values, NAV calculations, or aggregated positions, u64 integer overflow was unavoidable.

AncDec128 provides:
- **38-digit integer part** (u128::MAX ~ 3.4 x 10^38)
- **38-digit fractional part** (independent)
- **Tiered fast paths** for arithmetic: u64 -> partial product -> u128 -> u256, selecting the cheapest path automatically

## Core Structures

### AncDec8 (u8) — 4 bytes
```rust
pub struct AncDec8 {
    // Fields are pub(crate) - use new() and getters
    int: u8,        // Integer part (up to 2 digits)
    frac: u8,       // Fractional part (up to 2 digits)
    scale: u8,      // Number of decimal places (0-2)
    neg: bool,      // Sign flag
}
```

### AncDec32 (u32) — 12 bytes
```rust
pub struct AncDec32 {
    // Fields are pub(crate) - use new() and getters
    int: u32,       // Integer part (up to 9 digits)
    frac: u32,      // Fractional part (up to 9 digits)
    scale: u8,      // Number of decimal places (0-9)
    neg: bool,      // Sign flag
}
```

### AncDec (u64) — 24 bytes
```rust
pub struct AncDec {
    pub int: u64,   // Integer part (up to 19 digits)
    pub frac: u64,  // Fractional part (up to 19 digits)
    pub scale: u8,  // Number of decimal places (0-19)
    pub neg: bool,  // Sign flag
}
```

### AncDec128 (u128) — 40 bytes
```rust
pub struct AncDec128 {
    // Fields are pub(crate) - use new() and getters
    int: u128,      // Integer part (up to 38 digits)
    frac: u128,     // Fractional part (up to 38 digits)
    scale: u8,      // Number of decimal places (0-38)
    neg: bool,      // Sign flag
}
```

**Why `pub(crate)` on AncDec8, AncDec32, AncDec128?**

Their arithmetic relies on the invariant `frac < 10^scale`. Fields are `pub(crate)` to enforce validation through `new()` with `debug_assert!` at zero runtime cost in release builds.

AncDec keeps `pub` fields for backward compatibility and FFI use cases.

All structs use `#[repr(C)]` layout for FFI bindings.

## Installation
```toml
[dependencies]
ancdec = "0.3"
```

**Zero dependencies** by default. All 4 types included. Only `core` is used (no `std`, no `alloc`).

Minimal embedded build (single type only):
```toml
ancdec = { version = "0.3", default-features = false, features = ["dec8"] }
```

With serde support:
```toml
ancdec = { version = "0.3", features = ["serde"] }
```

With SQLx (PostgreSQL) support:
```toml
ancdec = { version = "0.3", features = ["sqlx"] }
```

## Usage

### Construction

```rust
use ancdec::{AncDec8, AncDec32, AncDec, AncDec128};

// From string (all types)
let a: AncDec8 = "1.23".parse()?;
let b = AncDec32::parse("123456.789")?;
let c: AncDec = "456.789".parse()?;
let d = AncDec128::parse("123456789012345678901234567890.12")?;

// Validated constructor (AncDec8, AncDec32, AncDec128)
let e = AncDec8::new(1, 23, 2, false);     // 1.23
let f = AncDec32::new(123, 456, 3, false);  // 123.456
let g = AncDec128::new(123, 456, 3, false); // 123.456

// AncDec has pub fields — direct construction
let h = AncDec { int: 123, frac: 456, scale: 3, neg: false };

// From integer primitives
let i: AncDec8 = 42u8.into();            // AncDec8: i8, u8
let j: AncDec32 = 1000i32.into();        // AncDec32: i8-i32, u8-u32
let k: AncDec = 123i64.into();           // AncDec: all 12 integer types
let l: AncDec128 = u128::MAX.into();     // AncDec128: all 12 integer types

// From float (all types, fallible)
let m = AncDec::try_from(3.14f64)?;      // TryFrom<f64>
let n = AncDec8::try_from(1.5f32)?;      // TryFrom<f32>
```

### Accessors (AncDec8, AncDec32, AncDec128)

```rust
let a = AncDec32::new(123, 456, 3, true);  // -123.456
assert_eq!(a.int(), 123);
assert_eq!(a.frac(), 456);
assert_eq!(a.scale(), 3);
assert!(a.is_neg());
```

### Arithmetic

```rust
// All 4 types support: +, -, *, /, %, +=, -=, *=, /=, %=, unary -
let a: AncDec = "12.345".parse()?;
let b: AncDec = "1.2".parse()?;

let sum = a + b;          // 13.545
let diff = a - b;         // 11.145
let product = a * b;      // 14.814
let quotient = a / b;     // 10.2875
let remainder = a % b;    // 0.345
let negated = -a;         // -12.345

// Reference variants
let c = &a + &b;
let d = a + &b;
let e = &a + b;

// Assign operators
let mut x = a;
x += b;
x -= b;
x *= b;
x /= b;
x %= b;
```

### Primitive Arithmetic

```rust
// Direct arithmetic with integer primitives (no conversion needed)
let a: AncDec8 = "1.5".parse()?;
let b = a + 2u8;             // AncDec8 + u8 → AncDec8
let c = 3i8 * a;             // i8 * AncDec8 → AncDec8

let d: AncDec32 = "100.5".parse()?;
let e = d + 50i32;           // AncDec32 + i32 → AncDec32
let f = 2u16 * d;            // u16 * AncDec32 → AncDec32

let g: AncDec = "100.0".parse()?;
let h = g / 3i64;            // AncDec / i64 → AncDec
let i = 1000u128 - g;        // u128 - AncDec → AncDec

// Supported primitive types per variant:
// AncDec8:   i8, u8
// AncDec32:  i8, i16, i32, u8, u16, u32
// AncDec:    i8-i128, isize, u8-u128, usize (all 12 types)
// AncDec128: i8-i128, isize, u8-u128, usize (all 12 types)
```

### Cross-Type Arithmetic

```rust
use ancdec::{AncDec8, AncDec32, AncDec, AncDec128};

// Smaller + larger → larger type (all 5 ops: +, -, *, /, %)
let a = AncDec8::parse("1.5")?;
let b = AncDec32::parse("2.5")?;
let c: AncDec32 = a + b;             // AncDec8 + AncDec32 → AncDec32
let d: AncDec32 = b * a;             // AncDec32 * AncDec8 → AncDec32

let e: AncDec = AncDec::parse("100.0")? + a;     // AncDec + AncDec8 → AncDec
let f: AncDec128 = AncDec128::parse("1.0")? - b;  // AncDec128 - AncDec32 → AncDec128

// 6 pairs: (8↔32), (8↔64), (8↔128), (32↔64), (32↔128), (64↔128)
// Each pair: 5 ops × 2 directions = 10 impls

// Explicit widening via From (lossless)
let g: AncDec32 = AncDec32::from(a);     // AncDec8 → AncDec32
let h: AncDec = AncDec::from(a);         // AncDec8 → AncDec
let i: AncDec128 = AncDec128::from(b);   // AncDec32 → AncDec128
```

### Math

```rust
let a: AncDec = "123.456".parse()?;

// Square root (all 4 types)
// AncDec8: 1-digit fractional, AncDec32: 8-digit, AncDec: 18-digit, AncDec128: 37-digit
let root = a.sqrt();              // 11.1111075555498660

// Power (all 4 types, supports negative exponents)
let squared = a.pow(2);           // 15241.383936
let cubed = a.pow(3);             // 1881640.295202816
let inverse = a.pow(-1);          // 1 / 123.456
let one = a.pow(0);               // 1

// Sign and query (all 4 types)
let abs_val = (-a).abs();         // 123.456
let sign = a.signum();            // 1
assert!(a.is_positive());
assert!(!a.is_negative());
assert!(!a.is_zero());

// Range (all 4 types)
let b: AncDec = "200.0".parse()?;
let min_val = a.min(b);           // 123.456
let max_val = a.max(b);           // 200.0
let clamped = a.clamp(AncDec::ZERO, AncDec::from(100i64));  // 100
```

### Rounding

```rust
use ancdec::RoundMode;

// All 4 types support all 7 rounding modes
let a: AncDec = "123.456789".parse()?;

a.round(2, RoundMode::HalfUp);     // 123.46
a.round(2, RoundMode::HalfDown);   // 123.45
a.round(2, RoundMode::HalfEven);   // 123.46
a.round(2, RoundMode::Ceil);       // 123.46
a.round(2, RoundMode::Floor);      // 123.45
a.round(2, RoundMode::Truncate);   // 123.45

// Convenience methods
a.floor();                          // 123
a.ceil();                           // 124
a.trunc();                          // 123
a.fract();                          // 0.456789
```

### Conversion

```rust
// Output conversions (all 4 types)
let a: AncDec = "123.456".parse()?;
let f: f64 = a.to_f64();           // 123.456
let i: i64 = a.to_i64();           // 123
let i128: i128 = a.to_i128();      // 123

// Display with precision (all 4 types)
let s = format!("{}", a);           // "123.456"
let s2 = format!("{:.2}", a);       // "123.45"
let s0 = format!("{:.0}", a);       // "123"
```

### Iterator Support

```rust
// Sum and Product (all 4 types, owned and reference)
let values: Vec<AncDec> = vec!["1.1", "2.2", "3.3"]
    .into_iter()
    .map(|s| s.parse().unwrap())
    .collect();

let total: AncDec = values.iter().sum();      // 6.6
let product: AncDec = values.iter().product(); // 7.986
```

## Benchmarks

### All Types vs rust_decimal

| Operation | AncDec8 | AncDec32 | AncDec | AncDec128 | rust_decimal |
|-----------|---------|----------|--------|-----------|--------------|
| add       | 2.8 ns  | 4.0 ns   | 6.3 ns | 14.5 ns   | 11.4 ns      |
| sub       | 3.2 ns  | 3.9 ns   | 6.2 ns | 14.9 ns   | 11.4 ns      |
| mul       | 4.4 ns  | 4.3 ns   | 7.4 ns | 13.5 ns   | 11.1 ns      |
| div       | 3.8 ns  | 5.5 ns   | 13.4 ns| 20.3 ns   | 20.5 ns      |
| cmp       | 1.2 ns  | 3.0 ns   | 4.4 ns | 8.0 ns    | 5.1 ns       |
| parse     | 5.7 ns  | 10.5 ns  | 10.8 ns| 14.6 ns   | 10.4 ns      |

### Speedup vs rust_decimal

| Operation | AncDec8 | AncDec32 | AncDec | AncDec128 |
|-----------|---------|----------|--------|-----------|
| add       | **4.07x** | **2.85x** | **1.81x** | 0.79x |
| sub       | **3.56x** | **2.92x** | **1.84x** | 0.77x |
| mul       | **2.52x** | **2.58x** | **1.50x** | 0.82x |
| div       | **5.39x** | **3.73x** | **1.53x** | **1.01x** |
| cmp       | **4.25x** | **1.70x** | **1.16x** | 0.64x |
| parse     | **1.82x** | ~1.0x    | ~1.0x    | 0.71x |

### AncDec128 High Precision

| Operation | AncDec128 | rust_decimal | Ratio |
|-----------|-----------|--------------|-------|
| mul       | 19.6 ns   | 18.3 ns      | 1.07x |
| div       | 54.5 ns   | 55.8 ns      | **0.98x** |
| parse     | 34.7 ns   | 30.3 ns      | 1.15x |

AncDec128 is comparable to rust_decimal while supporting **38+38 digit precision** vs rust_decimal's 28 shared digits.

*Benchmarked on Intel Core i7-10750H @ 2.60GHz, Rust 1.87.0, release mode*

## Performance Architecture

### AncDec Multiplication Fast Path

AncDec combines `int` and `frac` into a single u128 value (`int × 10^scale + frac`) before multiplication. When both combined values exceed u64 range, this requires u256 wide arithmetic (`mul_wide`). To avoid this overhead for common cases:

```
combined = int × 10^scale + frac

if combined ≤ u64::MAX for both operands:
    → cast to u64, multiply natively (u64 × u64 → u128)     ~7 ns
else:
    → mul_wide (u128 × u128 → u256) + div_wide              ~21 ns
```

**Why this is safe:** The fast path guard `a ≤ u64::MAX && b ≤ u64::MAX` guarantees the product fits in u128, because `(2⁶⁴ - 1)² = 2¹²⁸ - 2⁶⁵ + 1 < 2¹²⁸ - 1 = u128::MAX`. The operands are explicitly cast down to u64 before multiplication to make the intent unambiguous: `(a as u64 as u128) * (b as u64 as u128)`. When the combined value exceeds u64 range (e.g., `int=10^18, scale=19` → combined ≈ 10^37), the condition fails and execution falls through to `mul_wide` which handles the full u128×u128→u256 range safely.

### AncDec128 Tiered Fast Paths

AncDec128 automatically selects the fastest arithmetic path based on operand size:

**Multiplication:**
| Tier | Condition | Method | Cost |
|------|-----------|--------|------|
| 1 | Both fit in u64 combined | Native u64 x u64 | ~15 ns |
| 2 | Parts fit in u64, scale <= 19 | 4x partial product | ~25 ns |
| 3 | Both fit in u128 combined | `mul_wide` + `divmod_u256` | ~45 ns |
| 4 | Everything else | Full u256 x u256 -> u512 | ~80 ns |

**Division:**
| Tier | Condition | Method | Cost |
|------|-----------|--------|------|
| 1 | Both fit in u64 combined | Algebraic decomposition | ~25 ns |
| 2 | Both fit in u128 combined | `mul_wide` + `divmod_u256` | ~40 ns |
| 3 | Everything else | Full u512 / u256 | ~80 ns |

### Performance Cliffs

Performance drops when operands exceed a tier's threshold:

| Trigger | Effect | Typical cause |
|---------|--------|---------------|
| `scale > 19` | Skips u64 and partial product tiers | `div()` produces `scale=38` |
| `int > u64::MAX` | Skips u64 and partial product tiers | Large aggregated values |
| `int * 10^scale + frac > u128::MAX` | Falls to u256 slow path | High-scale large values |

**Common pattern:** `a.div(&b).mul(&c)` -- division produces `scale=38`, forcing subsequent multiplication into the u128 or u256 path. This is inherent to the split int/frac representation, not a bug.

## Precision Limits

| | AncDec8 (u8) | AncDec32 (u32) | AncDec (u64) | AncDec128 (u128) |
|---|---|---|---|---|
| Integer part | 2 digits | 9 digits | 19 digits | 38 digits |
| Fractional part | 2 digits | 9 digits | 19 digits | 38 digits |
| Total precision | 4 digits | 18 digits | 38 digits | 76 digits |
| sqrt() precision | 1 digit | 8 digits | 18 digits | 37 digits |
| Scale range | 0-2 | 0-9 | 0-19 | 0-38 |
| Struct size | 4 bytes | 12 bytes | 24 bytes | 40 bytes |

Fractional digits beyond the limit are truncated during parsing. Integer parts saturate at `MAX`.

## Complete API Reference

### Methods (all 4 types)

| Category | Methods |
|----------|---------|
| Construction | `parse(T)`, `new(int, frac, scale, neg)` (8/32/128), direct fields (AncDec) |
| Accessors | `int()`, `frac()`, `scale()`, `is_neg()` (8/32/128) |
| Arithmetic | `add`, `sub`, `mul`, `div`, `rem`, `checked_add`, `checked_sub`, `checked_mul` |
| Math | `sqrt()`, `pow(i32)`, `abs()`, `signum()` |
| Query | `is_zero()`, `is_positive()`, `is_negative()` |
| Range | `min()`, `max()`, `clamp()` |
| Rounding | `round(places, mode)`, `floor()`, `ceil()`, `trunc()`, `fract()` |
| Conversion | `to_f64()`, `to_i64()`, `to_i128()` |

### Operator Traits (all 4 types)

| Trait | Operators | Variants |
|-------|-----------|----------|
| `Add`, `Sub`, `Mul`, `Div`, `Rem` | `+`, `-`, `*`, `/`, `%` | value, `&ref`, cross-type, primitives |
| `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`, `RemAssign` | `+=`, `-=`, `*=`, `/=`, `%=` | |
| `Neg` | `-a` | value, `&ref` |

### Conversion Traits

| Trait | AncDec8 | AncDec32 | AncDec | AncDec128 |
|-------|---------|----------|--------|-----------|
| `From<i8>`, `From<u8>` | Yes | Yes | Yes | Yes |
| `From<i16>`, `From<u16>` | — | Yes | Yes | Yes |
| `From<i32>`, `From<u32>` | — | Yes | Yes | Yes |
| `From<i64>`, `From<u64>` | — | — | Yes | Yes |
| `From<i128>`, `From<u128>` | — | — | Yes | Yes |
| `From<isize>`, `From<usize>` | — | — | Yes | Yes |
| `TryFrom<f32>`, `TryFrom<f64>` | Yes | Yes | Yes | Yes |
| `TryFrom<&str>`, `FromStr` | Yes | Yes | Yes | Yes |

### Widening From (lossless, cfg-gated)

```
AncDec8 → AncDec32 → AncDec → AncDec128
```

| From | To AncDec32 | To AncDec | To AncDec128 |
|------|-------------|-----------|--------------|
| AncDec8 | Yes | Yes | Yes |
| AncDec32 | — | Yes | Yes |
| AncDec | — | — | Yes |

### Primitive Arithmetic

| Type | Supported primitives for `+`, `-`, `*`, `/` (both directions) |
|------|--------------------------------------------------------------|
| AncDec8 | `i8`, `u8` |
| AncDec32 | `i8`, `i16`, `i32`, `u8`, `u16`, `u32` |
| AncDec | `i8`-`i128`, `isize`, `u8`-`u128`, `usize` (12 types) |
| AncDec128 | `i8`-`i128`, `isize`, `u8`-`u128`, `usize` (12 types) |

### Cross-Type Arithmetic (cfg-gated)

All 5 operators (`+`, `-`, `*`, `/`, `%`) in both directions. Output = larger type.

| Pair | Output | Feature gate |
|------|--------|-------------|
| AncDec8 ↔ AncDec32 | AncDec32 | `dec8` + `dec32` |
| AncDec8 ↔ AncDec | AncDec | `dec8` + `dec64` |
| AncDec8 ↔ AncDec128 | AncDec128 | `dec8` + `dec128` |
| AncDec32 ↔ AncDec | AncDec | `dec32` + `dec64` |
| AncDec32 ↔ AncDec128 | AncDec128 | `dec32` + `dec128` |
| AncDec ↔ AncDec128 | AncDec128 | `dec64` + `dec128` |

### Other Traits (all 4 types)

| Trait | Notes |
|-------|-------|
| `PartialEq`, `Eq` | `0 == -0`, trailing zeros normalized |
| `PartialOrd`, `Ord` | Total ordering |
| `Hash` | Normalized (trailing zeros, `0 == -0`), usable in `HashMap`/`HashSet` |
| `Clone`, `Copy`, `Debug` | Derived |
| `Default` | Returns `ZERO` |
| `Display` | Precision support: `format!("{:.2}", a)` |
| `Sum`, `Product` | Iterator support (owned + reference) |
| `Serialize`, `Deserialize` | String-based, with `serde` feature |

### Constants

```rust
AncDec8::ZERO      AncDec32::ZERO      AncDec::ZERO       AncDec128::ZERO
AncDec8::ONE       AncDec32::ONE       AncDec::ONE        AncDec128::ONE
AncDec8::TWO       AncDec32::TWO       AncDec::TWO        AncDec128::TWO
AncDec8::TEN       AncDec32::TEN       AncDec::TEN        AncDec128::TEN
AncDec8::MAX       AncDec32::MAX       AncDec::MAX        AncDec128::MAX
```

## Features

| Feature | Dependencies | Description |
|---------|--------------|-------------|
| (default) | **None** | All 4 types, only uses `core` |
| `dec8` | — | AncDec8 only |
| `dec32` | — | AncDec32 only |
| `dec64` | — | AncDec only |
| `dec128` | — | AncDec128 only |
| `serde` | `serde` | Serialization for all enabled types |
| `sqlx` | `sqlx`, `std` | PostgreSQL NUMERIC (AncDec only) |

### Serde

All types serialize as decimal strings:
```rust
#[derive(Serialize, Deserialize)]
struct Position {
    sensor: AncDec8,
    price: AncDec,
    total_value: AncDec128,
}
// JSON: {"sensor": "1.23", "price": "123.456", "total_value": "12345678901234567890.123456"}
```

### SQLx (AncDec only)

PostgreSQL NUMERIC binary wire protocol for `AncDec` only. Implements `Type<Postgres>`, `Encode<Postgres>`, `Decode<Postgres>`.

```rust
let price: AncDec = "123.456".parse()?;
sqlx::query("INSERT INTO products (price) VALUES ($1)")
    .bind(&price)
    .execute(&pool)
    .await?;

let row: AncDec = sqlx::query_scalar("SELECT price FROM products")
    .fetch_one(&pool)
    .await?;
```

## Safety Design

### Public API - Always Safe

All public APIs return `Result` for fallible operations. Integer conversions via `From` are infallible. `checked_add`, `checked_sub`, `checked_mul` return `Option<Self>` for overflow-safe arithmetic.

### Invariant Enforcement

AncDec8, AncDec32, and AncDec128 enforce `frac < 10^scale` through:
- **`pub(crate)` fields** -- external code must use `new()` or `parse()`
- **`debug_assert!` in `new()`** -- catches violations in debug builds at zero release cost
- **All arithmetic preserves the invariant** -- internal construction is trusted

### Division by Zero

Division by zero panics (consistent with Rust's integer division). Use `is_zero()` to check before division.

## Comparison with Alternatives

| Feature | AncDec8 | AncDec32 | AncDec | AncDec128 | rust_decimal | f64 |
|---------|---------|----------|--------|-----------|--------------|-----|
| Integer precision | 2 digits | 9 digits | 19 digits | 38 digits | 28 shared | ~15 shared |
| Fractional precision | 2 digits | 9 digits | 19 digits | 38 digits | 28 shared | ~15 shared |
| Exact decimal | Yes | Yes | Yes | Yes | Yes | No |
| no_std | Yes | Yes | Yes | Yes | Feature flag | Yes |
| Zero dependencies | Yes | Yes | Yes | Yes | No | Yes |
| FFI-friendly | Yes | Yes | Yes | Yes | No | Yes |
| Struct size | 4 bytes | 12 bytes | 24 bytes | 40 bytes | 16 bytes | 8 bytes |

## Changelog

### v0.3.0

**New Types:**
- `AncDec8` (u8): 4-byte decimal for embedded/IoT (2+2 digit precision)
- `AncDec32` (u32): 12-byte decimal for general purpose (9+9 digit precision)

**New Features:**
- Feature flags (`dec8`, `dec32`, `dec64`, `dec128`) for selective compilation
- Cross-type arithmetic: `AncDec8 + AncDec32 → AncDec32` (automatic widening)
- Widening conversions via `From`: `AncDec8 → AncDec32 → AncDec → AncDec128`
- Serde support for all 4 types
- `sqrt()` for AncDec (18-digit fractional precision via Newton-Raphson on u256)
- `sqrt()` for AncDec128 (37-digit fractional precision via Newton-Raphson on u512)
- `checked_add`, `checked_sub`, `checked_mul` for all 4 types (returns `Option`)

**Breaking Changes:**
- Default features changed from none to `["dec8", "dec32", "dec64", "dec128"]`
  - Existing code compiles unchanged (all types enabled by default)
  - `default-features = false` now requires explicit feature selection
- `AncDec128` fields changed from `pub` to `pub(crate)`
  - Use `AncDec128::new(int, frac, scale, neg)` for construction
  - Use `.int()`, `.frac()`, `.scale()`, `.is_neg()` for field access
  - Enforces `frac < 10^scale` invariant via `debug_assert!`

**Bug Fixes:**
- Fixed `mul_wide` overflow in debug mode (`hl + lh` → `wrapping_add`)

**Performance:**
- AncDec mul: u64 fast path bypasses `mul_wide` when both operands fit in u64 (-65%)
- AncDec128 mul: partial product fast path for u64-sized operands (-45% for high precision)
- AncDec128 mul: u64 ultra-fast path for small operands (-10%)
- AncDec128 div: algebraic decomposition for u64 operands (-33%)
- AncDec128 div: u128 fast path avoiding full u256 arithmetic (-20%)
- AncDec128 sub: branchless `borrow * limit` pattern (-37%)
- AncDec128 add: branchless `overflow * limit` pattern (-17%)
- AncDec128 parse: two-stage u64/u128 accumulator with stage 2 gating (-18%)

### v0.2.0

- Added serde serialization/deserialization support
- Added SQLx PostgreSQL support
- Fixed mul/div overflow with u256 wide arithmetic

### v0.1.0

- Initial release with AncDec (u64-based decimal)

## License

MIT License

## Contributing

Contributions welcome! Please ensure:
- All tests pass (`cargo test`)
- Individual type tests pass (`cargo test --no-default-features --features dec8`)
- Serde tests pass (`cargo test --features serde`)
- Benchmarks don't regress (`cargo bench`)
- Code follows existing style
