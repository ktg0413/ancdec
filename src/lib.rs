//! A `#![no_std]` fixed-point decimal library with four types for different precision/size needs.
//!
//! | Type | Integer/Frac | Scale | Size |
//! |---|---|---|---|
//! | [`AncDec8`] | `u8` | 0-2 | 4 bytes |
//! | [`AncDec32`] | `u32` | 0-9 | 12 bytes |
//! | [`AncDec`] | `u64` | 0-19 | 24 bytes |
//! | [`AncDec128`] | `u128` | 0-38 | 40 bytes |
//!
//! All types store integer and fractional parts separately with an explicit scale,
//! avoiding the precision loss inherent in floating-point representations.
//!
//! # Feature flags
//!
//! - **`dec8`** / **`dec32`** / **`dec64`** / **`dec128`** -- enable individual types (all on by default)
//! - **`serde`** -- string-based `Serialize`/`Deserialize`
//! - **`sqlx`** -- PostgreSQL `NUMERIC` support (implies `std` + `dec64`)

#![no_std]

#[cfg(feature = "std")]
extern crate std;

// Shared modules (always compiled)
mod error;
mod round_mode;
mod util;

pub use error::ParseError;
pub use round_mode::RoundMode;

// Wide arithmetic: needed by dec32 (isqrt_u128), dec64 and dec128
#[cfg(any(feature = "dec32", feature = "dec64", feature = "dec128"))]
pub(crate) mod wide;

// ============ AncDec8 (u8) ============
#[cfg(feature = "dec8")]
mod ancdec8;
#[cfg(feature = "dec8")]
pub use ancdec8::AncDec8;

// ============ AncDec32 (u32) ============
#[cfg(feature = "dec32")]
mod ancdec32;
#[cfg(feature = "dec32")]
pub use ancdec32::AncDec32;

// ============ AncDec (u64) ============
#[cfg(feature = "dec64")]
mod ancdec;
#[cfg(feature = "dec64")]
pub use ancdec::AncDec;

// ============ AncDec128 (u128) ============
#[cfg(feature = "dec128")]
mod ancdec128;
#[cfg(feature = "dec128")]
pub use ancdec128::AncDec128;

// ============ Cross-type operations ============
mod cross_ops;
