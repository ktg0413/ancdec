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
use crate::util::{pow10_u8, pow10_16, StackBuf, TARGET_SCALE_8};
use core::fmt::{Display, Write};

/// 8-bit fixed-point decimal (u8 int/frac, 2-digit precision, 4 bytes).
///
/// Stores integer and fractional parts as separate `u8` values with an explicit scale (0-2).
///
/// # Example
/// ```
/// use ancdec::AncDec8;
/// let a: AncDec8 = "1.23".parse().unwrap();
/// assert_eq!(a.int(), 1);
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AncDec8 {
    pub(crate) int: u8,
    pub(crate) frac: u8,
    pub(crate) scale: u8,
    pub(crate) neg: bool,
}

// ============ Constants ============
impl AncDec8 {
    /// The value `0`.
    pub const ZERO: AncDec8 = AncDec8 {
        int: 0,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `1`.
    pub const ONE: AncDec8 = AncDec8 {
        int: 1,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `2`.
    pub const TWO: AncDec8 = AncDec8 {
        int: 2,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The value `10`.
    pub const TEN: AncDec8 = AncDec8 {
        int: 10,
        frac: 0,
        scale: 0,
        neg: false,
    };
    /// The maximum representable value (`255.99`).
    pub const MAX: AncDec8 = AncDec8 {
        int: u8::MAX,
        frac: 99,
        scale: 2,
        neg: false,
    };
}

// ============ Constructor / Accessors ============
impl AncDec8 {
    /// Creates a new `AncDec8`. Panics if `scale > 2` or `frac >= 10^scale`.
    #[inline(always)]
    pub fn new(int: u8, frac: u8, scale: u8, neg: bool) -> Self {
        assert!(scale <= 2, "scale must be <= 2");
        assert!(frac < pow10_u8(scale), "frac must be < 10^scale");
        Self { int, frac, scale, neg }
    }

    /// Returns the integer part.
    #[inline(always)]
    pub fn int(&self) -> u8 {
        self.int
    }

    /// Returns the fractional part as a raw value (0 to `10^scale - 1`).
    #[inline(always)]
    pub fn frac(&self) -> u8 {
        self.frac
    }

    /// Returns the number of fractional digits (0-2).
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
impl AncDec8 {
    /// Parses any `Display` type into an `AncDec8` using a stack buffer (no heap allocation).
    pub fn parse<T: Display>(value: T) -> Result<Self, ParseError> {
        let mut buf = StackBuf::<16>::new();
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
        let mut int: u8 = 0;
        let mut has_digits = false;
        while i < len {
            let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
            if d > 9 {
                break;
            }
            has_digits = true;
            int = int.checked_mul(10).and_then(|v| v.checked_add(d))
                .ok_or(ParseError::Overflow)?;
            i += 1;
        }

        // skip '.'
        if i < len && unsafe { *b.get_unchecked(i) } == b'.' {
            i += 1;
        }

        // parse fractional part (truncate at 2 digits)
        let mut frac: u8 = 0;
        let mut frac_digits: u8 = 0;
        while i < len {
            let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
            if d > 9 {
                break;
            }
            if frac_digits < TARGET_SCALE_8 {
                frac = frac * 10 + d;
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
    pub(crate) fn align_frac(&self, other: &Self) -> (u8, u8, u8, u8) {
        if self.scale == other.scale {
            (self.frac, other.frac, self.scale, pow10_u8(self.scale))
        } else if self.scale > other.scale {
            let limit = pow10_u8(self.scale);
            // SAFETY: frac < 10^scale by invariant, so frac * 10^delta < 10^target <= 100 <= u8::MAX
            debug_assert!(other.frac < pow10_u8(other.scale));
            (
                self.frac,
                other.frac * pow10_u8(self.scale - other.scale),
                self.scale,
                limit,
            )
        } else {
            let limit = pow10_u8(other.scale);
            debug_assert!(self.frac < pow10_u8(self.scale));
            (
                self.frac * pow10_u8(other.scale - self.scale),
                other.frac,
                other.scale,
                limit,
            )
        }
    }

    /// Add aligned values (same sign), handles frac overflow
    #[inline(always)]
    pub(crate) fn add_aligned(
        a_int: u8,
        a_frac: u8,
        b_int: u8,
        b_frac: u8,
        scale: u8,
        limit: u8,
    ) -> (u8, u8, u8) {
        let frac = a_frac + b_frac;
        let overflow = (frac >= limit) as u8;
        let int = a_int.checked_add(b_int)
            .and_then(|v| v.checked_add(overflow))
            .expect("integer overflow in addition");
        (int, frac - overflow * limit, scale)
    }

    /// Subtract with magnitude comparison, returns result with correct sign
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn sub_with_cmp(
        a_int: u8,
        a_frac: u8,
        a_neg: bool,
        b_int: u8,
        b_frac: u8,
        b_neg: bool,
        scale: u8,
        limit: u8,
    ) -> Self {
        if (a_int, a_frac) >= (b_int, b_frac) {
            let borrow = (a_frac < b_frac) as u8;
            Self {
                int: a_int - b_int - borrow,
                frac: a_frac.wrapping_sub(b_frac).wrapping_add(borrow * limit),
                scale,
                neg: a_neg,
            }
        } else {
            let borrow = (b_frac < a_frac) as u8;
            Self {
                int: b_int - a_int - borrow,
                frac: b_frac.wrapping_sub(a_frac).wrapping_add(borrow * limit),
                scale,
                neg: b_neg,
            }
        }
    }

    /// Split u16 combined value back to int/frac
    #[inline(always)]
    pub(crate) fn from_combined(n: u16, scale: u8, neg: bool) -> Self {
        if scale == 0 {
            assert!(n <= u8::MAX as u16, "integer overflow in from_combined");
            return Self {
                int: n as u8,
                frac: 0,
                scale: 0,
                neg,
            };
        }
        let divisor = pow10_16(scale);
        let int_part = n / divisor;
        assert!(int_part <= u8::MAX as u16, "integer overflow in from_combined");
        Self {
            int: int_part as u8,
            frac: (n % divisor) as u8,
            scale,
            neg,
        }
    }
}
