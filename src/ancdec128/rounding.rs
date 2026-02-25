use super::AncDec128;
use crate::util::pow10_128;
use crate::wide::{divmod_u256, mul_wide};
use crate::RoundMode;

impl AncDec128 {
    /// Rounds to the given number of decimal places using the specified mode.
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

        // combined = int * 10^scale + frac as u256
        let (hi, lo) = mul_wide(self.int, pow10_128(self.scale));
        let (lo, carry) = lo.overflowing_add(self.frac);
        let combined = (hi + carry as u128, lo);

        let cut = self.scale - decimal_places;
        let divisor = pow10_128(cut);

        // divmod_u256: combined / divisor -> (truncated_u256, remainder_u128)
        let (truncated_256, remainder) = divmod_u256(combined.0, combined.1, divisor);
        // truncated fits in u128 for practical values (int * 10^decimal_places + frac_part)
        let truncated = truncated_256.1;

        if self.should_round_up(truncated, remainder, divisor, mode) {
            // from_combined with truncated + 1, carry into hi limb on overflow
            let (lo, carry) = truncated.overflowing_add(1);
            Self::from_combined((carry as u128, lo), decimal_places, self.neg)
        } else {
            Self::from_combined((0, truncated), decimal_places, self.neg)
        }
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

    /// Returns the largest integer less than or equal to `self`.
    #[inline(always)]
    pub fn floor(&self) -> Self {
        self.round(0, RoundMode::Floor)
    }
    /// Returns the smallest integer greater than or equal to `self`.
    #[inline(always)]
    pub fn ceil(&self) -> Self {
        self.round(0, RoundMode::Ceil)
    }
    /// Returns the integer part, truncating toward zero.
    #[inline(always)]
    pub fn trunc(&self) -> Self {
        self.round(0, RoundMode::Truncate)
    }
    /// Returns the fractional part only.
    #[inline(always)]
    pub fn fract(&self) -> Self {
        self.round(0, RoundMode::Fract)
    }
}
