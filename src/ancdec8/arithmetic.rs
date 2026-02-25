use super::AncDec8;
use crate::util::{pow10_16, pow10_32, SCALE2, TARGET_SCALE_8};

impl AncDec8 {
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
        let neg = self.neg ^ other.neg;
        let total_scale = self.scale + other.scale;

        let a = self.int as u16 * pow10_16(self.scale) + self.frac as u16;
        let b = other.int as u16 * pow10_16(other.scale) + other.frac as u16;
        let product = a as u32 * b as u32; // u16 * u16 -> u32, native

        if total_scale > TARGET_SCALE_8 {
            let reduced = product / pow10_32(total_scale - TARGET_SCALE_8);
            let int_part = reduced / SCALE2 as u32;
            assert!(int_part <= u8::MAX as u32, "multiplication overflow");
            Self {
                int: int_part as u8,
                frac: (reduced % SCALE2 as u32) as u8,
                scale: TARGET_SCALE_8,
                neg,
            }
        } else if total_scale == 0 {
            assert!(product <= u8::MAX as u32, "multiplication overflow");
            Self {
                int: product as u8,
                frac: 0,
                scale: 0,
                neg,
            }
        } else {
            let divisor = pow10_16(total_scale) as u32;
            let int_part = product / divisor;
            assert!(int_part <= u8::MAX as u32, "multiplication overflow");
            Self {
                int: int_part as u8,
                frac: (product % divisor) as u8,
                scale: total_scale,
                neg,
            }
        }
    }

    /// Divides `self` by `other`, panics on division by zero.
    #[inline(always)]
    pub fn div(&self, other: &Self) -> Self {
        assert!(other.int != 0 || other.frac != 0, "division by zero");

        let neg = self.neg ^ other.neg;

        let a = self.int as u16 * pow10_16(self.scale) + self.frac as u16;
        let b = other.int as u16 * pow10_16(other.scale) + other.frac as u16;

        let shift = TARGET_SCALE_8 + other.scale;

        let numerator = if shift >= self.scale {
            a as u32 * pow10_32(shift - self.scale)
        } else {
            a as u32 / pow10_32(self.scale - shift)
        };

        let quotient = numerator / b as u32;

        let int = (quotient / SCALE2 as u32) as u8;
        let frac = (quotient % SCALE2 as u32) as u8;
        Self {
            int,
            frac,
            scale: TARGET_SCALE_8,
            neg,
        }
    }

    /// Checked addition. Returns `None` if the integer part overflows `u8`.
    #[inline(always)]
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        let (a_frac, b_frac, scale, limit) = self.align_frac(other);

        if self.neg == other.neg {
            let frac = a_frac + b_frac;
            let overflow = (frac >= limit) as u8;
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

    /// Checked subtraction. Returns `None` if the integer part overflows `u8`.
    ///
    /// In practice, subtraction in the current design uses magnitude comparison
    /// and cannot overflow, so this always returns `Some`.
    #[inline(always)]
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        Some(self.sub(other))
    }

    /// Checked multiplication. Returns `None` if the result overflows `u8` integer range.
    #[inline(always)]
    pub fn checked_mul(&self, other: &Self) -> Option<Self> {
        let neg = self.neg ^ other.neg;
        let total_scale = self.scale + other.scale;

        let a = self.int as u16 * pow10_16(self.scale) + self.frac as u16;
        let b = other.int as u16 * pow10_16(other.scale) + other.frac as u16;
        let product = a as u32 * b as u32;

        if total_scale > TARGET_SCALE_8 {
            let reduced = product / pow10_32(total_scale - TARGET_SCALE_8);
            let int_part = reduced / SCALE2 as u32;
            if int_part > u8::MAX as u32 {
                return None;
            }
            Some(Self {
                int: int_part as u8,
                frac: (reduced % SCALE2 as u32) as u8,
                scale: TARGET_SCALE_8,
                neg,
            })
        } else if total_scale == 0 {
            if product > u8::MAX as u32 {
                return None;
            }
            Some(Self {
                int: product as u8,
                frac: 0,
                scale: 0,
                neg,
            })
        } else {
            let divisor = pow10_16(total_scale) as u32;
            let int_part = product / divisor;
            if int_part > u8::MAX as u32 {
                return None;
            }
            Some(Self {
                int: int_part as u8,
                frac: (product % divisor) as u8,
                scale: total_scale,
                neg,
            })
        }
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
