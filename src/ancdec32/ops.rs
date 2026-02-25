use super::AncDec32;
use crate::util::pow10;
use core::hash::{Hash, Hasher};
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

// ============ Operator Traits ============
impl Add for AncDec32 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self.add(&rhs)
    }
}

impl Sub for AncDec32 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        self.sub(&rhs)
    }
}

impl Mul for AncDec32 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        self.mul(&rhs)
    }
}

impl Div for AncDec32 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        self.div(&rhs)
    }
}

impl Rem for AncDec32 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        self.rem(&rhs)
    }
}

impl Neg for AncDec32 {
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
impl AddAssign for AncDec32 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(&rhs);
    }
}

impl SubAssign for AncDec32 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(&rhs);
    }
}

impl MulAssign for AncDec32 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(&rhs);
    }
}

impl DivAssign for AncDec32 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(&rhs);
    }
}

impl RemAssign for AncDec32 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(&rhs);
    }
}

// ============ Reference Ops ============
impl<'b> Add<&'b AncDec32> for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn add(self, rhs: &'b AncDec32) -> AncDec32 {
        self.add(rhs)
    }
}
impl<'a> Add<&'a AncDec32> for AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn add(self, rhs: &'a AncDec32) -> AncDec32 {
        AncDec32::add(&self, rhs)
    }
}
impl Add<AncDec32> for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn add(self, rhs: AncDec32) -> AncDec32 {
        self.add(&rhs)
    }
}

impl<'b> Sub<&'b AncDec32> for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn sub(self, rhs: &'b AncDec32) -> AncDec32 {
        self.sub(rhs)
    }
}
impl<'a> Sub<&'a AncDec32> for AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn sub(self, rhs: &'a AncDec32) -> AncDec32 {
        AncDec32::sub(&self, rhs)
    }
}
impl Sub<AncDec32> for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn sub(self, rhs: AncDec32) -> AncDec32 {
        self.sub(&rhs)
    }
}

impl<'b> Mul<&'b AncDec32> for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn mul(self, rhs: &'b AncDec32) -> AncDec32 {
        self.mul(rhs)
    }
}
impl<'a> Mul<&'a AncDec32> for AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn mul(self, rhs: &'a AncDec32) -> AncDec32 {
        AncDec32::mul(&self, rhs)
    }
}
impl Mul<AncDec32> for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn mul(self, rhs: AncDec32) -> AncDec32 {
        self.mul(&rhs)
    }
}

impl<'b> Div<&'b AncDec32> for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn div(self, rhs: &'b AncDec32) -> AncDec32 {
        self.div(rhs)
    }
}
impl<'a> Div<&'a AncDec32> for AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn div(self, rhs: &'a AncDec32) -> AncDec32 {
        AncDec32::div(&self, rhs)
    }
}
impl Div<AncDec32> for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn div(self, rhs: AncDec32) -> AncDec32 {
        self.div(&rhs)
    }
}

impl<'b> Rem<&'b AncDec32> for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn rem(self, rhs: &'b AncDec32) -> AncDec32 {
        self.rem(rhs)
    }
}
impl<'a> Rem<&'a AncDec32> for AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn rem(self, rhs: &'a AncDec32) -> AncDec32 {
        AncDec32::rem(&self, rhs)
    }
}
impl Rem<AncDec32> for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn rem(self, rhs: AncDec32) -> AncDec32 {
        self.rem(&rhs)
    }
}

impl Neg for &AncDec32 {
    type Output = AncDec32;
    #[inline(always)]
    fn neg(self) -> AncDec32 {
        AncDec32 {
            neg: !self.neg,
            ..*self
        }
    }
}

// ============ Default ============
impl Default for AncDec32 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

// ============ Hash ============
/// Normalizes trailing zeros so 1.0 == 1.00 have same hash
impl Hash for AncDec32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut combined = self.int as u64 * pow10(self.scale) + self.frac as u64;
        // Binary search: strip trailing zeros in O(log scale) instead of O(scale)
        if combined > 0 {
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
impl Sum for AncDec32 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, x| a.add(&x))
    }
}
impl<'a> Sum<&'a AncDec32> for AncDec32 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, x| a.add(x))
    }
}

impl Product for AncDec32 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, x| a.mul(&x))
    }
}
impl<'a> Product<&'a AncDec32> for AncDec32 {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, x| a.mul(x))
    }
}

// ============ From Integer ============
macro_rules! impl_from_signed_32 { ($($t:ty),*) => {$( impl From<$t> for AncDec32 { #[inline(always)] fn from(n: $t) -> Self { Self { int: n.unsigned_abs() as u32, frac: 0, scale: 0, neg: n < 0 } } } )*}; }

macro_rules! impl_from_unsigned_32 { ($($t:ty),*) => {$( impl From<$t> for AncDec32 { #[inline(always)] fn from(n: $t) -> Self { Self { int: n as u32, frac: 0, scale: 0, neg: false } } } )*}; }

impl_from_signed_32!(i8, i16, i32);
impl_from_unsigned_32!(u8, u16, u32);

// ============ Ops with Primitives ============
macro_rules! impl_ops_primitive_32 {
    ($($t:ty),*) => {$(
        impl Add<$t> for AncDec32 { type Output = AncDec32; #[inline(always)] fn add(self, rhs: $t) -> AncDec32 { self.add(&AncDec32::from(rhs)) } }
        impl Add<AncDec32> for $t { type Output = AncDec32; #[inline(always)] fn add(self, rhs: AncDec32) -> AncDec32 { AncDec32::from(self).add(&rhs) } }
        impl Sub<$t> for AncDec32 { type Output = AncDec32; #[inline(always)] fn sub(self, rhs: $t) -> AncDec32 { self.sub(&AncDec32::from(rhs)) } }
        impl Sub<AncDec32> for $t { type Output = AncDec32; #[inline(always)] fn sub(self, rhs: AncDec32) -> AncDec32 { AncDec32::from(self).sub(&rhs) } }
        impl Mul<$t> for AncDec32 { type Output = AncDec32; #[inline(always)] fn mul(self, rhs: $t) -> AncDec32 { self.mul(&AncDec32::from(rhs)) } }
        impl Mul<AncDec32> for $t { type Output = AncDec32; #[inline(always)] fn mul(self, rhs: AncDec32) -> AncDec32 { AncDec32::from(self).mul(&rhs) } }
        impl Div<$t> for AncDec32 { type Output = AncDec32; #[inline(always)] fn div(self, rhs: $t) -> AncDec32 { self.div(&AncDec32::from(rhs)) } }
        impl Div<AncDec32> for $t { type Output = AncDec32; #[inline(always)] fn div(self, rhs: AncDec32) -> AncDec32 { AncDec32::from(self).div(&rhs) } }
    )*};
}
impl_ops_primitive_32!(i8, i16, i32, u8, u16, u32);
