use super::AncDec32;
use crate::util::pow10;
use crate::RoundMode;

impl AncDec32 {
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

        let combined = self.int as u64 * pow10(self.scale) + self.frac as u64;
        let cut = self.scale - decimal_places;
        let divisor = pow10(cut);
        let remainder = combined % divisor;
        let mut truncated = combined / divisor;

        if self.should_round_up(truncated, remainder, divisor, mode) {
            truncated += 1;
        }
        Self::from_combined(truncated, decimal_places, self.neg)
    }

    fn should_round_up(
        &self,
        truncated: u64,
        remainder: u64,
        divisor: u64,
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
