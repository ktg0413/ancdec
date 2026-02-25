use super::AncDec32;
use crate::util::pow10;
use crate::wide::isqrt_u128;

impl AncDec32 {
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

    /// Returns the square root with 8 fractional digits of precision. Panics if negative.
    pub fn sqrt(&self) -> Self {
        assert!(!self.neg || self.is_zero(), "square root of negative number");
        if self.is_zero() {
            return Self::ZERO;
        }

        // combined = int * 10^scale + frac (u64)
        let combined = self.int as u64 * pow10(self.scale) + self.frac as u64;

        // N = combined * 10^(16 - scale) -> u128
        // isqrt(N) = floor(sqrt(value) * 10^8)
        let n = combined as u128 * pow10(16 - self.scale) as u128;
        let x = isqrt_u128(n);

        let scale8 = pow10(8);
        Self {
            int: (x / scale8 as u128) as u32,
            frac: (x % scale8 as u128) as u32,
            scale: 8,
            neg: false,
        }
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
