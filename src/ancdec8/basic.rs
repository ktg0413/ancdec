use super::AncDec8;
use crate::util::{pow10_u8, pow10_16, pow10_32};

/// Integer square root of a u32 value, returns u16
#[inline(always)]
fn isqrt_u32(n: u32) -> u16 {
    if n <= 1 {
        return n as u16;
    }
    let mut x = 1u32 << (32 - n.leading_zeros()).div_ceil(2);
    loop {
        let q = n / x;
        let x_new = (x >> 1) + (q >> 1) + (x & q & 1);
        if x_new >= x {
            break;
        }
        x = x_new;
    }
    if x * x > n {
        x -= 1;
    }
    x as u16
}

impl AncDec8 {
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

    /// Returns the square root with 1 fractional digit of precision. Panics if negative.
    pub fn sqrt(&self) -> Self {
        assert!(!self.neg || self.is_zero(), "square root of negative number");
        if self.is_zero() {
            return Self::ZERO;
        }

        // combined = int * 10^scale + frac as u16
        let combined = self.int as u16 * pow10_16(self.scale) + self.frac as u16;

        // N = combined * 10^(2 - scale) -> u32
        // isqrt(N) = floor(sqrt(value) * 10^1)
        let n = combined as u32 * pow10_32(2 - self.scale);
        let x = isqrt_u32(n);

        let int = (x / pow10_u8(1) as u16) as u8;
        let frac = (x % pow10_u8(1) as u16) as u8;
        Self {
            int,
            frac,
            scale: 1,
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
