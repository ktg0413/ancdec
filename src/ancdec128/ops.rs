use super::AncDec128;
use crate::wide::mul_wide;
use crate::util::pow10_128;
use core::hash::{Hash, Hasher};
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

// ============ Operator Traits ============
impl Add for AncDec128 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self.add(&rhs)
    }
}

impl Sub for AncDec128 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        self.sub(&rhs)
    }
}

impl Mul for AncDec128 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        self.mul(&rhs)
    }
}

impl Div for AncDec128 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        self.div(&rhs)
    }
}

impl Rem for AncDec128 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        self.rem(&rhs)
    }
}

impl Neg for AncDec128 {
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
impl AddAssign for AncDec128 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(&rhs);
    }
}

impl SubAssign for AncDec128 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(&rhs);
    }
}

impl MulAssign for AncDec128 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(&rhs);
    }
}

impl DivAssign for AncDec128 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(&rhs);
    }
}

impl RemAssign for AncDec128 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(&rhs);
    }
}

// ============ Reference Ops ============
impl<'b> Add<&'b AncDec128> for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn add(self, rhs: &'b AncDec128) -> AncDec128 {
        self.add(rhs)
    }
}
impl<'a> Add<&'a AncDec128> for AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn add(self, rhs: &'a AncDec128) -> AncDec128 {
        AncDec128::add(&self, rhs)
    }
}
impl Add<AncDec128> for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn add(self, rhs: AncDec128) -> AncDec128 {
        self.add(&rhs)
    }
}

impl<'b> Sub<&'b AncDec128> for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn sub(self, rhs: &'b AncDec128) -> AncDec128 {
        self.sub(rhs)
    }
}
impl<'a> Sub<&'a AncDec128> for AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn sub(self, rhs: &'a AncDec128) -> AncDec128 {
        AncDec128::sub(&self, rhs)
    }
}
impl Sub<AncDec128> for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn sub(self, rhs: AncDec128) -> AncDec128 {
        self.sub(&rhs)
    }
}

impl<'b> Mul<&'b AncDec128> for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn mul(self, rhs: &'b AncDec128) -> AncDec128 {
        self.mul(rhs)
    }
}
impl<'a> Mul<&'a AncDec128> for AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn mul(self, rhs: &'a AncDec128) -> AncDec128 {
        AncDec128::mul(&self, rhs)
    }
}
impl Mul<AncDec128> for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn mul(self, rhs: AncDec128) -> AncDec128 {
        self.mul(&rhs)
    }
}

impl<'b> Div<&'b AncDec128> for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn div(self, rhs: &'b AncDec128) -> AncDec128 {
        self.div(rhs)
    }
}
impl<'a> Div<&'a AncDec128> for AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn div(self, rhs: &'a AncDec128) -> AncDec128 {
        AncDec128::div(&self, rhs)
    }
}
impl Div<AncDec128> for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn div(self, rhs: AncDec128) -> AncDec128 {
        self.div(&rhs)
    }
}

impl<'b> Rem<&'b AncDec128> for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn rem(self, rhs: &'b AncDec128) -> AncDec128 {
        self.rem(rhs)
    }
}
impl<'a> Rem<&'a AncDec128> for AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn rem(self, rhs: &'a AncDec128) -> AncDec128 {
        AncDec128::rem(&self, rhs)
    }
}
impl Rem<AncDec128> for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn rem(self, rhs: AncDec128) -> AncDec128 {
        self.rem(&rhs)
    }
}

impl Neg for &AncDec128 {
    type Output = AncDec128;
    #[inline(always)]
    fn neg(self) -> AncDec128 {
        AncDec128 {
            neg: !self.neg,
            ..*self
        }
    }
}

// ============ Default ============
impl Default for AncDec128 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

// ============ Hash ============
/// Normalizes trailing zeros so 1.0 == 1.00 have same hash
/// Uses u256 combined value via mul_wide
impl Hash for AncDec128 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (mut hi, mut lo) = mul_wide(self.int, pow10_128(self.scale));
        let (lo2, carry) = lo.overflowing_add(self.frac);
        lo = lo2;
        hi += carry as u128;

        // Binary search: strip trailing zeros in O(log scale) instead of O(scale)
        // u256 divisibility by 10: (hi % 10) * 6 + lo % 10) % 10 == 0  (since 2^128 mod 10 = 6)
        if (hi > 0 || lo > 0) && ((hi % 10) * 6 + lo % 10) % 10 == 0 {
            macro_rules! strip_u256 {
                ($pow:expr) => {
                    let ((qh, ql), r) = crate::wide::divmod_u256(hi, lo, $pow);
                    if r == 0 { hi = qh; lo = ql; }
                };
            }
            strip_u256!(pow10_128(32));
            strip_u256!(pow10_128(16));
            strip_u256!(100_000_000u128);
            strip_u256!(10_000u128);
            strip_u256!(100u128);
            strip_u256!(10u128);
        }

        hi.hash(state);
        lo.hash(state);
        if hi != 0 || lo != 0 {
            self.neg.hash(state);
        }
    }
}

// ============ Iterator Traits ============
impl Sum for AncDec128 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, x| a.add(&x))
    }
}
impl<'a> Sum<&'a AncDec128> for AncDec128 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, x| a.add(x))
    }
}

impl Product for AncDec128 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, x| a.mul(&x))
    }
}
impl<'a> Product<&'a AncDec128> for AncDec128 {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, x| a.mul(x))
    }
}

// ============ From Integer ============
macro_rules! impl_from_signed_128 { ($($t:ty),*) => {$( impl From<$t> for AncDec128 { #[inline(always)] fn from(n: $t) -> Self { Self { int: n.unsigned_abs() as u128, frac: 0, scale: 0, neg: n < 0 } } } )*}; }

macro_rules! impl_from_unsigned_128 { ($($t:ty),*) => {$( impl From<$t> for AncDec128 { #[inline(always)] fn from(n: $t) -> Self { Self { int: n as u128, frac: 0, scale: 0, neg: false } } } )*}; }

impl_from_signed_128!(i8, i16, i32, i64, i128, isize);
impl_from_unsigned_128!(u8, u16, u32, u64, u128, usize);

// ============ Ops with Primitives ============
macro_rules! impl_ops_primitive_128 {
    ($($t:ty),*) => {$(
        impl Add<$t> for AncDec128 { type Output = AncDec128; #[inline(always)] fn add(self, rhs: $t) -> AncDec128 { self.add(&AncDec128::from(rhs)) } }
        impl Add<AncDec128> for $t { type Output = AncDec128; #[inline(always)] fn add(self, rhs: AncDec128) -> AncDec128 { AncDec128::from(self).add(&rhs) } }
        impl Sub<$t> for AncDec128 { type Output = AncDec128; #[inline(always)] fn sub(self, rhs: $t) -> AncDec128 { self.sub(&AncDec128::from(rhs)) } }
        impl Sub<AncDec128> for $t { type Output = AncDec128; #[inline(always)] fn sub(self, rhs: AncDec128) -> AncDec128 { AncDec128::from(self).sub(&rhs) } }
        impl Mul<$t> for AncDec128 { type Output = AncDec128; #[inline(always)] fn mul(self, rhs: $t) -> AncDec128 { self.mul(&AncDec128::from(rhs)) } }
        impl Mul<AncDec128> for $t { type Output = AncDec128; #[inline(always)] fn mul(self, rhs: AncDec128) -> AncDec128 { AncDec128::from(self).mul(&rhs) } }
        impl Div<$t> for AncDec128 { type Output = AncDec128; #[inline(always)] fn div(self, rhs: $t) -> AncDec128 { self.div(&AncDec128::from(rhs)) } }
        impl Div<AncDec128> for $t { type Output = AncDec128; #[inline(always)] fn div(self, rhs: AncDec128) -> AncDec128 { AncDec128::from(self).div(&rhs) } }
    )*};
}
impl_ops_primitive_128!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
