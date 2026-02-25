mod arithmetic;
mod basic;
mod cmp;
mod convert;
mod fmt_impl;
mod ops;
mod rounding;

#[cfg(feature = "serde")]
mod serde_impl;

use crate::error::ParseError;
use crate::util::{pow10, pow10_128, StackBuf, TARGET_SCALE_128};
use crate::wide::divmod_u256;
use core::fmt::{Display, Write};

/// 128-bit fixed-point decimal (u128 int/frac, 38-digit precision, 40 bytes).
///
/// Stores integer and fractional parts as separate `u128` values with an explicit scale (0-38).
///
/// # Example
/// ```
/// use ancdec::AncDec128;
/// let a: AncDec128 = "123.456".parse().unwrap();
/// assert_eq!(a.int(), 123);
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AncDec128 {
    pub(crate) int: u128,
    pub(crate) frac: u128,
    pub(crate) scale: u8,
    pub(crate) neg: bool,
}

// ============ Constants ============
impl AncDec128 {
    /// The value `0`.
    pub const ZERO: AncDec128 = AncDec128 {
        int: 0,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `1`.
    pub const ONE: AncDec128 = AncDec128 {
        int: 1,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `2`.
    pub const TWO: AncDec128 = AncDec128 {
        int: 2,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `10`.
    pub const TEN: AncDec128 = AncDec128 {
        int: 10,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The maximum representable value.
    pub const MAX: AncDec128 = AncDec128 {
        int: u128::MAX,
        frac: 99_999_999_999_999_999_999_999_999_999_999_999_999,
        scale: 38,
        neg: false,
    };
}

// ============ Constructor / Accessors ============
impl AncDec128 {
    /// Creates a new `AncDec128`. Panics if `scale > 38` or `frac >= 10^scale`.
    #[inline(always)]
    pub fn new(int: u128, frac: u128, scale: u8, neg: bool) -> Self {
        assert!(scale <= 38, "scale must be <= 38");
        assert!(frac < pow10_128(scale), "frac must be < 10^scale");
        Self { int, frac, scale, neg }
    }

    /// Returns the integer part.
    #[inline(always)]
    pub fn int(&self) -> u128 {
        self.int
    }

    /// Returns the fractional part as a raw value (0 to `10^scale - 1`).
    #[inline(always)]
    pub fn frac(&self) -> u128 {
        self.frac
    }

    /// Returns the number of fractional digits (0-38).
    #[inline(always)]
    pub fn scale(&self) -> u8 {
        self.scale
    }

    /// Returns `true` if the value is negative.
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        self.neg
    }
}

// ============ Core Methods ============
impl AncDec128 {
    /// Parses any `Display` type into an `AncDec128` using a stack buffer (no heap allocation).
    pub fn parse<T: Display>(value: T) -> Result<Self, ParseError> {
        let mut buf = StackBuf::<128>::new();
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

        // parse integer part: stage 1 as u64 (fast), stage 2 as u128 (rare)
        let mut int_u64: u64 = 0;
        let mut int: u128;
        let mut int_digits: u8 = 0;
        let mut has_digits = false;
        // Stage 1: accumulate first 18 digits as u64
        while i < len && int_digits < 18 {
            let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
            if d > 9 {
                break;
            }
            has_digits = true;
            int_u64 = int_u64 * 10 + d as u64;
            int_digits += 1;
            i += 1;
        }
        int = int_u64 as u128;
        // Stage 2: remaining digits as u128 (only when stage 1 hit 18-digit limit)
        if int_digits == 18 {
            while i < len {
                let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
                if d > 9 {
                    break;
                }
                has_digits = true;
                int = int.checked_mul(10).and_then(|v| v.checked_add(d as u128))
                    .ok_or(ParseError::Overflow)?;
                i += 1;
            }
        }

        // skip '.'
        if i < len && unsafe { *b.get_unchecked(i) } == b'.' {
            i += 1;
        }

        // parse fractional part: stage 1 as u64 (fast), stage 2 as u128 (rare)
        let mut frac_u64: u64 = 0;
        let mut frac: u128;
        let mut frac_digits: u8 = 0;
        // Stage 1: accumulate first 18 digits as u64
        while i < len && frac_digits < 18 {
            let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
            if d > 9 {
                break;
            }
            frac_u64 = frac_u64 * 10 + d as u64;
            frac_digits += 1;
            i += 1;
        }
        frac = frac_u64 as u128;
        // Stage 2: remaining digits 19-38 as u128 (only when stage 1 hit 18-digit limit)
        if frac_digits == 18 {
            while i < len {
                let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
                if d > 9 {
                    break;
                }
                if frac_digits < TARGET_SCALE_128 {
                    frac = frac * 10 + d as u128;
                    frac_digits += 1;
                }
                i += 1;
            }
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
    /// Fracs stay in u128 since they are bounded by 10^38 < u128::MAX
    #[inline(always)]
    pub(crate) fn align_frac(&self, other: &Self) -> (u128, u128, u8, u128) {
        if self.scale == other.scale {
            (self.frac, other.frac, self.scale, pow10_128(self.scale))
        } else if self.scale > other.scale {
            let limit = pow10_128(self.scale);
            (
                self.frac,
                // SAFETY: frac < 10^scale by invariant, so frac * 10^delta < 10^38 < u128::MAX
                other.frac * pow10_128(self.scale - other.scale),
                self.scale,
                limit,
            )
        } else {
            let limit = pow10_128(other.scale);
            (
                self.frac * pow10_128(other.scale - self.scale),
                other.frac,
                other.scale,
                limit,
            )
        }
    }

    /// Try to combine int and frac into a single u64 = int * 10^scale + frac
    /// Returns None if any value overflows u64
    #[inline(always)]
    pub(crate) fn try_combine_u64(int: u128, frac: u128, scale: u8) -> Option<u64> {
        if scale > 19 || int > u64::MAX as u128 || frac > u64::MAX as u128 {
            return None;
        }
        (int as u64).checked_mul(pow10(scale))?.checked_add(frac as u64)
    }

    /// Try to combine int and frac into a single u128 = int * 10^scale + frac
    /// Returns None if the result overflows u128
    #[inline(always)]
    pub(crate) fn try_combine_u128(int: u128, frac: u128, scale: u8) -> Option<u128> {
        int.checked_mul(pow10_128(scale))?.checked_add(frac)
    }

    /// Add aligned values (same sign), handles frac overflow
    #[inline(always)]
    pub(crate) fn add_aligned(
        a_int: u128,
        a_frac: u128,
        b_int: u128,
        b_frac: u128,
        scale: u8,
        limit: u128,
    ) -> (u128, u128, u8) {
        let frac = a_frac + b_frac;
        let overflow = (frac >= limit) as u128;
        let int = a_int.checked_add(b_int)
            .and_then(|v| v.checked_add(overflow))
            .expect("integer overflow in addition");
        (int, frac - overflow * limit, scale)
    }

    /// Subtract with magnitude comparison, returns result with correct sign
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn sub_with_cmp(
        a_int: u128,
        a_frac: u128,
        a_neg: bool,
        b_int: u128,
        b_frac: u128,
        b_neg: bool,
        scale: u8,
        limit: u128,
    ) -> Self {
        if (a_int, a_frac) >= (b_int, b_frac) {
            let borrow = (a_frac < b_frac) as u128;
            Self {
                int: a_int - b_int - borrow,
                frac: a_frac.wrapping_sub(b_frac).wrapping_add(borrow * limit),
                scale,
                neg: a_neg,
            }
        } else {
            let borrow = (b_frac < a_frac) as u128;
            Self {
                int: b_int - a_int - borrow,
                frac: b_frac.wrapping_sub(a_frac).wrapping_add(borrow * limit),
                scale,
                neg: b_neg,
            }
        }
    }

    /// Split u256 (hi, lo) back to int/frac using divmod_u256
    #[inline(always)]
    pub(crate) fn from_combined(n: (u128, u128), scale: u8, neg: bool) -> Self {
        if scale == 0 {
            assert!(n.0 == 0, "integer overflow in from_combined");
            return Self {
                int: n.1,
                frac: 0,
                scale: 0,
                neg,
            };
        }
        let divisor = pow10_128(scale);
        let ((q_hi, q), r) = divmod_u256(n.0, n.1, divisor);
        assert!(q_hi == 0, "integer overflow in from_combined");
        Self {
            int: q,
            frac: r,
            scale,
            neg,
        }
    }
}
