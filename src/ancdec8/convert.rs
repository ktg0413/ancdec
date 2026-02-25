use super::AncDec8;
use crate::error::ParseError;
use crate::util::{pow10_u8, StackBuf};
use core::convert::TryFrom;
use core::fmt::Write;
use core::str::FromStr;

impl AncDec8 {
    /// Converts to `f64` (may lose precision).
    pub fn to_f64(&self) -> f64 {
        let v = self.int as f64
            + if self.scale == 0 {
                0.0
            } else {
                self.frac as f64 / pow10_u8(self.scale) as f64
            };
        if self.neg {
            -v
        } else {
            v
        }
    }

    /// Converts to `i64`, truncating the fractional part.
    pub fn to_i64(&self) -> i64 {
        if self.neg {
            -(self.int as i64)
        } else {
            self.int as i64
        }
    }

    /// Converts to `i128`, truncating the fractional part.
    pub fn to_i128(&self) -> i128 {
        if self.neg {
            -(self.int as i128)
        } else {
            self.int as i128
        }
    }
}

/// FromStr trait: enables `"1.23".parse::<AncDec8>()`
impl FromStr for AncDec8 {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl TryFrom<&str> for AncDec8 {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse_str(s)
    }
}

impl TryFrom<f32> for AncDec8 {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(n: f32) -> Result<Self, Self::Error> {
        AncDec8::try_from(n as f64)
    }
}

impl TryFrom<f64> for AncDec8 {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(n: f64) -> Result<Self, Self::Error> {
        if n.is_nan() || n.is_infinite() {
            return Err(ParseError::InvalidFloat);
        }
        let mut buf = StackBuf::<16>::new();
        write!(buf, "{}", n).ok();
        Self::parse_str(buf.as_str())
    }
}
