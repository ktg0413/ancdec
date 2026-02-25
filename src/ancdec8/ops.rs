use super::AncDec8;
use crate::util::pow10_16;
use core::hash::{Hash, Hasher};
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

// ============ Operator Traits ============
impl Add for AncDec8 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self.add(&rhs)
    }
}

impl Sub for AncDec8 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        self.sub(&rhs)
    }
}

impl Mul for AncDec8 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        self.mul(&rhs)
    }
}

impl Div for AncDec8 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        self.div(&rhs)
    }
}

impl Rem for AncDec8 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        self.rem(&rhs)
    }
}

impl Neg for AncDec8 {
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
impl AddAssign for AncDec8 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(&rhs);
    }
}

impl SubAssign for AncDec8 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(&rhs);
    }
}

impl MulAssign for AncDec8 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(&rhs);
    }
}

impl DivAssign for AncDec8 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(&rhs);
    }
}

impl RemAssign for AncDec8 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(&rhs);
    }
}

// ============ Reference Ops ============
impl<'b> Add<&'b AncDec8> for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn add(self, rhs: &'b AncDec8) -> AncDec8 {
        self.add(rhs)
    }
}
impl<'a> Add<&'a AncDec8> for AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn add(self, rhs: &'a AncDec8) -> AncDec8 {
        AncDec8::add(&self, rhs)
    }
}
impl Add<AncDec8> for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn add(self, rhs: AncDec8) -> AncDec8 {
        self.add(&rhs)
    }
}

impl<'b> Sub<&'b AncDec8> for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn sub(self, rhs: &'b AncDec8) -> AncDec8 {
        self.sub(rhs)
    }
}
impl<'a> Sub<&'a AncDec8> for AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn sub(self, rhs: &'a AncDec8) -> AncDec8 {
        AncDec8::sub(&self, rhs)
    }
}
impl Sub<AncDec8> for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn sub(self, rhs: AncDec8) -> AncDec8 {
        self.sub(&rhs)
    }
}

impl<'b> Mul<&'b AncDec8> for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn mul(self, rhs: &'b AncDec8) -> AncDec8 {
        self.mul(rhs)
    }
}
impl<'a> Mul<&'a AncDec8> for AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn mul(self, rhs: &'a AncDec8) -> AncDec8 {
        AncDec8::mul(&self, rhs)
    }
}
impl Mul<AncDec8> for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn mul(self, rhs: AncDec8) -> AncDec8 {
        self.mul(&rhs)
    }
}

impl<'b> Div<&'b AncDec8> for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn div(self, rhs: &'b AncDec8) -> AncDec8 {
        self.div(rhs)
    }
}
impl<'a> Div<&'a AncDec8> for AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn div(self, rhs: &'a AncDec8) -> AncDec8 {
        AncDec8::div(&self, rhs)
    }
}
impl Div<AncDec8> for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn div(self, rhs: AncDec8) -> AncDec8 {
        self.div(&rhs)
    }
}

impl<'b> Rem<&'b AncDec8> for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn rem(self, rhs: &'b AncDec8) -> AncDec8 {
        self.rem(rhs)
    }
}
impl<'a> Rem<&'a AncDec8> for AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn rem(self, rhs: &'a AncDec8) -> AncDec8 {
        AncDec8::rem(&self, rhs)
    }
}
impl Rem<AncDec8> for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn rem(self, rhs: AncDec8) -> AncDec8 {
        self.rem(&rhs)
    }
}

impl Neg for &AncDec8 {
    type Output = AncDec8;
    #[inline(always)]
    fn neg(self) -> AncDec8 {
        AncDec8 {
            neg: !self.neg,
            ..*self
        }
    }
}

// ============ Default ============
impl Default for AncDec8 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

// ============ Hash ============
/// Normalizes trailing zeros so 1.0 == 1.00 have same hash
/// Uses u16 combined value
impl Hash for AncDec8 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut combined = (self.int as u16) * pow10_16(self.scale) + (self.frac as u16);
        // Unrolled: max 2 trailing zeros for scale <= 2
        if combined > 0 {
            if combined % 100 == 0 { combined /= 100; }
            else if combined % 10 == 0 { combined /= 10; }
        }
        combined.hash(state);
        if combined != 0 {
            self.neg.hash(state);
        } // 0 == -0
    }
}

// ============ Iterator Traits ============
impl Sum for AncDec8 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, x| a.add(&x))
    }
}
impl<'a> Sum<&'a AncDec8> for AncDec8 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, x| a.add(x))
    }
}

impl Product for AncDec8 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, x| a.mul(&x))
    }
}
impl<'a> Product<&'a AncDec8> for AncDec8 {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, x| a.mul(x))
    }
}

// ============ From Integer ============
impl From<i8> for AncDec8 {
    #[inline(always)]
    fn from(n: i8) -> Self {
        Self {
            int: n.unsigned_abs(),
            frac: 0,
            scale: 0,
            neg: n < 0,
        }
    }
}

impl From<u8> for AncDec8 {
    #[inline(always)]
    fn from(n: u8) -> Self {
        Self {
            int: n,
            frac: 0,
            scale: 0,
            neg: false,
        }
    }
}

// ============ Ops with Primitives ============
impl Add<i8> for AncDec8 { type Output = AncDec8; #[inline(always)] fn add(self, rhs: i8) -> AncDec8 { self.add(&AncDec8::from(rhs)) } }
impl Add<AncDec8> for i8 { type Output = AncDec8; #[inline(always)] fn add(self, rhs: AncDec8) -> AncDec8 { AncDec8::from(self).add(&rhs) } }
impl Sub<i8> for AncDec8 { type Output = AncDec8; #[inline(always)] fn sub(self, rhs: i8) -> AncDec8 { self.sub(&AncDec8::from(rhs)) } }
impl Sub<AncDec8> for i8 { type Output = AncDec8; #[inline(always)] fn sub(self, rhs: AncDec8) -> AncDec8 { AncDec8::from(self).sub(&rhs) } }
impl Mul<i8> for AncDec8 { type Output = AncDec8; #[inline(always)] fn mul(self, rhs: i8) -> AncDec8 { self.mul(&AncDec8::from(rhs)) } }
impl Mul<AncDec8> for i8 { type Output = AncDec8; #[inline(always)] fn mul(self, rhs: AncDec8) -> AncDec8 { AncDec8::from(self).mul(&rhs) } }
impl Div<i8> for AncDec8 { type Output = AncDec8; #[inline(always)] fn div(self, rhs: i8) -> AncDec8 { self.div(&AncDec8::from(rhs)) } }
impl Div<AncDec8> for i8 { type Output = AncDec8; #[inline(always)] fn div(self, rhs: AncDec8) -> AncDec8 { AncDec8::from(self).div(&rhs) } }

impl Add<u8> for AncDec8 { type Output = AncDec8; #[inline(always)] fn add(self, rhs: u8) -> AncDec8 { self.add(&AncDec8::from(rhs)) } }
impl Add<AncDec8> for u8 { type Output = AncDec8; #[inline(always)] fn add(self, rhs: AncDec8) -> AncDec8 { AncDec8::from(self).add(&rhs) } }
impl Sub<u8> for AncDec8 { type Output = AncDec8; #[inline(always)] fn sub(self, rhs: u8) -> AncDec8 { self.sub(&AncDec8::from(rhs)) } }
impl Sub<AncDec8> for u8 { type Output = AncDec8; #[inline(always)] fn sub(self, rhs: AncDec8) -> AncDec8 { AncDec8::from(self).sub(&rhs) } }
impl Mul<u8> for AncDec8 { type Output = AncDec8; #[inline(always)] fn mul(self, rhs: u8) -> AncDec8 { self.mul(&AncDec8::from(rhs)) } }
impl Mul<AncDec8> for u8 { type Output = AncDec8; #[inline(always)] fn mul(self, rhs: AncDec8) -> AncDec8 { AncDec8::from(self).mul(&rhs) } }
impl Div<u8> for AncDec8 { type Output = AncDec8; #[inline(always)] fn div(self, rhs: u8) -> AncDec8 { self.div(&AncDec8::from(rhs)) } }
impl Div<AncDec8> for u8 { type Output = AncDec8; #[inline(always)] fn div(self, rhs: AncDec8) -> AncDec8 { AncDec8::from(self).div(&rhs) } }
