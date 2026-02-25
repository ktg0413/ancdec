use core::fmt;

/// Error returned when parsing a string into a decimal type fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// Input string is empty.
    Empty,
    /// No numeric digits found in input.
    NoDigits,
    /// Unexpected characters after the numeric value.
    TrailingChars,
    /// Input float is NaN or Infinity.
    InvalidFloat,
    /// Integer part overflows the target type's range.
    Overflow,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("empty string"),
            Self::NoDigits => f.write_str("no digits found"),
            Self::TrailingChars => f.write_str("trailing characters"),
            Self::InvalidFloat => f.write_str("invalid float (NaN or Infinity)"),
            Self::Overflow => f.write_str("integer overflow during parsing"),
        }
    }
}
