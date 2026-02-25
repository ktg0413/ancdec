use super::AncDec32;
use crate::error::ParseError;
use crate::util::{pow10_32, StackBuf};
use core::convert::TryFrom;
use core::fmt::Write;
use core::str::FromStr;

impl AncDec32 {
    /// Converts to `f64` (may lose precision).
    pub fn to_f64(&self) -> f64 {
        let v = self.int as f64
            + if self.scale == 0 {
                0.0
            } else {
                self.frac as f64 / pow10_32(self.scale) as f64
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

/// FromStr trait: enables `"123.45".parse::<AncDec32>()`
impl FromStr for AncDec32 {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl TryFrom<&str> for AncDec32 {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse_str(s)
    }
}

impl TryFrom<f32> for AncDec32 {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(n: f32) -> Result<Self, Self::Error> {
        AncDec32::try_from(n as f64)
    }
}

impl TryFrom<f64> for AncDec32 {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(n: f64) -> Result<Self, Self::Error> {
        if n.is_nan() || n.is_infinite() {
            return Err(ParseError::InvalidFloat);
        }
        let mut buf = StackBuf::<32>::new();
        write!(buf, "{}", n).ok();
        Self::parse_str(buf.as_str())
    }
}

/// Lossless widening from AncDec8 (u8) to AncDec32 (u32)
#[cfg(feature = "dec8")]
impl From<crate::ancdec8::AncDec8> for AncDec32 {
    #[inline(always)]
    fn from(a: crate::ancdec8::AncDec8) -> Self {
        Self {
            int: a.int as u32,
            frac: a.frac as u32,
            scale: a.scale,
            neg: a.neg,
        }
    }
}
