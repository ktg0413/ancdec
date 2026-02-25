mod arithmetic;
mod basic;
mod cmp;
mod convert;
mod fmt_impl;
mod ops;
mod rounding;

#[cfg(feature = "serde")]
mod serde_impl;

#[cfg(feature = "sqlx")]
mod sqlx_impl;

use crate::error::ParseError;
use crate::util::{pow10, pow10_128, StackBuf, TARGET_SCALE};
use core::fmt::{Display, Write};

/// 64-bit fixed-point decimal (u64 int/frac, 19-digit precision, 24 bytes).
///
/// Stores integer and fractional parts as separate `u64` values with an explicit scale (0-19).
/// Fields are `pub` for backward compatibility and FFI use.
///
/// # Example
/// ```
/// use ancdec::AncDec;
/// let a: AncDec = "123.456".parse().unwrap();
/// assert_eq!(a.int, 123);
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AncDec {
    /// Integer part (0 to `u64::MAX`).
    pub int: u64,
    /// Fractional part (0 to `10^scale - 1`).
    pub frac: u64,
    /// Number of fractional digits (0-19).
    pub scale: u8,
    /// Sign flag: `true` if negative.
    pub neg: bool,
}

// ============ Constants ============
impl AncDec {
    /// The value `0`.
    pub const ZERO: AncDec = AncDec {
        int: 0,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `1`.
    pub const ONE: AncDec = AncDec {
        int: 1,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `2`.
    pub const TWO: AncDec = AncDec {
        int: 2,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `10`.
    pub const TEN: AncDec = AncDec {
        int: 10,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The maximum representable value.
    pub const MAX: AncDec = AncDec {
        int: u64::MAX,
        frac: 9_999_999_999_999_999_999,
        scale: 19,
        neg: false,
    };
}

// ============ Core Methods ============
impl AncDec {
    /// Parses any `Display` type into an `AncDec` using a stack buffer (no heap allocation).
    pub fn parse<T: Display>(value: T) -> Result<Self, ParseError> {
        let mut buf = StackBuf::<64>::new();
        write!(buf, "{}", value).ok();
        Self::parse_str(buf.as_str())
    }

    /// Byte-level string parsing with validation
    pub(crate) fn parse_str(s: &str) -> Result<Self, ParseError> {
        let b = s.as_bytes();
        let len = b.len();
        if len == 0 {
            return Err(ParseError::Empty);
        }

        let mut i = 0;

        let neg = unsafe { *b.get_unchecked(0) } == b'-';
        i += neg as usize;

        if i >= len {
            return Err(ParseError::NoDigits);
        }

        // parse integer part (error on overflow)
        let mut int: u64 = 0;
        let mut has_digits = false;
        while i < len {
            let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
            if d > 9 {
                break;
            }
            has_digits = true;
            int = int.checked_mul(10).and_then(|v| v.checked_add(d as u64))
                .ok_or(ParseError::Overflow)?;
            i += 1;
        }

        // skip '.'
        if i < len && unsafe { *b.get_unchecked(i) } == b'.' {
            i += 1;
        }

        // parse fractional part (truncate at 19 digits)
        let mut frac: u64 = 0;
        let mut frac_digits: u8 = 0;
        while i < len {
            let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
            if d > 9 {
                break;
            }
            if frac_digits < TARGET_SCALE {
                frac = frac * 10 + d as u64;
                frac_digits += 1;
            }
            i += 1;
        }

        if !has_digits && frac_digits == 0 {
            return Err(ParseError::NoDigits);
        }

        if i != len {
            return Err(ParseError::TrailingChars);
        }

        Ok(Self {
            int,
            frac,
            scale: frac_digits,
            neg,
        })
    }

    /// Align fractional parts to same scale, returns (self_frac, other_frac, scale, limit)
    #[inline(always)]
    pub(crate) fn align_frac(&self, other: &Self) -> (u64, u64, u8, u64) {
        if self.scale == other.scale {
            (self.frac, other.frac, self.scale, pow10(self.scale))
        } else if self.scale > other.scale {
            let limit = pow10(self.scale);
            (
                self.frac,
                // SAFETY: frac < 10^scale by invariant, so frac * 10^delta < 10^19 < u64::MAX
                other.frac * pow10(self.scale - other.scale),
                self.scale,
                limit,
            )
        } else {
            let limit = pow10(other.scale);
            (
                self.frac * pow10(other.scale - self.scale),
                other.frac,
                other.scale,
                limit,
            )
        }
    }

    /// Add aligned values (same sign), handles frac overflow
    #[inline(always)]
    pub(crate) fn add_aligned(
        a_int: u64,
        a_frac: u64,
        b_int: u64,
        b_frac: u64,
        scale: u8,
        limit: u64,
    ) -> (u64, u64, u8) {
        // Use u128 for frac sum since u64 max frac (10^19-1) * 2 can overflow u64
        let frac_wide = a_frac as u128 + b_frac as u128;
        let overflow = (frac_wide >= limit as u128) as u64;
        let frac = (frac_wide - overflow as u128 * limit as u128) as u64;
        let int = a_int.checked_add(b_int)
            .and_then(|v| v.checked_add(overflow))
            .expect("integer overflow in addition");
        (int, frac, scale)
    }

    /// Subtract with magnitude comparison, returns result with correct sign
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn sub_with_cmp(
        a_int: u64,
        a_frac: u64,
        a_neg: bool,
        b_int: u64,
        b_frac: u64,
        b_neg: bool,
        scale: u8,
        limit: u64,
    ) -> Self {
        if (a_int, a_frac) >= (b_int, b_frac) {
            let borrow = (a_frac < b_frac) as u64;
            Self {
                int: a_int - b_int - borrow,
                frac: a_frac.wrapping_sub(b_frac).wrapping_add(borrow * limit),
                scale,
                neg: a_neg,
            }
        } else {
            let borrow = (b_frac < a_frac) as u64;
            Self {
                int: b_int - a_int - borrow,
                frac: b_frac.wrapping_sub(a_frac).wrapping_add(borrow * limit),
                scale,
                neg: b_neg,
            }
        }
    }

    /// Split combined u128 back to int/frac
    #[inline(always)]
    pub(crate) fn from_combined(n: u128, scale: u8, neg: bool) -> Self {
        if scale == 0 {
            assert!(n <= u64::MAX as u128, "integer overflow in from_combined");
            return Self {
                int: n as u64,
                frac: 0,
                scale: 0,
                neg,
            };
        }
        let divisor = pow10_128(scale);
        let int_part = n / divisor;
        assert!(int_part <= u64::MAX as u128, "integer overflow in from_combined");
        Self {
            int: int_part as u64,
            frac: (n % divisor) as u64,
            scale,
            neg,
        }
    }
}
