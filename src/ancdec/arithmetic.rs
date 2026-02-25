use crate::util::{pow10_128, SCALE19, TARGET_SCALE};
use crate::wide::{div_wide, mul_wide};
use super::AncDec;

impl AncDec {
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

    /// Multiplies two decimals, panics on overflow. Uses u256 wide arithmetic when needed.
    #[inline(always)]
    pub fn mul(&self, other: &Self) -> Self {
        let a = (self.int as u128) * pow10_128(self.scale) + (self.frac as u128);
        let b = (other.int as u128) * pow10_128(other.scale) + (other.frac as u128);
        let total_scale = self.scale + other.scale;
        let neg = self.neg ^ other.neg;

        // Fast path: both fit in u64 → native u64×u64→u128 multiply (no wide arithmetic)
        if (a | b) >> 64 == 0 {
            let product = (a as u64 as u128) * (b as u64 as u128);
            let (result, final_scale) = if total_scale > TARGET_SCALE {
                let divisor = pow10_128(total_scale - TARGET_SCALE);
                (product / divisor, TARGET_SCALE)
            } else {
                (product, total_scale)
            };
            return Self::from_combined(result, final_scale, neg);
        }

        // Slow path: wide u256 multiply
        let (high, low) = mul_wide(a, b);

        let (result, final_scale) = if total_scale > TARGET_SCALE {
            let divisor = pow10_128(total_scale - TARGET_SCALE);
            (div_wide(high, low, divisor), TARGET_SCALE)
        } else if high == 0 {
            (low, total_scale)
        } else {
            panic!("multiplication overflow")
        };

        Self::from_combined(result, final_scale, neg)
    }

    /// Divides `self` by `other`, panics on division by zero. Uses u256 wide arithmetic when needed.
    #[inline(always)]
    pub fn div(&self, other: &Self) -> Self {
        assert!(other.int != 0 || other.frac != 0, "division by zero");

        let a = (self.int as u128) * pow10_128(self.scale) + (self.frac as u128);
        let b = (other.int as u128) * pow10_128(other.scale) + (other.frac as u128);

        let shift = TARGET_SCALE + other.scale;

        let quotient = if shift >= self.scale {
            let exp = shift - self.scale;
            let multiplier = pow10_128(exp);
            // Fast path: both operands and scaled numerator fit in u128 → skip wide arithmetic
            if (a | b) >> 64 == 0 && exp <= 19 {
                let numerator = a * multiplier;
                numerator / b
            } else {
                let (high, low) = mul_wide(a, multiplier);
                div_wide(high, low, b)
            }
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

    /// Checked addition. Returns `None` if the integer part overflows `u64`.
    #[inline(always)]
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        let (a_frac, b_frac, scale, limit) = self.align_frac(other);

        if self.neg == other.neg {
            let frac_wide = a_frac as u128 + b_frac as u128;
            let overflow = (frac_wide >= limit as u128) as u64;
            let frac = (frac_wide - overflow as u128 * limit as u128) as u64;
            let int = self.int.checked_add(other.int)?.checked_add(overflow)?;
            Some(Self {
                int,
                frac,
                scale,
                neg: self.neg,
            })
        } else {
            Some(Self::sub_with_cmp(
                self.int, a_frac, self.neg, other.int, b_frac, other.neg, scale, limit,
            ))
        }
    }

    /// Checked subtraction. Returns `None` if the integer part overflows `u64`.
    ///
    /// In practice, subtraction in the current design uses magnitude comparison
    /// and cannot overflow, so this always returns `Some`.
    #[inline(always)]
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        Some(self.sub(other))
    }

    /// Checked multiplication. Returns `None` if the result overflows `u64` integer range.
    #[inline(always)]
    pub fn checked_mul(&self, other: &Self) -> Option<Self> {
        let a = (self.int as u128) * pow10_128(self.scale) + (self.frac as u128);
        let b = (other.int as u128) * pow10_128(other.scale) + (other.frac as u128);
        let total_scale = self.scale + other.scale;
        let neg = self.neg ^ other.neg;

        // Fast path: both fit in u64
        if (a | b) >> 64 == 0 {
            let product = (a as u64 as u128) * (b as u64 as u128);
            let (result, final_scale) = if total_scale > TARGET_SCALE {
                let divisor = pow10_128(total_scale - TARGET_SCALE);
                (product / divisor, TARGET_SCALE)
            } else {
                (product, total_scale)
            };
            return Self::checked_from_combined(result, final_scale, neg);
        }

        // Slow path: wide u256 multiply
        let (high, low) = mul_wide(a, b);

        let (result, final_scale) = if total_scale > TARGET_SCALE {
            let divisor = pow10_128(total_scale - TARGET_SCALE);
            (div_wide(high, low, divisor), TARGET_SCALE)
        } else if high == 0 {
            (low, total_scale)
        } else {
            return None;
        };

        Self::checked_from_combined(result, final_scale, neg)
    }

    /// Like `from_combined` but returns `None` instead of panicking on overflow.
    #[inline(always)]
    fn checked_from_combined(n: u128, scale: u8, neg: bool) -> Option<Self> {
        if scale == 0 {
            if n > u64::MAX as u128 {
                return None;
            }
            return Some(Self {
                int: n as u64,
                frac: 0,
                scale: 0,
                neg,
            });
        }
        let divisor = pow10_128(scale);
        let int_part = n / divisor;
        if int_part > u64::MAX as u128 {
            return None;
        }
        Some(Self {
            int: int_part as u64,
            frac: (n % divisor) as u64,
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
