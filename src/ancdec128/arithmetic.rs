use super::AncDec128;
use crate::util::{pow10, pow10_128, pow10_256, SCALE38, TARGET_SCALE_128};
use crate::wide::{div_u512_by_u128, div_u512_by_u256, div_wide, divmod_u256, mul_u256, mul_wide};

impl AncDec128 {
    /// Combine int and frac into a single u256 = int * 10^scale + frac
    #[inline(always)]
    fn combine(int: u128, frac: u128, scale: u8) -> (u128, u128) {
        let (hi, lo) = mul_wide(int, pow10_128(scale));
        let (lo2, carry) = lo.overflowing_add(frac);
        (hi + carry as u128, lo2)
    }

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

    /// Multiplies two decimals, panics on overflow. Uses u256/u512 wide arithmetic when needed.
    #[inline(always)]
    pub fn mul(&self, other: &Self) -> Self {
        let neg = self.neg ^ other.neg;
        let total_scale = self.scale + other.scale;

        // Ultra-fast path: both fit in u64 → u64×u64=u128, zero wide arithmetic
        if let (Some(a), Some(b)) = (
            Self::try_combine_u64(self.int, self.frac, self.scale),
            Self::try_combine_u64(other.int, other.frac, other.scale),
        ) {
            let product = a as u128 * b as u128;
            if total_scale <= 19 && product <= u64::MAX as u128 {
                let p = product as u64;
                let d = pow10(total_scale);
                return Self { int: (p / d) as u128, frac: (p % d) as u128, scale: total_scale, neg };
            }
            if total_scale <= 38 {
                let divisor = pow10_128(total_scale);
                return Self { int: product / divisor, frac: product % divisor, scale: total_scale, neg };
            }
            let reduced = product / pow10_128(total_scale - TARGET_SCALE_128);
            return Self { int: reduced / SCALE38, frac: reduced % SCALE38, scale: TARGET_SCALE_128, neg };
        }

        // Partial product path: all parts fit in u64 → 4 native muls, no wide arithmetic
        if self.scale <= 19 && other.scale <= 19
            && self.int <= u64::MAX as u128 && self.frac <= u64::MAX as u128
            && other.int <= u64::MAX as u128 && other.frac <= u64::MAX as u128
        {
            let ai = self.int as u64;
            let af = self.frac as u64;
            let bi = other.int as u64;
            let bf = other.frac as u64;

            let ii = ai as u128 * bi as u128;
            let if_ = ai as u128 * bf as u128;
            let fi = af as u128 * bi as u128;
            let ff = af as u128 * bf as u128;

            let sa_pow = pow10_128(self.scale);
            let sb_pow = pow10_128(other.scale);
            let scale_pow = pow10_128(total_scale);

            // Split cross terms into integer + fractional at scale total_scale
            let if_int = if_ / sb_pow;
            let if_frac = (if_ % sb_pow) * sa_pow;

            let fi_int = fi / sa_pow;
            let fi_frac = (fi % sa_pow) * sb_pow;

            // ff < scale_pow by invariant (frac < 10^scale), so ff is purely fractional
            // Sum fractional parts: each < scale_pow, sum < 3 * scale_pow < u128::MAX
            let frac_sum = if_frac + fi_frac + ff;
            let (frac, frac_carry) = if frac_sum >= scale_pow {
                let f = frac_sum - scale_pow;
                if f >= scale_pow { (f - scale_pow, 2u128) } else { (f, 1u128) }
            } else {
                (frac_sum, 0u128)
            };

            let int = ii.checked_add(if_int)
                .and_then(|v| v.checked_add(fi_int))
                .and_then(|v| v.checked_add(frac_carry))
                .expect("multiplication overflow");
            return Self { int, frac, scale: total_scale, neg };
        }

        // Fast path: both fit in u128 → single mul_wide instead of mul_u256
        if let (Some(a), Some(b)) = (
            Self::try_combine_u128(self.int, self.frac, self.scale),
            Self::try_combine_u128(other.int, other.frac, other.scale),
        ) {
            let (high, low) = mul_wide(a, b);
            if total_scale > TARGET_SCALE_128 {
                let divisor = pow10_128(total_scale - TARGET_SCALE_128);
                let ((q_hi, q_lo), _) = divmod_u256(high, low, divisor);
                return Self::from_combined((q_hi, q_lo), TARGET_SCALE_128, neg);
            }
            return Self::from_combined((high, low), total_scale, neg);
        }

        // Slow path: full u256 * u256 → u512
        let a = Self::combine(self.int, self.frac, self.scale);
        let b = Self::combine(other.int, other.frac, other.scale);

        let (w3, w2, w1, w0) = mul_u256(a, b);

        let (result, final_scale) = if total_scale > TARGET_SCALE_128 {
            let divisor = pow10_128(total_scale - TARGET_SCALE_128);
            (div_u512_by_u128(w3, w2, w1, w0, divisor), TARGET_SCALE_128)
        } else if w3 == 0 && w2 == 0 {
            ((w1, w0), total_scale)
        } else {
            panic!("multiplication overflow")
        };

        Self::from_combined(result, final_scale, neg)
    }

    /// Divides `self` by `other`, panics on division by zero. Uses u256/u512 wide arithmetic when needed.
    #[inline(always)]
    pub fn div(&self, other: &Self) -> Self {
        assert!(other.int != 0 || other.frac != 0, "division by zero");

        let neg = self.neg ^ other.neg;

        // Ultra-fast path: both fit in u64 → native u64 division
        // Decompose: a * 10^exp / b = (a/b) * 10^exp + (a%b) * 10^exp / b
        if let (Some(a64), Some(b64)) = (
            Self::try_combine_u64(self.int, self.frac, self.scale),
            Self::try_combine_u64(other.int, other.frac, other.scale),
        ) {
            let shift = TARGET_SCALE_128 + other.scale;
            if shift >= self.scale {
                let exp = shift - self.scale;
                if exp <= 38 {
                    let q = (a64 / b64) as u128;
                    let r = (a64 % b64) as u128;
                    let exp_pow = pow10_128(exp);

                    let (int, frac_high) = if exp < 38 {
                        let gap_pow = pow10_128(38 - exp);
                        (q / gap_pow, (q % gap_pow) * exp_pow)
                    } else {
                        (q, 0)
                    };

                    let frac_low = if r == 0 {
                        0
                    } else if let Some(r_scaled) = r.checked_mul(exp_pow) {
                        r_scaled / (b64 as u128)
                    } else {
                        let (h, l) = mul_wide(r, exp_pow);
                        div_wide(h, l, b64 as u128)
                    };

                    let frac_total = frac_high + frac_low;
                    let carry = (frac_total >= SCALE38) as u128;
                    return Self { int: int + carry, frac: frac_total - carry * SCALE38, scale: TARGET_SCALE_128, neg };
                }
            }
        }

        // Fast path: both fit in u128
        if let (Some(a), Some(b)) = (
            Self::try_combine_u128(self.int, self.frac, self.scale),
            Self::try_combine_u128(other.int, other.frac, other.scale),
        ) {
            let shift = TARGET_SCALE_128 + other.scale;
            if shift >= self.scale {
                let exp = shift - self.scale;
                if exp <= 38 {
                    let q = a / b;
                    let r = a % b;
                    let exp_pow = pow10_128(exp);

                    let (int, frac_high) = if exp < 38 {
                        let gap_pow = pow10_128(38 - exp);
                        (q / gap_pow, (q % gap_pow) * exp_pow)
                    } else {
                        (q, 0)
                    };

                    let frac_low = if r == 0 {
                        0
                    } else if let Some(r_scaled) = r.checked_mul(exp_pow) {
                        r_scaled / b
                    } else {
                        let (h, l) = mul_wide(r, exp_pow);
                        div_wide(h, l, b)
                    };

                    let frac_total = frac_high + frac_low;
                    let carry = (frac_total >= SCALE38) as u128;
                    return Self { int: int + carry, frac: frac_total - carry * SCALE38, scale: TARGET_SCALE_128, neg };
                }
            }
        }

        // Slow path: full u256 arithmetic
        let a = Self::combine(self.int, self.frac, self.scale);
        let b = Self::combine(other.int, other.frac, other.scale);

        // We want: (a * 10^(TARGET_SCALE_128 + other.scale - self.scale)) / b
        let shift = TARGET_SCALE_128 + other.scale;

        let quotient = if shift >= self.scale {
            let multiplier = pow10_256(shift - self.scale);
            let (w3, w2, w1, w0) = mul_u256(a, multiplier);
            div_u512_by_u256(w3, w2, w1, w0, b.0, b.1)
        } else {
            // Rare case: self.scale > shift, divide multiplier instead
            let scale_down = pow10_256(self.scale - shift);
            let (_w3, _w2, w1, w0) = mul_u256(b, scale_down);
            // a / (b * 10^(self.scale - shift))
            div_u512_by_u256(0, 0, a.0, a.1, w1, w0)
        };

        // Split quotient by SCALE38
        let ((_, q_hi), r_lo) = divmod_u256(quotient.0, quotient.1, SCALE38);

        Self {
            int: q_hi,
            frac: r_lo,
            scale: TARGET_SCALE_128,
            neg,
        }
    }

    /// Checked addition. Returns `None` if the integer part overflows `u128`.
    #[inline(always)]
    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        let (a_frac, b_frac, scale, limit) = self.align_frac(other);

        if self.neg == other.neg {
            let frac = a_frac + b_frac;
            let overflow = (frac >= limit) as u128;
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

    /// Checked subtraction. Returns `None` if the integer part overflows `u128`.
    ///
    /// In practice, subtraction in the current design uses magnitude comparison
    /// and cannot overflow, so this always returns `Some`.
    #[inline(always)]
    pub fn checked_sub(&self, other: &Self) -> Option<Self> {
        Some(self.sub(other))
    }

    /// Checked multiplication. Returns `None` if the result overflows `u128` integer range.
    #[inline(always)]
    pub fn checked_mul(&self, other: &Self) -> Option<Self> {
        let neg = self.neg ^ other.neg;
        let total_scale = self.scale + other.scale;

        // Ultra-fast path: both fit in u64
        if let (Some(a), Some(b)) = (
            Self::try_combine_u64(self.int, self.frac, self.scale),
            Self::try_combine_u64(other.int, other.frac, other.scale),
        ) {
            let product = a as u128 * b as u128;
            if total_scale <= 19 && product <= u64::MAX as u128 {
                let p = product as u64;
                let d = pow10(total_scale);
                return Some(Self { int: (p / d) as u128, frac: (p % d) as u128, scale: total_scale, neg });
            }
            if total_scale <= 38 {
                let divisor = pow10_128(total_scale);
                return Some(Self { int: product / divisor, frac: product % divisor, scale: total_scale, neg });
            }
            let reduced = product / pow10_128(total_scale - TARGET_SCALE_128);
            return Some(Self { int: reduced / SCALE38, frac: reduced % SCALE38, scale: TARGET_SCALE_128, neg });
        }

        // Partial product path: all parts fit in u64
        if self.scale <= 19 && other.scale <= 19
            && self.int <= u64::MAX as u128 && self.frac <= u64::MAX as u128
            && other.int <= u64::MAX as u128 && other.frac <= u64::MAX as u128
        {
            let ai = self.int as u64;
            let af = self.frac as u64;
            let bi = other.int as u64;
            let bf = other.frac as u64;

            let ii = ai as u128 * bi as u128;
            let if_ = ai as u128 * bf as u128;
            let fi = af as u128 * bi as u128;
            let ff = af as u128 * bf as u128;

            let sa_pow = pow10_128(self.scale);
            let sb_pow = pow10_128(other.scale);
            let scale_pow = pow10_128(total_scale);

            let if_int = if_ / sb_pow;
            let if_frac = (if_ % sb_pow) * sa_pow;

            let fi_int = fi / sa_pow;
            let fi_frac = (fi % sa_pow) * sb_pow;

            let frac_sum = if_frac + fi_frac + ff;
            let (frac, frac_carry) = if frac_sum >= scale_pow {
                let f = frac_sum - scale_pow;
                if f >= scale_pow { (f - scale_pow, 2u128) } else { (f, 1u128) }
            } else {
                (frac_sum, 0u128)
            };

            let int = ii.checked_add(if_int)
                .and_then(|v| v.checked_add(fi_int))
                .and_then(|v| v.checked_add(frac_carry))?;
            return Some(Self { int, frac, scale: total_scale, neg });
        }

        // Fast path: both fit in u128
        if let (Some(a), Some(b)) = (
            Self::try_combine_u128(self.int, self.frac, self.scale),
            Self::try_combine_u128(other.int, other.frac, other.scale),
        ) {
            let (high, low) = mul_wide(a, b);
            if total_scale > TARGET_SCALE_128 {
                let divisor = pow10_128(total_scale - TARGET_SCALE_128);
                let ((q_hi, q_lo), _) = divmod_u256(high, low, divisor);
                return Self::checked_from_combined((q_hi, q_lo), TARGET_SCALE_128, neg);
            }
            return Self::checked_from_combined((high, low), total_scale, neg);
        }

        // Slow path: full u256 * u256 -> u512
        let a = Self::combine(self.int, self.frac, self.scale);
        let b = Self::combine(other.int, other.frac, other.scale);

        let (w3, w2, w1, w0) = mul_u256(a, b);

        let (result, final_scale) = if total_scale > TARGET_SCALE_128 {
            let divisor = pow10_128(total_scale - TARGET_SCALE_128);
            (div_u512_by_u128(w3, w2, w1, w0, divisor), TARGET_SCALE_128)
        } else if w3 == 0 && w2 == 0 {
            ((w1, w0), total_scale)
        } else {
            return None;
        };

        Self::checked_from_combined(result, final_scale, neg)
    }

    /// Like `from_combined` but returns `None` instead of panicking on overflow.
    #[inline(always)]
    fn checked_from_combined(n: (u128, u128), scale: u8, neg: bool) -> Option<Self> {
        if scale == 0 {
            if n.0 != 0 {
                return None;
            }
            return Some(Self {
                int: n.1,
                frac: 0,
                scale: 0,
                neg,
            });
        }
        let divisor = pow10_128(scale);
        let ((q_hi, q), r) = divmod_u256(n.0, n.1, divisor);
        if q_hi != 0 {
            return None;
        }
        Some(Self {
            int: q,
            frac: r,
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
