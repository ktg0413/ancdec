use crate::error::ParseError;
use crate::util::{pow10, StackBuf};
use super::AncDec;
use core::convert::TryFrom;
use core::fmt::Write;
use core::str::FromStr;

impl AncDec {
    /// Converts to `f64` (may lose precision for large values).
    pub fn to_f64(&self) -> f64 {
        let v = self.int as f64
            + if self.scale == 0 {
                0.0
            } else {
                self.frac as f64 / pow10(self.scale) as f64
            };
        if self.neg {
            -v
        } else {
            v
        }
    }

    /// Converts to `i64`, truncating the fractional part. Panics on overflow.
    pub fn to_i64(&self) -> i64 {
        if self.neg {
            assert!(self.int <= i64::MAX as u64 + 1, "integer overflow in to_i64");
            -(self.int as i64)
        } else {
            assert!(self.int <= i64::MAX as u64, "integer overflow in to_i64");
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

/// FromStr trait: enables `"123.45".parse::<AncDec>()`
impl FromStr for AncDec {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl TryFrom<&str> for AncDec {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse_str(s)
    }
}

impl TryFrom<f32> for AncDec {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(n: f32) -> Result<Self, Self::Error> {
        AncDec::try_from(n as f64)
    }
}

impl TryFrom<f64> for AncDec {
    type Error = ParseError;
    #[inline(always)]
    fn try_from(n: f64) -> Result<Self, Self::Error> {
        if n.is_nan() || n.is_infinite() {
            return Err(ParseError::InvalidFloat);
        }
        let mut buf = StackBuf::<64>::new();
        write!(buf, "{}", n).ok();
        Self::parse_str(buf.as_str())
    }
}

/// Lossless widening from AncDec8 (u8) to AncDec (u64)
#[cfg(feature = "dec8")]
impl From<crate::ancdec8::AncDec8> for AncDec {
    #[inline(always)]
    fn from(a: crate::ancdec8::AncDec8) -> Self {
        Self {
            int: a.int as u64,
            frac: a.frac as u64,
            scale: a.scale,
            neg: a.neg,
        }
    }
}

/// Lossless widening from AncDec32 (u32) to AncDec (u64)
#[cfg(feature = "dec32")]
impl From<crate::ancdec32::AncDec32> for AncDec {
    #[inline(always)]
    fn from(a: crate::ancdec32::AncDec32) -> Self {
        Self {
            int: a.int as u64,
            frac: a.frac as u64,
            scale: a.scale,
            neg: a.neg,
        }
    }
}
