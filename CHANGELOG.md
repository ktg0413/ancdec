# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-02-24

### Added
- `AncDec8` (u8): 4-byte decimal for embedded/IoT (2+2 digit precision)
- `AncDec32` (u32): 12-byte decimal for general purpose (9+9 digit precision)
- Feature flags (`dec8`, `dec32`, `dec64`, `dec128`) for selective compilation
- Cross-type arithmetic: `AncDec8 + AncDec32 → AncDec32` (automatic widening)
- Widening conversions via `From`: `AncDec8 → AncDec32 → AncDec → AncDec128`
- Serde support for all 4 types
- `sqrt()` for AncDec (18-digit fractional precision via Newton-Raphson on u256)
- `sqrt()` for AncDec128 (37-digit fractional precision via Newton-Raphson on u512)
- `checked_add`, `checked_sub`, `checked_mul` methods for all types

### Changed
- Default features changed from none to `["dec8", "dec32", "dec64", "dec128"]`
  - Existing code compiles unchanged (all types enabled by default)
  - `default-features = false` now requires explicit feature selection
- `AncDec128` fields changed from `pub` to `pub(crate)`
  - Use `AncDec128::new(int, frac, scale, neg)` for construction
  - Use `.int()`, `.frac()`, `.scale()`, `.is_neg()` for field access

### Fixed
- `mul_wide` overflow in debug mode (`hl + lh` → `wrapping_add`)

### Performance
- AncDec mul: u64 fast path bypasses `mul_wide` when both operands fit in u64 (-65%)
- AncDec128 mul: partial product fast path for u64-sized operands (-45% for high precision)
- AncDec128 mul: u64 ultra-fast path for small operands (-10%)
- AncDec128 div: algebraic decomposition for u64 operands (-33%)
- AncDec128 div: u128 fast path avoiding full u256 arithmetic (-20%)
- AncDec128 sub: branchless `borrow * limit` pattern (-37%)
- AncDec128 add: branchless `overflow * limit` pattern (-17%)
- AncDec128 parse: two-stage u64/u128 accumulator with stage 2 gating (-18%)

## [0.2.0] - 2025-01-15

### Added
- Serde serialization/deserialization support
- SQLx PostgreSQL NUMERIC support

### Fixed
- mul/div overflow with u256 wide arithmetic

## [0.1.0] - 2025-01-01

### Added
- Initial release with AncDec (u64-based decimal)
- 19-digit integer + 19-digit fractional precision
- Basic arithmetic (add, sub, mul, div, rem)
- Comparison, rounding, conversion traits
- `#![no_std]` support
