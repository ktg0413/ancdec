use super::AncDec128;
use crate::util::{pow10_128, pow10_256};
use crate::wide::{divmod_u256, isqrt_u512, mul_u256, mul_wide};

impl AncDec128 {
    /// Returns the absolute value.
    #[inline(always)]
    pub fn abs(&self) -> Self {
        Self {
            neg: false,
            ..*self
        }
    }

    /// Returns the sign: `1` for positive, `-1` for negative, `0` for zero.
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

    /// Returns `true` if the value is strictly positive.
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        !self.neg && !self.is_zero()
    }

    /// Returns `true` if the value is strictly negative.
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        self.neg && !self.is_zero()
    }

    /// Returns `true` if the value is zero.
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.int == 0 && self.frac == 0
    }

    /// Returns the smaller of `self` and `other`.
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }

    /// Returns the larger of `self` and `other`.
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }

    /// Clamps the value to the range `[min, max]`.
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

    /// Returns the square root with 37 fractional digits of precision. Panics if negative.
    pub fn sqrt(&self) -> Self {
        assert!(!self.neg || self.is_zero(), "square root of negative number");
        if self.is_zero() {
            return Self::ZERO;
        }

        // combined = int * 10^scale + frac as u256
        let (c_hi, c_lo) = mul_wide(self.int, pow10_128(self.scale));
        let (c_lo, carry) = c_lo.overflowing_add(self.frac);
        let combined = (c_hi + carry as u128, c_lo);

        // N = combined * 10^(74 - scale) → u512
        // isqrt(N) = floor(sqrt(value) * 10^37)
        let multiplier = pow10_256(74 - self.scale);
        let (w3, w2, w1, w0) = mul_u256(combined, multiplier);
        let (r_hi, r_lo) = isqrt_u512(w3, w2, w1, w0);

        let scale37 = pow10_128(37);
        let ((_, q), r) = divmod_u256(r_hi, r_lo, scale37);
        Self { int: q, frac: r, scale: 37, neg: false }
    }

    /// Raises `self` to the power `n` using binary exponentiation. Supports negative exponents.
    pub fn pow(&self, n: i32) -> Self {
        if n == 0 {
            return Self::ONE;
        }

        let mut base = if n < 0 {
            assert!(!self.is_zero(), "division by zero in pow with negative exponent");
            Self::ONE.div(self)
        } else {
            *self
        };
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
