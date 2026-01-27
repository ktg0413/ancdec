#![no_std]

use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt::{self, Display, Write};
use core::hash::{Hash, Hasher};
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use core::str::FromStr;

#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

// ============ Constants ============
const SCALE19: u128 = 10_000_000_000_000_000_000; // 10^19: for splitting div result
const TARGET_SCALE: u8 = 19; // max fractional digits for mul/div

// ============ Error ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    NoDigits,
    TrailingChars,
    InvalidFloat,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("empty string"),
            Self::NoDigits => f.write_str("no digits found"),
            Self::TrailingChars => f.write_str("trailing characters"),
            Self::InvalidFloat => f.write_str("invalid float (NaN or Infinity)"),
        }
    }
}

// ============ Stack Buffer ============
/// Heap-free buffer for Display -> &str. Safety: only write_str can write, which guarantees UTF-8.
struct StackBuf<const N: usize> {
    buf: [u8; N],
    pos: usize, // invariant: buf[..pos] is valid UTF-8
}

impl<const N: usize> StackBuf<N> {
    #[inline(always)]
    fn new() -> Self {
        Self {
            buf: [0; N],
            pos: 0,
        }
    }

    #[inline(always)]
    fn as_str(&self) -> &str {
        // SAFETY: write_str only accepts &str (valid UTF-8), so buf[..pos] is always valid
        debug_assert!(core::str::from_utf8(&self.buf[..self.pos]).is_ok());
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.pos]) }
    }
}

/// Write trait: enables `write!` macro usage
impl<const N: usize> Write for StackBuf<N> {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let remaining = N - self.pos;

        // truncate at UTF-8 boundary if buffer full
        let len = if bytes.len() <= remaining {
            bytes.len()
        } else {
            let mut i = remaining;
            while i > 0 && !s.is_char_boundary(i) {
                i -= 1;
            }
            i
        };

        self.buf[self.pos..self.pos + len].copy_from_slice(&bytes[..len]);
        self.pos += len;
        Ok(())
    }
}

// ============ Util ============
/// Power of 10 lookup for u64 (0-19)
#[inline(always)]
pub(crate) const fn pow10(exp: u8) -> u64 {
    match exp {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1_000,
        4 => 10_000,
        5 => 100_000,
        6 => 1_000_000,
        7 => 10_000_000,
        8 => 100_000_000,
        9 => 1_000_000_000,
        10 => 10_000_000_000,
        11 => 100_000_000_000,
        12 => 1_000_000_000_000,
        13 => 10_000_000_000_000,
        14 => 100_000_000_000_000,
        15 => 1_000_000_000_000_000,
        16 => 10_000_000_000_000_000,
        17 => 100_000_000_000_000_000,
        18 => 1_000_000_000_000_000_000,
        19 => 10_000_000_000_000_000_000,
        _ => panic!("scale overflow"),
    }
}

/// Power of 10 lookup for u128 (0-38)
#[inline(always)]
pub(crate) const fn pow10_128(exp: u8) -> u128 {
    match exp {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1_000,
        4 => 10_000,
        5 => 100_000,
        6 => 1_000_000,
        7 => 10_000_000,
        8 => 100_000_000,
        9 => 1_000_000_000,
        10 => 10_000_000_000,
        11 => 100_000_000_000,
        12 => 1_000_000_000_000,
        13 => 10_000_000_000_000,
        14 => 100_000_000_000_000,
        15 => 1_000_000_000_000_000,
        16 => 10_000_000_000_000_000,
        17 => 100_000_000_000_000_000,
        18 => 1_000_000_000_000_000_000,
        19 => 10_000_000_000_000_000_000,
        20 => 100_000_000_000_000_000_000,
        21 => 1_000_000_000_000_000_000_000,
        22 => 10_000_000_000_000_000_000_000,
        23 => 100_000_000_000_000_000_000_000,
        24 => 1_000_000_000_000_000_000_000_000,
        25 => 10_000_000_000_000_000_000_000_000,
        26 => 100_000_000_000_000_000_000_000_000,
        27 => 1_000_000_000_000_000_000_000_000_000,
        28 => 10_000_000_000_000_000_000_000_000_000,
        29 => 100_000_000_000_000_000_000_000_000_000,
        30 => 1_000_000_000_000_000_000_000_000_000_000,
        31 => 10_000_000_000_000_000_000_000_000_000_000,
        32 => 100_000_000_000_000_000_000_000_000_000_000,
        33 => 1_000_000_000_000_000_000_000_000_000_000_000,
        34 => 10_000_000_000_000_000_000_000_000_000_000_000,
        35 => 100_000_000_000_000_000_000_000_000_000_000_000,
        36 => 1_000_000_000_000_000_000_000_000_000_000_000_000,
        37 => 10_000_000_000_000_000_000_000_000_000_000_000_000,
        38 => 100_000_000_000_000_000_000_000_000_000_000_000_000,
        _ => panic!("scale overflow"),
    }
}

// ============ RoundMode ============
/// Rounding modes for decimal operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundMode {
    Floor,    // toward -∞
    Ceil,     // toward +∞
    Truncate, // toward zero
    HalfUp,   // >= 0.5 away from zero
    HalfDown, // > 0.5 away from zero
    HalfEven, // banker's rounding
    Fract,    // return fractional part only
}

// ============ AncDec ============
/// Anchored Decimal: fixed-point decimal with separate int/frac storage
/// - int: integer part (0 to u64::MAX)
/// - frac: fractional part (0 to 10^scale - 1)
/// - scale: fractional digits (0-19)
/// - neg: sign flag
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AncDec {
    pub int: u64,
    pub frac: u64,
    pub scale: u8,
    pub neg: bool,
}

// ============ Constants ============
impl AncDec {
    pub const ZERO: AncDec = AncDec {
        int: 0,
        frac: 0,
        scale: 0,
        neg: false,
    };
    pub const ONE: AncDec = AncDec {
        int: 1,
        frac: 0,
        scale: 0,
        neg: false,
    };
    pub const TWO: AncDec = AncDec {
        int: 2,
        frac: 0,
        scale: 0,
        neg: false,
    };
    pub const TEN: AncDec = AncDec {
        int: 10,
        frac: 0,
        scale: 0,
        neg: false,
    };
    pub const MAX: AncDec = AncDec {
        int: u64::MAX,
        frac: u64::MAX,
        scale: 19,
        neg: false,
    };
}

// ============ Core Methods ============
impl AncDec {
    /// Create from any Display type via stack buffer (no heap)
    pub fn parse<T: Display>(value: T) -> Result<Self, ParseError> {
        let mut buf = StackBuf::<64>::new();
        write!(buf, "{}", value).ok();
        Self::parse_str(buf.as_str())
    }

    /// Byte-level string parsing with validation
    fn parse_str(s: &str) -> Result<Self, ParseError> {
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

        // parse integer part (saturate at u64::MAX)
        let mut int: u64 = 0;
        let mut has_digits = false;
        while i < len {
            let d = unsafe { *b.get_unchecked(i) }.wrapping_sub(b'0');
            if d > 9 {
                break;
            }
            has_digits = true;
            int = int.saturating_mul(10).saturating_add(d as u64);
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

        if !has_digits {
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
    fn align_frac(&self, other: &Self) -> (u64, u64, u8, u64) {
        if self.scale == other.scale {
            (self.frac, other.frac, self.scale, pow10(self.scale))
        } else if self.scale > other.scale {
            let limit = pow10(self.scale);
            (
                self.frac,
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
    fn add_aligned(
        a_int: u64,
        a_frac: u64,
        b_int: u64,
        b_frac: u64,
        scale: u8,
        limit: u64,
    ) -> (u64, u64, u8) {
        let frac = a_frac + b_frac;
        let overflow = (frac >= limit) as u64;
        (a_int + b_int + overflow, frac - overflow * limit, scale)
    }

    /// Subtract with magnitude comparison, returns result with correct sign
    #[inline(always)]
    fn sub_with_cmp(
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
    fn from_combined(n: u128, scale: u8, neg: bool) -> Self {
        if scale == 0 {
            return Self {
                int: n as u64,
                frac: 0,
                scale: 0,
                neg,
            };
        }
        let divisor = pow10_128(scale);
        Self {
            int: (n / divisor) as u64,
            frac: (n % divisor) as u64,
            scale,
            neg,
        }
    }
}

// ============ Arithmetic ============
impl AncDec {
    #[inline(always)]
    pub fn add(&self, other: &Self) -> Self {
        let (a_frac, b_frac, scale, limit) = self.align_frac(other);

        if self.neg == other.neg {
            let (int, frac, scale) =
                Self::add_aligned(self.int, a_frac, other.int, b_frac, scale, limit);
            Self {
                int,
                frac,
                scale,
                neg: self.neg,
            }
        } else {
            Self::sub_with_cmp(
                self.int, a_frac, self.neg, other.int, b_frac, other.neg, scale, limit,
            )
        }
    }

    #[inline(always)]
    pub fn sub(&self, other: &Self) -> Self {
        let (a_frac, b_frac, scale, limit) = self.align_frac(other);
        let other_neg = !other.neg;

        if self.neg == other_neg {
            let (int, frac, scale) =
                Self::add_aligned(self.int, a_frac, other.int, b_frac, scale, limit);
            Self {
                int,
                frac,
                scale,
                neg: self.neg,
            }
        } else {
            Self::sub_with_cmp(
                self.int, a_frac, self.neg, other.int, b_frac, other_neg, scale, limit,
            )
        }
    }

    #[inline(always)]
    pub fn mul(&self, other: &Self) -> Self {
        let a = (self.int as u128) * pow10_128(self.scale) + (self.frac as u128);
        let b = (other.int as u128) * pow10_128(other.scale) + (other.frac as u128);
        let total_scale = self.scale + other.scale;

        let (high, low) = mul_wide(a, b);

        let (result, final_scale) = if total_scale > TARGET_SCALE {
            let divisor = pow10_128(total_scale - TARGET_SCALE);
            (div_wide(high, low, divisor), TARGET_SCALE)
        } else if high == 0 {
            (low, total_scale)
        } else {
            (u128::MAX, total_scale)
        };

        let limit = pow10_128(final_scale);
        Self {
            int: (result / limit) as u64,
            frac: (result % limit) as u64,
            scale: final_scale,
            neg: self.neg ^ other.neg,
        }
    }

    #[inline(always)]
    pub fn div(&self, other: &Self) -> Self {
        debug_assert!(other.int != 0 || other.frac != 0, "division by zero");

        let a = (self.int as u128) * pow10_128(self.scale) + (self.frac as u128);
        let b = (other.int as u128) * pow10_128(other.scale) + (other.frac as u128);

        let shift = TARGET_SCALE + other.scale;

        let quotient = if shift >= self.scale {
            let multiplier = pow10_128(shift - self.scale);
            let (high, low) = mul_wide(a, multiplier);
            div_wide(high, low, b)
        } else {
            a / (b * pow10_128(self.scale - shift))
        };

        let q = quotient / SCALE19;
        let r = quotient - q * SCALE19;
        Self {
            int: q as u64,
            frac: r as u64,
            scale: TARGET_SCALE,
            neg: self.neg ^ other.neg,
        }
    }

    /// a % b = a - floor(a/b) * b
    #[inline(always)]
    pub fn rem(&self, other: &Self) -> Self {
        debug_assert!(other.int != 0 || other.frac != 0, "division by zero");
        let q = self.div(other);
        let floored = Self {
            int: q.int,
            frac: 0,
            scale: 0,
            neg: q.neg,
        };
        self.sub(&floored.mul(other))
    }
}

// ============ Basic Methods ============
impl AncDec {
    #[inline(always)]
    pub fn abs(&self) -> Self {
        Self {
            neg: false,
            ..*self
        }
    }

    #[inline(always)]
    pub fn signum(&self) -> Self {
        if self.is_zero() {
            Self::ZERO
        } else if self.neg {
            Self {
                int: 1,
                frac: 0,
                scale: 0,
                neg: true,
            }
        } else {
            Self::ONE
        }
    }

    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        !self.neg && !self.is_zero()
    }

    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        self.neg && !self.is_zero()
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.int == 0 && self.frac == 0
    }
}
// ============ Wide Arithmetic (u256) ============
// 모듈 내부에서만 사용되는 헬퍼 함수들

/// u128 * u128 -> (high, low)
#[inline]
fn mul_wide(a: u128, b: u128) -> (u128, u128) {
    let a_lo = a as u64 as u128;
    let a_hi = a >> 64;
    let b_lo = b as u64 as u128;
    let b_hi = b >> 64;

    let ll = a_lo * b_lo;
    let hl = a_hi * b_lo;
    let lh = a_lo * b_hi;
    let hh = a_hi * b_hi;

    let mid = hl + lh;
    let mid_carry = (mid < hl) as u128;

    let (low, carry) = ll.overflowing_add(mid << 64);
    let high = hh + (mid >> 64) + (mid_carry << 64) + carry as u128;

    (high, low)
}

/// Shift u256 left by `shift` bits (shift < 128)
#[inline]
fn shl_u256(high: u128, low: u128, shift: u32) -> (u64, u64, u64, u64) {
    if shift == 0 {
        return (
            (high >> 64) as u64,
            high as u64,
            (low >> 64) as u64,
            low as u64,
        );
    }

    let high_shifted = (high << shift) | (low >> (128 - shift));
    let low_shifted = low << shift;

    (
        (high_shifted >> 64) as u64,
        high_shifted as u64,
        (low_shifted >> 64) as u64,
        low_shifted as u64,
    )
}

/// u128 / u64 -> (quotient, remainder)
#[inline]
fn div_u128_by_u64(n: u128, d: u64) -> (u128, u128) {
    (n / (d as u128), n % (d as u128))
}

/// Divide u256 by u64 -> u128 quotient
#[inline]
fn div_wide_by_u64(high: u128, low: u128, divisor: u64) -> u128 {
    let n3 = (high >> 64) as u64;
    let n2 = high as u64;
    let n1 = (low >> 64) as u64;
    let n0 = low as u64;

    let (q3, r3) = div_u128_by_u64(n3 as u128, divisor);
    let (q2, r2) = div_u128_by_u64((r3 << 64) | (n2 as u128), divisor);
    let (q1, r1) = div_u128_by_u64((r2 << 64) | (n1 as u128), divisor);
    let (q0, _) = div_u128_by_u64((r1 << 64) | (n0 as u128), divisor);

    debug_assert!(q3 == 0 && q2 == 0);
    (q1 << 64) | q0
}

/// Core: divide (n2, n1, n0) by (d1, d0) where values are normalized
#[inline]
fn div_3by2(n2: u64, n1: u64, n0: u64, d1: u64, d0: u64) -> (u64, u128) {
    let n_hi = ((n2 as u128) << 64) | (n1 as u128);
    let mut q_hat = if n2 >= d1 {
        u64::MAX
    } else {
        (n_hi / (d1 as u128)) as u64
    };
    let mut r_hat = n_hi - (q_hat as u128) * (d1 as u128);

    loop {
        if r_hat <= u64::MAX as u128 {
            let check = (q_hat as u128) * (d0 as u128);
            let right = (r_hat << 64) | (n0 as u128);
            if check <= right {
                break;
            }
        } else {
            break;
        }
        q_hat -= 1;
        r_hat += d1 as u128;
        if r_hat > u64::MAX as u128 {
            break;
        }
    }

    let product = (q_hat as u128) * (d0 as u128);
    let product_hi = (q_hat as u128) * (d1 as u128) + (product >> 64);

    let (sub_lo, borrow1) = (n0 as u128).overflowing_sub(product & ((1u128 << 64) - 1));
    let (sub_mid, borrow2) =
        (n1 as u128).overflowing_sub((product_hi & ((1u128 << 64) - 1)) + borrow1 as u128);
    let sub_hi = (n2 as u128).wrapping_sub((product_hi >> 64) + borrow2 as u128);

    let (rem_hi, q_final) = if sub_hi > n2 as u128 {
        let add_lo = sub_lo.wrapping_add(d0 as u128);
        let carry = (add_lo < sub_lo) as u128;
        let add_mid = sub_mid.wrapping_add((d1 as u128) + carry);
        ((add_mid << 64) | (add_lo & ((1u128 << 64) - 1)), q_hat - 1)
    } else {
        (
            ((sub_mid & ((1u128 << 64) - 1)) << 64) | (sub_lo & ((1u128 << 64) - 1)),
            q_hat,
        )
    };

    (q_final, rem_hi)
}

/// Full 4-by-2 division: (n3,n2,n1,n0) / (d1,d0) -> (q1, q0)
#[inline]
fn div_4by2(n3: u64, n2: u64, n1: u64, n0: u64, d1: u64, d0: u64) -> (u64, u64) {
    let (q1, rem1) = div_3by2(n3, n2, n1, d1, d0);
    let r1_hi = (rem1 >> 64) as u64;
    let r1_lo = rem1 as u64;
    let (q0, _) = div_3by2(r1_hi, r1_lo, n0, d1, d0);
    (q1, q0)
}

/// Knuth Algorithm D: (u256) / (u128) -> u128
#[inline]
fn div_wide(high: u128, low: u128, divisor: u128) -> u128 {
    debug_assert!(divisor != 0, "division by zero");

    if high == 0 {
        return low / divisor;
    }

    debug_assert!(high < divisor, "quotient overflow");

    let d_hi = (divisor >> 64) as u64;

    if d_hi == 0 {
        return div_wide_by_u64(high, low, divisor as u64);
    }

    let shift = divisor.leading_zeros();
    let divisor_norm = divisor << shift;
    let d1 = (divisor_norm >> 64) as u64;
    let d0 = divisor_norm as u64;

    let (n3, n2, n1, n0) = shl_u256(high, low, shift);
    let (q1, q0) = div_4by2(n3, n2, n1, n0, d1, d0);

    ((q1 as u128) << 64) | (q0 as u128)
}
// ============ Range ============
impl AncDec {
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }

    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }

    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

// ============ Power ============
impl AncDec {
    /// Binary exponentiation: O(log n)
    pub fn pow(&self, n: i32) -> Self {
        if n == 0 {
            return Self::ONE;
        }

        let mut base = if n < 0 { Self::ONE.div(self) } else { *self };
        let mut exp = n.unsigned_abs();
        let mut result = Self::ONE;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            exp /= 2;
        }
        result
    }
}

// ============ Rounding ============
impl AncDec {
    pub fn round(&self, decimal_places: u8, mode: RoundMode) -> Self {
        if mode == RoundMode::Fract {
            return Self {
                int: 0,
                frac: self.frac,
                scale: self.scale,
                neg: self.neg,
            };
        }
        if self.scale <= decimal_places {
            return *self;
        }

        let combined = (self.int as u128) * pow10_128(self.scale) + (self.frac as u128);
        let cut = self.scale - decimal_places;
        let divisor = pow10_128(cut);
        let remainder = combined % divisor;
        let mut truncated = combined / divisor;

        if self.should_round_up(truncated, remainder, divisor, mode) {
            truncated += 1;
        }
        Self::from_combined(truncated, decimal_places, self.neg)
    }

    fn should_round_up(
        &self,
        truncated: u128,
        remainder: u128,
        divisor: u128,
        mode: RoundMode,
    ) -> bool {
        if remainder == 0 {
            return false;
        }
        let half = divisor / 2;

        match mode {
            RoundMode::Floor => self.neg,
            RoundMode::Ceil => !self.neg,
            RoundMode::Truncate => false,
            RoundMode::HalfUp => remainder >= half,
            RoundMode::HalfDown => remainder > half,
            RoundMode::HalfEven => remainder > half || (remainder == half && truncated % 2 == 1),
            RoundMode::Fract => false,
        }
    }

    #[inline(always)]
    pub fn floor(&self) -> Self {
        self.round(0, RoundMode::Floor)
    }
    #[inline(always)]
    pub fn ceil(&self) -> Self {
        self.round(0, RoundMode::Ceil)
    }
    #[inline(always)]
    pub fn trunc(&self) -> Self {
        self.round(0, RoundMode::Truncate)
    }
    #[inline(always)]
    pub fn fract(&self) -> Self {
        self.round(0, RoundMode::Fract)
    }
}

// ============ Conversion ============
impl AncDec {
    pub fn to_f64(&self) -> f64 {
        let v = self.int as f64
            + if self.scale == 0 {
                0.0
            } else {
                self.frac as f64 / pow10(self.scale) as f64
            };
        if self.neg {
            -v
        } else {
            v
        }
    }

    pub fn to_i64(&self) -> i64 {
        if self.neg {
            -(self.int as i64)
        } else {
            self.int as i64
        }
    }

    pub fn to_i128(&self) -> i128 {
        if self.neg {
            -(self.int as i128)
        } else {
            self.int as i128
        }
    }
}

// ============ Display ============
/// Display trait: enables `format!`, `println!`, `to_string()`
impl fmt::Display for AncDec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.neg { "-" } else { "" };

        if let Some(prec) = f.precision() {
            if prec == 0 {
                write!(f, "{}{}", sign, self.int)
            } else if self.scale == 0 {
                write!(f, "{}{}.{:0>w$}", sign, self.int, "", w = prec)
            } else if prec <= self.scale as usize {
                let div = 10u64.pow((self.scale as u32) - (prec as u32));
                write!(f, "{}{}.{:0>w$}", sign, self.int, self.frac / div, w = prec)
            } else {
                write!(
                    f,
                    "{}{}.{:0>s$}{:0>p$}",
                    sign,
                    self.int,
                    self.frac,
                    "",
                    s = self.scale as usize,
                    p = prec - self.scale as usize
                )
            }
        } else if self.scale == 0 {
            write!(f, "{}{}", sign, self.int)
        } else {
            write!(
                f,
                "{}{}.{:0>w$}",
                sign,
                self.int,
                self.frac,
                w = self.scale as usize
            )
        }
    }
}

// ============ FromStr ============
/// FromStr trait: enables `"123.45".parse::<AncDec>()`
impl FromStr for AncDec {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

// ============ TryFrom &str ============
impl TryFrom<&str> for AncDec {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse_str(s)
    }
}

// ============ TryFrom f32/f64 ============
impl TryFrom<f32> for AncDec {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(n: f32) -> Result<Self, Self::Error> {
        AncDec::try_from(n as f64)
    }
}

impl TryFrom<f64> for AncDec {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(n: f64) -> Result<Self, Self::Error> {
        if n.is_nan() || n.is_infinite() {
            return Err(ParseError::InvalidFloat);
        }
        let mut buf = StackBuf::<64>::new();
        write!(buf, "{}", n).ok();
        Self::parse_str(buf.as_str())
    }
}

// ============ Serde ============
/// Serialize as string "123.45"
#[cfg(feature = "serde")]
impl Serialize for AncDec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// Deserialize from string
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for AncDec {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> de::Visitor<'de> for V {
            type Value = AncDec;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("decimal string")
            }
            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                AncDec::parse_str(s).map_err(|e| E::custom(e))
            }
        }
        deserializer.deserialize_str(V)
    }
}

// ============ Comparison ============
/// Compare absolute values
#[inline(always)]
fn cmp_abs(a: &AncDec, b: &AncDec) -> Ordering {
    if a.int != b.int {
        return a.int.cmp(&b.int);
    }

    let (a_frac, b_frac) = if a.scale == b.scale {
        (a.frac, b.frac)
    } else if a.scale > b.scale {
        (a.frac, b.frac * pow10(a.scale - b.scale))
    } else {
        (a.frac * pow10(b.scale - a.scale), b.frac)
    };
    a_frac.cmp(&b_frac)
}

/// Ord trait: enables `<`, `>`, `sort()`, `min()`, `max()`
impl Ord for AncDec {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        // 0 == -0
        if self.is_zero() && other.is_zero() {
            return Ordering::Equal;
        }

        match (self.neg, other.neg) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => cmp_abs(self, other),
            (true, true) => cmp_abs(self, other).reverse(),
        }
    }
}

/// PartialOrd trait: enables `<`, `>` comparisons
impl PartialOrd for AncDec {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// PartialEq trait: enables `==`, `!=`
impl PartialEq for AncDec {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

/// Eq trait: marker for total equality (no NaN)
impl Eq for AncDec {}

// ============ Operator Traits ============
/// Add trait: enables `a + b`
impl Add for AncDec {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        (&self).add(&rhs)
    }
}

/// Sub trait: enables `a - b`
impl Sub for AncDec {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        (&self).sub(&rhs)
    }
}

/// Mul trait: enables `a * b`
impl Mul for AncDec {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        (&self).mul(&rhs)
    }
}

/// Div trait: enables `a / b`
impl Div for AncDec {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        (&self).div(&rhs)
    }
}

/// Rem trait: enables `a % b`
impl Rem for AncDec {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        (&self).rem(&rhs)
    }
}

/// Neg trait: enables `-a`
impl Neg for AncDec {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            neg: !self.neg,
            ..self
        }
    }
}

// ============ Assign Ops ============
/// AddAssign: enables `a += b`
impl AddAssign for AncDec {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(&rhs);
    }
}

/// SubAssign: enables `a -= b`
impl SubAssign for AncDec {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(&rhs);
    }
}

/// MulAssign: enables `a *= b`
impl MulAssign for AncDec {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(&rhs);
    }
}

/// DivAssign: enables `a /= b`
impl DivAssign for AncDec {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(&rhs);
    }
}

/// RemAssign: enables `a %= b`
impl RemAssign for AncDec {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(&rhs);
    }
}

// ============ Reference Ops ============
// All combinations: &T op &T, T op &T, &T op T
impl<'a, 'b> Add<&'b AncDec> for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn add(self, rhs: &'b AncDec) -> AncDec {
        self.add(rhs)
    }
}
impl<'a> Add<&'a AncDec> for AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn add(self, rhs: &'a AncDec) -> AncDec {
        (&self).add(rhs)
    }
}
impl<'a> Add<AncDec> for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn add(self, rhs: AncDec) -> AncDec {
        self.add(&rhs)
    }
}

impl<'a, 'b> Sub<&'b AncDec> for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn sub(self, rhs: &'b AncDec) -> AncDec {
        self.sub(rhs)
    }
}
impl<'a> Sub<&'a AncDec> for AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn sub(self, rhs: &'a AncDec) -> AncDec {
        (&self).sub(rhs)
    }
}
impl<'a> Sub<AncDec> for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn sub(self, rhs: AncDec) -> AncDec {
        self.sub(&rhs)
    }
}

impl<'a, 'b> Mul<&'b AncDec> for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn mul(self, rhs: &'b AncDec) -> AncDec {
        self.mul(rhs)
    }
}
impl<'a> Mul<&'a AncDec> for AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn mul(self, rhs: &'a AncDec) -> AncDec {
        (&self).mul(rhs)
    }
}
impl<'a> Mul<AncDec> for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn mul(self, rhs: AncDec) -> AncDec {
        self.mul(&rhs)
    }
}

impl<'a, 'b> Div<&'b AncDec> for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn div(self, rhs: &'b AncDec) -> AncDec {
        self.div(rhs)
    }
}
impl<'a> Div<&'a AncDec> for AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn div(self, rhs: &'a AncDec) -> AncDec {
        (&self).div(rhs)
    }
}
impl<'a> Div<AncDec> for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn div(self, rhs: AncDec) -> AncDec {
        self.div(&rhs)
    }
}

impl<'a, 'b> Rem<&'b AncDec> for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn rem(self, rhs: &'b AncDec) -> AncDec {
        self.rem(rhs)
    }
}
impl<'a> Rem<&'a AncDec> for AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn rem(self, rhs: &'a AncDec) -> AncDec {
        (&self).rem(rhs)
    }
}
impl<'a> Rem<AncDec> for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn rem(self, rhs: AncDec) -> AncDec {
        self.rem(&rhs)
    }
}

impl<'a> Neg for &'a AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn neg(self) -> AncDec {
        AncDec {
            neg: !self.neg,
            ..*self
        }
    }
}

// ============ Default ============
/// Default trait: `AncDec::default()` returns ZERO
impl Default for AncDec {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

// ============ Hash ============
/// Hash trait: enables use in HashMap/HashSet
/// Normalizes trailing zeros so 1.0 == 1.00 have same hash
impl Hash for AncDec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut combined = (self.int as u128) * pow10_128(self.scale) + (self.frac as u128);
        while combined > 0 && combined % 10 == 0 {
            combined /= 10;
        }
        combined.hash(state);
        if combined != 0 {
            self.neg.hash(state);
        } // 0 == -0
    }
}

// ============ Iterator Traits ============
/// Sum trait: enables `iter.sum::<AncDec>()`
impl Sum for AncDec {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, x| a.add(&x))
    }
}
impl<'a> Sum<&'a AncDec> for AncDec {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, x| a.add(x))
    }
}

/// Product trait: enables `iter.product::<AncDec>()`
impl Product for AncDec {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, x| a.mul(&x))
    }
}
impl<'a> Product<&'a AncDec> for AncDec {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, x| a.mul(x))
    }
}

// ============ From Integer ============
/// From<signed>: negative values set neg=true
macro_rules! impl_from_signed { ($($t:ty),*) => {$( impl From<$t> for AncDec { #[inline(always)] fn from(n: $t) -> Self { Self { int: n.unsigned_abs() as u64, frac: 0, scale: 0, neg: n < 0 } } } )*}; }

/// From<unsigned>: always positive
macro_rules! impl_from_unsigned { ($($t:ty),*) => {$( impl From<$t> for AncDec { #[inline(always)] fn from(n: $t) -> Self { Self { int: n as u64, frac: 0, scale: 0, neg: false } } } )*}; }

impl_from_signed!(i8, i16, i32, i64, i128, isize);
impl_from_unsigned!(u8, u16, u32, u64, u128, usize);

// ============ Ops with Primitives ============
/// Enables `AncDec + i32`, `i32 + AncDec`, etc.
macro_rules! impl_ops_primitive {
    ($($t:ty),*) => {$(
        impl Add<$t> for AncDec { type Output = AncDec; #[inline(always)] fn add(self, rhs: $t) -> AncDec { self.add(&AncDec::from(rhs)) } }
        impl Add<AncDec> for $t { type Output = AncDec; #[inline(always)] fn add(self, rhs: AncDec) -> AncDec { AncDec::from(self).add(&rhs) } }
        impl Sub<$t> for AncDec { type Output = AncDec; #[inline(always)] fn sub(self, rhs: $t) -> AncDec { self.sub(&AncDec::from(rhs)) } }
        impl Sub<AncDec> for $t { type Output = AncDec; #[inline(always)] fn sub(self, rhs: AncDec) -> AncDec { AncDec::from(self).sub(&rhs) } }
        impl Mul<$t> for AncDec { type Output = AncDec; #[inline(always)] fn mul(self, rhs: $t) -> AncDec { self.mul(&AncDec::from(rhs)) } }
        impl Mul<AncDec> for $t { type Output = AncDec; #[inline(always)] fn mul(self, rhs: AncDec) -> AncDec { AncDec::from(self).mul(&rhs) } }
        impl Div<$t> for AncDec { type Output = AncDec; #[inline(always)] fn div(self, rhs: $t) -> AncDec { self.div(&AncDec::from(rhs)) } }
        impl Div<AncDec> for $t { type Output = AncDec; #[inline(always)] fn div(self, rhs: AncDec) -> AncDec { AncDec::from(self).div(&rhs) } }
    )*};
}
impl_ops_primitive!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
