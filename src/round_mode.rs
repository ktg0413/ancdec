/// Rounding modes for decimal operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundMode {
    /// Round toward negative infinity.
    Floor,
    /// Round toward positive infinity.
    Ceil,
    /// Round toward zero (truncate).
    Truncate,
    /// Round half away from zero (>= 0.5 rounds up).
    HalfUp,
    /// Round half toward zero (> 0.5 rounds up).
    HalfDown,
    /// Banker's rounding: half rounds to nearest even.
    HalfEven,
    /// Return the fractional part only.
    Fract,
}
