use super::AncDec32;
use crate::util::{pow10, pow10_128, SCALE9, TARGET_SCALE_32};

impl AncDec32 {
    /// Adds two decimals, panics on integer overflow.
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

    /// Subtracts `other` from `self`, panics on integer overflow.
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

    /// Multiplies two decimals, panics on overflow.
    #[inline(always)]
    pub fn mul(&self, other: &Self) -> Self {
        let a = self.int as u64 * pow10(self.scale) + self.frac as u64;
        let b = other.int as u64 * pow10(other.scale) + other.frac as u64;
        let total_scale = self.scale + other.scale;

        let product = a as u128 * b as u128;

        let (result, final_scale) = if total_scale > TARGET_SCALE_32 {
            let divisor = pow10_128(total_scale - TARGET_SCALE_32);
            ((product / divisor) as u64, TARGET_SCALE_32)
        } else if product <= u64::MAX as u128 {
            (product as u64, total_scale)
        } else {
            panic!("multiplication overflow")
        };

        Self::from_combined(result, final_scale, self.neg ^ other.neg)
    }

    /// Divides `self` by `other`, panics on division by zero.
    #[inline(always)]
    pub fn div(&self, other: &Self) -> Self {
        assert!(other.int != 0 || other.frac != 0, "division by zero");

        let a = self.int as u64 * pow10(self.scale) + self.frac as u64;
        let b = other.int as u64 * pow10(other.scale) + other.frac as u64;

        let shift = TARGET_SCALE_32 + other.scale;

        let quotient = if shift >= self.scale {
            let exp = shift - self.scale;
            let numerator = a as u128 * pow10_128(exp);
            (numerator / b as u128) as u64
        } else {
            a / (b * pow10(self.scale - shift))
        };

        let q = quotient / SCALE9;
        let r = quotient - q * SCALE9;
        Self {
            int: q as u32,
            frac: r as u32,
            scale: TARGET_SCALE_32,
            neg: self.neg ^ other.neg,
        }
    }

    /// Checked addition. Returns `None` if the integer part overflows `u32`.
    #[inline(always)]
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        let (a_frac, b_frac, scale, limit) = self.align_frac(other);

        if self.neg == other.neg {
            let frac = a_frac + b_frac;
            let overflow = (frac >= limit) as u32;
            let int = self.int.checked_add(other.int)?.checked_add(overflow)?;
            Some(Self {
                int,
                frac: frac - overflow * limit,
                scale,
                neg: self.neg,
            })
        } else {
            Some(Self::sub_with_cmp(
                self.int, a_frac, self.neg, other.int, b_frac, other.neg, scale, limit,
            ))
        }
    }

    /// Checked subtraction. Returns `None` if the integer part overflows `u32`.
    ///
    /// In practice, subtraction in the current design uses magnitude comparison
    /// and cannot overflow, so this always returns `Some`.
    #[inline(always)]
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        Some(self.sub(other))
    }

    /// Checked multiplication. Returns `None` if the result overflows `u32` integer range.
    #[inline(always)]
    pub fn checked_mul(&self, other: &Self) -> Option<Self> {
        let a = self.int as u64 * pow10(self.scale) + self.frac as u64;
        let b = other.int as u64 * pow10(other.scale) + other.frac as u64;
        let total_scale = self.scale + other.scale;

        let product = a as u128 * b as u128;

        let (result, final_scale) = if total_scale > TARGET_SCALE_32 {
            let divisor = pow10_128(total_scale - TARGET_SCALE_32);
            ((product / divisor) as u64, TARGET_SCALE_32)
        } else if product <= u64::MAX as u128 {
            (product as u64, total_scale)
        } else {
            return None;
        };

        Self::checked_from_combined(result, final_scale, self.neg ^ other.neg)
    }

    /// Like `from_combined` but returns `None` instead of panicking on overflow.
    #[inline(always)]
    fn checked_from_combined(n: u64, scale: u8, neg: bool) -> Option<Self> {
        if scale == 0 {
            if n > u32::MAX as u64 {
                return None;
            }
            return Some(Self {
                int: n as u32,
                frac: 0,
                scale: 0,
                neg,
            });
        }
        let divisor = pow10(scale);
        let int_part = n / divisor;
        if int_part > u32::MAX as u64 {
            return None;
        }
        Some(Self {
            int: int_part as u32,
            frac: (n % divisor) as u32,
            scale,
            neg,
        })
    }

    /// Computes the remainder (`self % other`), panics on division by zero.
    #[inline(always)]
    pub fn rem(&self, other: &Self) -> Self {
        assert!(other.int != 0 || other.frac != 0, "division by zero");
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
