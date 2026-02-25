use super::AncDec128;
use crate::util::pow10_128;
use core::fmt;

/// Display trait: enables `format!`, `println!`, `to_string()`
impl fmt::Display for AncDec128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_zero = self.int == 0 && self.frac == 0;
        let sign = if self.neg && !is_zero { "-" } else { "" };

        if let Some(prec) = f.precision() {
            if prec == 0 {
                write!(f, "{}{}", sign, self.int)
            } else if self.scale == 0 {
                write!(f, "{}{}.{:0>w$}", sign, self.int, "", w = prec)
            } else if prec <= self.scale as usize {
                let div = pow10_128(self.scale - prec as u8);
                write!(f, "{}{}.{:0>w$}", sign, self.int, self.frac / div, w = prec)
            } else {
                write!(
                    f,
                    "{}{}.{:0>s$}{:0>p$}",
                    sign,
                    self.int,
                    self.frac,
                    "",
                    s = self.scale as usize,
                    p = prec - self.scale as usize
                )
            }
        } else if self.scale == 0 {
            write!(f, "{}{}", sign, self.int)
        } else {
            write!(
                f,
                "{}{}.{:0>w$}",
                sign,
                self.int,
                self.frac,
                w = self.scale as usize
            )
        }
    }
}

