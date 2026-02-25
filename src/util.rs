use core::fmt::{self, Write};

#[cfg(feature = "dec8")]
pub(crate) const TARGET_SCALE_8: u8 = 2; // max fractional digits for AncDec8
#[cfg(feature = "dec8")]
pub(crate) const SCALE2: u16 = 100; // 10^2: for splitting AncDec8 div result

#[cfg(feature = "dec32")]
pub(crate) const TARGET_SCALE_32: u8 = 9; // max fractional digits for AncDec32
#[cfg(feature = "dec32")]
pub(crate) const SCALE9: u64 = 1_000_000_000; // 10^9: for splitting AncDec32 div result

#[cfg(feature = "dec64")]
pub(crate) const SCALE19: u128 = 10_000_000_000_000_000_000; // 10^19: for splitting div result
#[cfg(feature = "dec64")]
pub(crate) const TARGET_SCALE: u8 = 19; // max fractional digits for mul/div

#[cfg(feature = "dec128")]
pub(crate) const SCALE38: u128 = 100_000_000_000_000_000_000_000_000_000_000_000_000; // 10^38
#[cfg(feature = "dec128")]
pub(crate) const TARGET_SCALE_128: u8 = 38; // max fractional digits for AncDec128

/// Heap-free buffer for Display -> &str. Safety: only write_str can write, which guarantees UTF-8.
pub(crate) struct StackBuf<const N: usize> {
    buf: [u8; N],
    pos: usize, // invariant: buf[..pos] is valid UTF-8
}

impl<const N: usize> StackBuf<N> {
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self {
            buf: [0; N],
            pos: 0,
        }
    }

    #[inline(always)]
    pub(crate) fn as_str(&self) -> &str {
        // SAFETY: write_str only accepts &str (valid UTF-8), so buf[..pos] is always valid
        debug_assert!(core::str::from_utf8(&self.buf[..self.pos]).is_ok());
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.pos]) }
    }
}

/// Write trait: enables `write!` macro usage
impl<const N: usize> Write for StackBuf<N> {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let remaining = N - self.pos;

        // truncate at UTF-8 boundary if buffer full
        let len = if bytes.len() <= remaining {
            bytes.len()
        } else {
            let mut i = remaining;
            while i > 0 && !s.is_char_boundary(i) {
                i -= 1;
            }
            i
        };

        self.buf[self.pos..self.pos + len].copy_from_slice(&bytes[..len]);
        self.pos += len;
        Ok(())
    }
}

/// Power of 10 lookup for u8 (0-2)
#[cfg(feature = "dec8")]
#[inline(always)]
pub(crate) const fn pow10_u8(exp: u8) -> u8 {
    match exp {
        0 => 1,
        1 => 10,
        2 => 100,
        _ => panic!("scale overflow"),
    }
}

/// Power of 10 lookup for u16 (0-4)
#[cfg(feature = "dec8")]
#[inline(always)]
pub(crate) const fn pow10_16(exp: u8) -> u16 {
    match exp {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1_000,
        4 => 10_000,
        _ => panic!("scale overflow"),
    }
}

/// Power of 10 lookup for u32 (0-9)
#[cfg(any(feature = "dec8", feature = "dec32"))]
#[inline(always)]
pub(crate) const fn pow10_32(exp: u8) -> u32 {
    match exp {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1_000,
        4 => 10_000,
        5 => 100_000,
        6 => 1_000_000,
        7 => 10_000_000,
        8 => 100_000_000,
        9 => 1_000_000_000,
        _ => panic!("scale overflow"),
    }
}

/// Power of 10 lookup for u64 (0-19)
#[cfg(any(feature = "dec32", feature = "dec64", feature = "dec128"))]
#[inline(always)]
pub(crate) const fn pow10(exp: u8) -> u64 {
    match exp {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1_000,
        4 => 10_000,
        5 => 100_000,
        6 => 1_000_000,
        7 => 10_000_000,
        8 => 100_000_000,
        9 => 1_000_000_000,
        10 => 10_000_000_000,
        11 => 100_000_000_000,
        12 => 1_000_000_000_000,
        13 => 10_000_000_000_000,
        14 => 100_000_000_000_000,
        15 => 1_000_000_000_000_000,
        16 => 10_000_000_000_000_000,
        17 => 100_000_000_000_000_000,
        18 => 1_000_000_000_000_000_000,
        19 => 10_000_000_000_000_000_000,
        _ => panic!("scale overflow"),
    }
}

/// Power of 10 lookup for u128 (0-38)
#[cfg(any(feature = "dec32", feature = "dec64", feature = "dec128"))]
#[inline(always)]
pub(crate) const fn pow10_128(exp: u8) -> u128 {
    match exp {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1_000,
        4 => 10_000,
        5 => 100_000,
        6 => 1_000_000,
        7 => 10_000_000,
        8 => 100_000_000,
        9 => 1_000_000_000,
        10 => 10_000_000_000,
        11 => 100_000_000_000,
        12 => 1_000_000_000_000,
        13 => 10_000_000_000_000,
        14 => 100_000_000_000_000,
        15 => 1_000_000_000_000_000,
        16 => 10_000_000_000_000_000,
        17 => 100_000_000_000_000_000,
        18 => 1_000_000_000_000_000_000,
        19 => 10_000_000_000_000_000_000,
        20 => 100_000_000_000_000_000_000,
        21 => 1_000_000_000_000_000_000_000,
        22 => 10_000_000_000_000_000_000_000,
        23 => 100_000_000_000_000_000_000_000,
        24 => 1_000_000_000_000_000_000_000_000,
        25 => 10_000_000_000_000_000_000_000_000,
        26 => 100_000_000_000_000_000_000_000_000,
        27 => 1_000_000_000_000_000_000_000_000_000,
        28 => 10_000_000_000_000_000_000_000_000_000,
        29 => 100_000_000_000_000_000_000_000_000_000,
        30 => 1_000_000_000_000_000_000_000_000_000_000,
        31 => 10_000_000_000_000_000_000_000_000_000_000,
        32 => 100_000_000_000_000_000_000_000_000_000_000,
        33 => 1_000_000_000_000_000_000_000_000_000_000_000,
        34 => 10_000_000_000_000_000_000_000_000_000_000_000,
        35 => 100_000_000_000_000_000_000_000_000_000_000_000,
        36 => 1_000_000_000_000_000_000_000_000_000_000_000_000,
        37 => 10_000_000_000_000_000_000_000_000_000_000_000_000,
        38 => 100_000_000_000_000_000_000_000_000_000_000_000_000,
        _ => panic!("scale overflow"),
    }
}

/// Power of 10 as u256 (0-76), returns (high_u128, low_u128)
#[cfg(feature = "dec128")]
#[inline(always)]
pub(crate) const fn pow10_256(exp: u8) -> (u128, u128) {
    if exp <= 38 {
        (0, pow10_128(exp))
    } else {
        // exp 39..=76: split as 10^38 * 10^(exp-38)
        // Use inline Karatsuba since mul_wide is not const fn
        let a = pow10_128(38);
        let b = pow10_128(exp - 38);
        let a_lo = a as u64 as u128;
        let a_hi = a >> 64;
        let b_lo = b as u64 as u128;
        let b_hi = b >> 64;
        let ll = a_lo * b_lo;
        let hl = a_hi * b_lo;
        let lh = a_lo * b_hi;
        let hh = a_hi * b_hi;
        let mid = hl + lh;
        let mid_carry = if mid < hl { 1u128 } else { 0u128 };
        let low = ll.wrapping_add(mid << 64);
        let carry = if low < ll { 1u128 } else { 0u128 };
        let high = hh + (mid >> 64) + (mid_carry << 64) + carry;
        (high, low)
    }
}
