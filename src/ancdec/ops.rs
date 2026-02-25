use crate::util::pow10_128;
use super::AncDec;
use core::hash::{Hash, Hasher};
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

// ============ Operator Traits ============
/// Add trait: enables `a + b`
impl Add for AncDec {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self.add(&rhs)
    }
}

/// Sub trait: enables `a - b`
impl Sub for AncDec {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        self.sub(&rhs)
    }
}

/// Mul trait: enables `a * b`
impl Mul for AncDec {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        self.mul(&rhs)
    }
}

/// Div trait: enables `a / b`
impl Div for AncDec {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        self.div(&rhs)
    }
}

/// Rem trait: enables `a % b`
impl Rem for AncDec {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        self.rem(&rhs)
    }
}

/// Neg trait: enables `-a`
impl Neg for AncDec {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            neg: !self.neg,
            ..self
        }
    }
}

// ============ Assign Ops ============
/// AddAssign: enables `a += b`
impl AddAssign for AncDec {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(&rhs);
    }
}

/// SubAssign: enables `a -= b`
impl SubAssign for AncDec {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(&rhs);
    }
}

/// MulAssign: enables `a *= b`
impl MulAssign for AncDec {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(&rhs);
    }
}

/// DivAssign: enables `a /= b`
impl DivAssign for AncDec {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(&rhs);
    }
}

/// RemAssign: enables `a %= b`
impl RemAssign for AncDec {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(&rhs);
    }
}

// ============ Reference Ops ============
// All combinations: &T op &T, T op &T, &T op T
impl<'b> Add<&'b AncDec> for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn add(self, rhs: &'b AncDec) -> AncDec {
        self.add(rhs)
    }
}
impl<'a> Add<&'a AncDec> for AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn add(self, rhs: &'a AncDec) -> AncDec {
        AncDec::add(&self, rhs)
    }
}
impl Add<AncDec> for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn add(self, rhs: AncDec) -> AncDec {
        self.add(&rhs)
    }
}

impl<'b> Sub<&'b AncDec> for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn sub(self, rhs: &'b AncDec) -> AncDec {
        self.sub(rhs)
    }
}
impl<'a> Sub<&'a AncDec> for AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn sub(self, rhs: &'a AncDec) -> AncDec {
        AncDec::sub(&self, rhs)
    }
}
impl Sub<AncDec> for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn sub(self, rhs: AncDec) -> AncDec {
        self.sub(&rhs)
    }
}

impl<'b> Mul<&'b AncDec> for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn mul(self, rhs: &'b AncDec) -> AncDec {
        self.mul(rhs)
    }
}
impl<'a> Mul<&'a AncDec> for AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn mul(self, rhs: &'a AncDec) -> AncDec {
        AncDec::mul(&self, rhs)
    }
}
impl Mul<AncDec> for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn mul(self, rhs: AncDec) -> AncDec {
        self.mul(&rhs)
    }
}

impl<'b> Div<&'b AncDec> for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn div(self, rhs: &'b AncDec) -> AncDec {
        self.div(rhs)
    }
}
impl<'a> Div<&'a AncDec> for AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn div(self, rhs: &'a AncDec) -> AncDec {
        AncDec::div(&self, rhs)
    }
}
impl Div<AncDec> for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn div(self, rhs: AncDec) -> AncDec {
        self.div(&rhs)
    }
}

impl<'b> Rem<&'b AncDec> for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn rem(self, rhs: &'b AncDec) -> AncDec {
        self.rem(rhs)
    }
}
impl<'a> Rem<&'a AncDec> for AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn rem(self, rhs: &'a AncDec) -> AncDec {
        AncDec::rem(&self, rhs)
    }
}
impl Rem<AncDec> for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn rem(self, rhs: AncDec) -> AncDec {
        self.rem(&rhs)
    }
}

impl Neg for &AncDec {
    type Output = AncDec;
    #[inline(always)]
    fn neg(self) -> AncDec {
        AncDec {
            neg: !self.neg,
            ..*self
        }
    }
}

// ============ Default ============
/// Default trait: `AncDec::default()` returns ZERO
impl Default for AncDec {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

// ============ Hash ============
/// Hash trait: enables use in HashMap/HashSet
/// Normalizes trailing zeros so 1.0 == 1.00 have same hash
impl Hash for AncDec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut combined = (self.int as u128) * pow10_128(self.scale) + (self.frac as u128);
        // Binary search: strip trailing zeros in O(log scale) instead of O(scale)
        if combined > 0 {
            if combined % 10_000_000_000_000_000 == 0 { combined /= 10_000_000_000_000_000; }
            if combined % 100_000_000 == 0 { combined /= 100_000_000; }
            if combined % 10_000 == 0 { combined /= 10_000; }
            if combined % 100 == 0 { combined /= 100; }
            if combined % 10 == 0 { combined /= 10; }
        }
        combined.hash(state);
        if combined != 0 {
            self.neg.hash(state);
        } // 0 == -0
    }
}

// ============ Iterator Traits ============
/// Sum trait: enables `iter.sum::<AncDec>()`
impl Sum for AncDec {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, x| a.add(&x))
    }
}
impl<'a> Sum<&'a AncDec> for AncDec {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, x| a.add(x))
    }
}

/// Product trait: enables `iter.product::<AncDec>()`
impl Product for AncDec {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, x| a.mul(&x))
    }
}
impl<'a> Product<&'a AncDec> for AncDec {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, x| a.mul(x))
    }
}

// ============ From Integer ============
/// From<signed>: negative values set neg=true
macro_rules! impl_from_signed { ($($t:ty),*) => {$( impl From<$t> for AncDec { #[inline(always)] fn from(n: $t) -> Self { Self { int: n.unsigned_abs() as u64, frac: 0, scale: 0, neg: n < 0 } } } )*}; }

/// From<unsigned>: always positive
macro_rules! impl_from_unsigned { ($($t:ty),*) => {$( impl From<$t> for AncDec { #[inline(always)] fn from(n: $t) -> Self { Self { int: n as u64, frac: 0, scale: 0, neg: false } } } )*}; }

impl_from_signed!(i8, i16, i32, i64, isize);
impl_from_unsigned!(u8, u16, u32, u64, usize);

impl From<u128> for AncDec {
    #[inline(always)]
    fn from(n: u128) -> Self {
        assert!(n <= u64::MAX as u128, "u128 value exceeds AncDec range");
        Self { int: n as u64, frac: 0, scale: 0, neg: false }
    }
}

impl From<i128> for AncDec {
    #[inline(always)]
    fn from(n: i128) -> Self {
        let abs = n.unsigned_abs();
        assert!(abs <= u64::MAX as u128, "i128 value exceeds AncDec range");
        Self { int: abs as u64, frac: 0, scale: 0, neg: n < 0 }
    }
}

// ============ Ops with Primitives ============
/// Enables `AncDec + i32`, `i32 + AncDec`, etc.
macro_rules! impl_ops_primitive {
    ($($t:ty),*) => {$(
        impl Add<$t> for AncDec { type Output = AncDec; #[inline(always)] fn add(self, rhs: $t) -> AncDec { self.add(&AncDec::from(rhs)) } }
        impl Add<AncDec> for $t { type Output = AncDec; #[inline(always)] fn add(self, rhs: AncDec) -> AncDec { AncDec::from(self).add(&rhs) } }
        impl Sub<$t> for AncDec { type Output = AncDec; #[inline(always)] fn sub(self, rhs: $t) -> AncDec { self.sub(&AncDec::from(rhs)) } }
        impl Sub<AncDec> for $t { type Output = AncDec; #[inline(always)] fn sub(self, rhs: AncDec) -> AncDec { AncDec::from(self).sub(&rhs) } }
        impl Mul<$t> for AncDec { type Output = AncDec; #[inline(always)] fn mul(self, rhs: $t) -> AncDec { self.mul(&AncDec::from(rhs)) } }
        impl Mul<AncDec> for $t { type Output = AncDec; #[inline(always)] fn mul(self, rhs: AncDec) -> AncDec { AncDec::from(self).mul(&rhs) } }
        impl Div<$t> for AncDec { type Output = AncDec; #[inline(always)] fn div(self, rhs: $t) -> AncDec { self.div(&AncDec::from(rhs)) } }
        impl Div<AncDec> for $t { type Output = AncDec; #[inline(always)] fn div(self, rhs: AncDec) -> AncDec { AncDec::from(self).div(&rhs) } }
    )*};
}
impl_ops_primitive!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
