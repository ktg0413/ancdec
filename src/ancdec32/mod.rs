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
use crate::util::{pow10_32, StackBuf, TARGET_SCALE_32};
use core::fmt::{Display, Write};

/// 32-bit fixed-point decimal (u32 int/frac, 9-digit precision, 12 bytes).
///
/// Stores integer and fractional parts as separate `u32` values with an explicit scale (0-9).
///
/// # Example
/// ```
/// use ancdec::AncDec32;
/// let a: AncDec32 = "123.456789".parse().unwrap();
/// assert_eq!(a.int(), 123);
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AncDec32 {
    pub(crate) int: u32,
    pub(crate) frac: u32,
    pub(crate) scale: u8,
    pub(crate) neg: bool,
}

// ============ Constants ============
impl AncDec32 {
    /// The value `0`.
    pub const ZERO: AncDec32 = AncDec32 {
        int: 0,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `1`.
    pub const ONE: AncDec32 = AncDec32 {
        int: 1,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `2`.
    pub const TWO: AncDec32 = AncDec32 {
        int: 2,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `10`.
    pub const TEN: AncDec32 = AncDec32 {
        int: 10,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The maximum representable value (`4294967295.999999999`).
    pub const MAX: AncDec32 = AncDec32 {
        int: u32::MAX,
        frac: 999_999_999,
        scale: 9,
        neg: false,
    };
}

// ============ Constructor / Accessors ============
impl AncDec32 {
    /// Creates a new `AncDec32`. Panics if `scale > 9` or `frac >= 10^scale`.
    #[inline(always)]
    pub fn new(int: u32, frac: u32, scale: u8, neg: bool) -> Self {
        assert!(scale <= 9, "scale must be <= 9");
        assert!(frac < pow10_32(scale), "frac must be < 10^scale");
        Self { int, frac, scale, neg }
    }

    /// Returns the integer part.
    #[inline(always)]
    pub fn int(&self) -> u32 {
        self.int
    }

    /// Returns the fractional part as a raw value (0 to `10^scale - 1`).
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        self.frac
    }

    /// Returns the number of fractional digits (0-9).
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
impl AncDec32 {
    /// Parses any `Display` type into an `AncDec32` using a stack buffer (no heap allocation).
    pub fn parse<T: Display>(value: T) -> Result<Self, ParseError> {
        let mut buf = StackBuf::<32>::new();
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
        let mut int: u32 = 0;
        let mut has_digits = false;
        while i < len {
            let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
            if d > 9 {
                break;
            }
            has_digits = true;
            int = int.checked_mul(10).and_then(|v| v.checked_add(d as u32))
                .ok_or(ParseError::Overflow)?;
            i += 1;
        }

        // skip '.'
        if i < len && unsafe { *b.get_unchecked(i) } == b'.' {
            i += 1;
        }

        // parse fractional part (truncate at 9 digits)
        let mut frac: u32 = 0;
        let mut frac_digits: u8 = 0;
        while i < len {
            let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
            if d > 9 {
                break;
            }
            if frac_digits < TARGET_SCALE_32 {
                frac = frac * 10 + d as u32;
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
    pub(crate) fn align_frac(&self, other: &Self) -> (u32, u32, u8, u32) {
        if self.scale == other.scale {
            (self.frac, other.frac, self.scale, pow10_32(self.scale))
        } else if self.scale > other.scale {
            let limit = pow10_32(self.scale);
            (
                self.frac,
                // SAFETY: frac < 10^scale by invariant, so frac * 10^delta < 10^9 < u32::MAX
                other.frac * pow10_32(self.scale - other.scale),
                self.scale,
                limit,
            )
        } else {
            let limit = pow10_32(other.scale);
            (
                self.frac * pow10_32(other.scale - self.scale),
                other.frac,
                other.scale,
                limit,
            )
        }
    }

    /// Add aligned values (same sign), handles frac overflow
    #[inline(always)]
    pub(crate) fn add_aligned(
        a_int: u32,
        a_frac: u32,
        b_int: u32,
        b_frac: u32,
        scale: u8,
        limit: u32,
    ) -> (u32, u32, u8) {
        let frac = a_frac + b_frac;
        let overflow = (frac >= limit) as u32;
        let int = a_int.checked_add(b_int)
            .and_then(|v| v.checked_add(overflow))
            .expect("integer overflow in addition");
        (int, frac - overflow * limit, scale)
    }

    /// Subtract with magnitude comparison, returns result with correct sign
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn sub_with_cmp(
        a_int: u32,
        a_frac: u32,
        a_neg: bool,
        b_int: u32,
        b_frac: u32,
        b_neg: bool,
        scale: u8,
        limit: u32,
    ) -> Self {
        if (a_int, a_frac) >= (b_int, b_frac) {
            let borrow = (a_frac < b_frac) as u32;
            Self {
                int: a_int - b_int - borrow,
                frac: a_frac.wrapping_sub(b_frac).wrapping_add(borrow * limit),
                scale,
                neg: a_neg,
            }
        } else {
            let borrow = (b_frac < a_frac) as u32;
            Self {
                int: b_int - a_int - borrow,
                frac: b_frac.wrapping_sub(a_frac).wrapping_add(borrow * limit),
                scale,
                neg: b_neg,
            }
        }
    }

    /// Split combined u64 back to int/frac
    #[inline(always)]
    pub(crate) fn from_combined(n: u64, scale: u8, neg: bool) -> Self {
        if scale == 0 {
            assert!(n <= u32::MAX as u64, "integer overflow in from_combined");
            return Self {
                int: n as u32,
                frac: 0,
                scale: 0,
                neg,
            };
        }
        let divisor = pow10_32(scale) as u64;
        let int_part = n / divisor;
        assert!(int_part <= u32::MAX as u64, "integer overflow in from_combined");
        Self {
            int: int_part as u32,
            frac: (n % divisor) as u32,
            scale,
            neg,
        }
    }
}
