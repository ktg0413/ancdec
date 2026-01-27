# AncDec

## Anchored Decimal

A fast, precise fixed-point decimal type for `no_std` environments with **independent** 19-digit integer and 19-digit fractional parts.

## Why AncDec?

### AncDec's Solution

- **Independent storage**: 19-digit integer + 19-digit fraction (not shared)
- **Exact arithmetic**: No floating-point rounding errors
- **Overflow-safe**: Wide arithmetic (u256) for mul/div prevents overflow
- **Fast**: 1.2x - 1.8x faster than rust_decimal
- **no_std**: Zero heap allocation, embedded-friendly
- **Zero dependencies**: No external crates required (serde optional)
- **Safe**: All public APIs return `Result`, internal panics are unreachable by design

## Core Structure
```rust
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AncDec {
    pub int: u64,   // Integer part (up to 19 digits)
    pub frac: u64,  // Fractional part (up to 19 digits)
    pub scale: u8,  // Number of decimal places (0-19)
    pub neg: bool,  // Sign flag
}
```

The `#[repr(C)]` layout enables easy FFI bindings for other languages.

## Installation
```toml
[dependencies]
ancdec = "0.1"
```

**Zero dependencies** by default. Only `core` is used (no `std`, no `alloc`).

With serde support:
```toml
[dependencies]
ancdec = { version = "0.1", features = ["serde"] }
```

## Usage

### Creating AncDec values
```rust
use ancdec::AncDec;

// From Rust integer primitives via From trait (infallible)
let a: AncDec = 123i64.into();
let b: AncDec = (-456i32).into();

// From string via parse (returns Result)
let c: AncDec = "456.789".parse()?;
let d = AncDec::parse("123.456")?;

// TryFrom for &str and floats
let e = AncDec::try_from("789.012")?;
let f = AncDec::try_from(3.14f64)?;
```

### Arithmetic
```rust
let a: AncDec = "123.456".parse()?;
let b: AncDec = "78.9".parse()?;

let sum = a + b;        // 202.356
let diff = a - b;       // 44.556
let product = a * b;    // 9740.6784
let quotient = a / b;   // 1.5647148288973384030
let remainder = a % b;  // 44.556
```

### Comparison
```rust
let a: AncDec = "100.5".parse()?;
let b: AncDec = "100.50".parse()?;

assert!(a == b);  // true (trailing zeros normalized)
assert!(a >= b);
```

### Rounding
```rust
use ancdec::RoundMode;

let a: AncDec = "123.456789".parse()?;

a.round(2, RoundMode::HalfUp);   // 123.46
a.round(0, RoundMode::Floor);    // 123
a.floor();                        // 123
a.ceil();                         // 124
a.trunc();                        // 123
a.fract();                        // 0.456789
```

### Conversion
```rust
let a: AncDec = "123.456".parse()?;

let f: f64 = a.to_f64();      // 123.456
let i: i64 = a.to_i64();      // 123
let i128: i128 = a.to_i128(); // 123
```

## Supported Types & Traits

### From Trait (Infallible)

Integer conversions never fail:

| Type | Example |
|------|---------|
| `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `(-123i64).into()` |
| `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `123u64.into()` |

### TryFrom / Parse (Fallible)

String and float conversions return `Result<AncDec, ParseError>`:
```rust
// FromStr trait
let a: AncDec = "123.456".parse()?;

// TryFrom<&str>
let b = AncDec::try_from("123.456")?;

// TryFrom<f64> - rejects NaN and Infinity
let c = AncDec::try_from(3.14f64)?;
let err = AncDec::try_from(f64::NAN);  // Err(InvalidFloat)

// parse method (works with any Display type)
let d = AncDec::parse("123.456")?;
let e = AncDec::parse(my_custom_type)?;
```

### ParseError

| Variant | Description |
|---------|-------------|
| `Empty` | Input string is empty |
| `NoDigits` | No valid digits found |
| `TrailingChars` | Invalid characters after number |
| `InvalidFloat` | NaN or Infinity |

### Operator Traits

| Trait | Operators |
|-------|-----------|
| `Add`, `Sub`, `Mul`, `Div`, `Rem` | `+`, `-`, `*`, `/`, `%` |
| `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`, `RemAssign` | `+=`, `-=`, `*=`, `/=`, `%=` |
| `Neg` | `-a` |

All operators work with `AncDec`, `&AncDec`, and integer primitives:
```rust
let a: AncDec = 10i64.into();
let b = a + 5;          // AncDec + i32
let c = 3 * a;          // i32 * AncDec
let d = &a + &a;        // &AncDec + &AncDec
```

### Comparison Traits

| Trait | Usage |
|-------|-------|
| `PartialEq`, `Eq` | `==`, `!=` |
| `PartialOrd`, `Ord` | `<`, `>`, `<=`, `>=`, `.cmp()` |

### Other Traits

| Trait | Usage |
|-------|-------|
| `Default` | `AncDec::default()` returns `ZERO` |
| `Hash` | Usable in `HashMap`, `HashSet` |
| `FromStr` | `"123.45".parse::<AncDec>()` |
| `TryFrom<&str>` | `AncDec::try_from("123.45")` |
| `TryFrom<f32>` | `AncDec::try_from(3.14f32)` |
| `TryFrom<f64>` | `AncDec::try_from(3.14f64)` |
| `Display` | `format!("{}", a)`, `format!("{:.2}", a)` |
| `Sum` | `iter.sum::<AncDec>()` |
| `Product` | `iter.product::<AncDec>()` |

### Constants
```rust
AncDec::ZERO   // 0
AncDec::ONE    // 1
AncDec::TWO    // 2
AncDec::TEN    // 10
AncDec::MAX    // Maximum representable value
```

### Utility Methods
```rust
let a: AncDec = "-123.456".parse()?;

a.abs();          // 123.456
a.signum();       // -1
a.is_positive();  // false
a.is_negative();  // true
a.is_zero();      // false
a.min(b);         // minimum of a and b
a.max(b);         // maximum of a and b
a.clamp(lo, hi);  // clamp to range
a.pow(3);         // a³ (binary exponentiation)
```

## Safety Design

AncDec is designed with a layered safety model:

### Public API - Always Safe

All public APIs return `Result` for fallible operations:

| Function | Return Type | Failure Case |
|----------|-------------|--------------|
| `"123".parse::<AncDec>()` | `Result<AncDec, ParseError>` | Invalid input |
| `AncDec::try_from("123")` | `Result<AncDec, ParseError>` | Invalid input |
| `AncDec::try_from(3.14f64)` | `Result<AncDec, ParseError>` | NaN, Infinity |
| `AncDec::parse(value)` | `Result<AncDec, ParseError>` | Invalid input |

Integer conversions via `From` trait are infallible by design.

### Internal Panic - Unreachable by Design

Internal functions like `pow10(exp)` contain panic for out-of-range values:
```rust
// Internal only (pub(crate))
const fn pow10(exp: u8) -> u64 {
    match exp {
        0..=19 => { /* valid values */ },
        _ => panic!("scale overflow"),  // Unreachable
    }
}
```

**Why this is safe:**

1. **`pow10` is `pub(crate)`** - External code cannot call it directly
2. **Scale is bounded at parse time** - Fractional digits are truncated to 19
3. **Arithmetic preserves bounds** - All operations maintain `scale ≤ 19`
4. **The panic exists only to catch internal bugs** - If reached, it indicates a logic error in the library itself

This design provides:
- **Zero runtime overhead** for range checks in hot paths
- **Compile-time guarantees** through type system
- **Defense in depth** against internal bugs

### Division by Zero

Division by zero panics (consistent with Rust's integer division):
```rust
let a: AncDec = 10i64.into();
let b = AncDec::ZERO;
let c = a / b;  // Panics
```

Use `is_zero()` to check before division if needed.

## Performance Optimizations

### Compile-time Power of 10

Power of 10 values are `const fn` with match expressions:
- **Compile-time evaluation** when exponent is constant
- **Zero heap allocation**
- **LLVM optimizes to jump table or direct substitution**

### Aggressive Inlining

All hot-path functions are `#[inline(always)]`:
- Arithmetic operations
- Comparisons
- Conversions

## Benchmarks

Compared against `rust_decimal` (lower is better):

| Operation | AncDec | rust_decimal | Speedup |
|-----------|--------|--------------|---------|
| add       | 6.3 ns | 11.1 ns      | **1.76x** |
| sub       | 6.3 ns | 11.3 ns      | **1.79x** |
| mul       | 9.5 ns | 11.4 ns      | **1.20x** |
| div       | 13.7 ns | 19.8 ns     | **1.45x** |
| cmp       | 4.5 ns | 5.3 ns       | **1.18x** |
| parse     | 11.3 ns | 11.4 ns     | **1.01x** |

*Benchmarked on Intel Core i7-10750H @ 2.60GHz, Rust 1.87.0, release mode*
*Benchmarked on Intel Core i7-10750H @ 2.60GHz, Rust 1.60.0, release mode*

## Precision Limits

| Component | Limit | Note |
|-----------|-------|------|
| Integer part | 19 digits | `u64::MAX` ≈ 1.8 × 10¹⁹ |
| Fractional part | 19 digits | Independent of integer |
| Total precision | 38 digits | Not shared like rust_decimal |

### Precision Behavior

- **Parsing**: Fractional digits beyond 19 are truncated, integer part saturates at `u64::MAX`
- **Multiplication**: Results clamped to 19 decimal places
- **Division**: Results have exactly 19 decimal places
```rust
// Fractional truncation during parse
let a: AncDec = "1.1234567890123456789999".parse()?;
// Stored as 1.1234567890123456789 (19 digits)

// Integer saturation during parse
let b: AncDec = "99999999999999999999".parse()?;
// Stored as u64::MAX (18446744073709551615)

// Multiplication precision
let c: AncDec = "0.1234567890123456789".parse()?;
let d = c * c;  // Result has 19 decimal places
```

## Features

| Feature | Dependencies | Description |
|---------|--------------|-------------|
| (default) | **None** | Core functionality, only uses `core` |
| `serde` | `serde` | Serialization/deserialization support |

### Serde

When enabled, AncDec serializes as decimal string:
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Price {
    amount: AncDec,
}

// JSON: {"amount": "123.456"}
```

## Use Cases

- **Finance**: Exact monetary calculations
- **Accounting**: Tax, invoicing, ledger systems
- **Cryptocurrency**: Wei/Satoshi precision arithmetic
- **Embedded**: Payment terminals, IoT devices (no_std, zero dependencies)
- **Games**: In-game currency systems
- **Scientific**: When exact decimal representation matters
- 
## Comparison with Alternatives

| Feature | AncDec | rust_decimal | f64 |
|---------|--------|--------------|-----|
| Precision | 19+19 digits (independent) | 28 digits (shared) | ~15 digits (shared) |
| Exact decimal | ✅ | ✅ | ❌ |
| Overflow handling | ✅ (u256 wide arithmetic) | ✅ | ❌ (silent) |
| no_std | ✅ | ⚠️ (feature flag) | ✅ |
| Zero dependencies | ✅ | ❌ | ✅ |
| Speed (vs rust_decimal) | **1.2-1.8x faster** | baseline | ~10x faster |
| FFI-friendly | ✅ (`repr(C)`) | ❌ | ✅ |
| Size | 24 bytes | 16 bytes | 8 bytes |

## License

MIT License

## Contributing

Contributions welcome! Please ensure:
- All tests pass (`cargo test`)
- Serde tests pass (`cargo test --features serde`)
- Benchmarks don't regress (`cargo bench`)
- Code follows existing style
