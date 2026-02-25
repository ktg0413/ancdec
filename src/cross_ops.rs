#[allow(unused_imports)]
use core::ops::{Add, Sub, Mul, Div, Rem};

/// Generate cross-type operator impls: Small op Large -> Large (both directions)
#[allow(unused_macros)]
macro_rules! impl_cross_ops {
    ($Small:ty, $Large:ty, $feat_small:literal, $feat_large:literal) => {
        #[cfg(all(feature = $feat_small, feature = $feat_large))]
        impl Add<$Large> for $Small {
            type Output = $Large;
            #[inline(always)]
            fn add(self, rhs: $Large) -> $Large {
                <$Large>::from(self).add(&rhs)
            }
        }
        #[cfg(all(feature = $feat_small, feature = $feat_large))]
        impl Add<$Small> for $Large {
            type Output = $Large;
            #[inline(always)]
            fn add(self, rhs: $Small) -> $Large {
                self.add(&<$Large>::from(rhs))
            }
        }

        #[cfg(all(feature = $feat_small, feature = $feat_large))]
        impl Sub<$Large> for $Small {
            type Output = $Large;
            #[inline(always)]
            fn sub(self, rhs: $Large) -> $Large {
                <$Large>::from(self).sub(&rhs)
            }
        }
        #[cfg(all(feature = $feat_small, feature = $feat_large))]
        impl Sub<$Small> for $Large {
            type Output = $Large;
            #[inline(always)]
            fn sub(self, rhs: $Small) -> $Large {
                self.sub(&<$Large>::from(rhs))
            }
        }

        #[cfg(all(feature = $feat_small, feature = $feat_large))]
        impl Mul<$Large> for $Small {
            type Output = $Large;
            #[inline(always)]
            fn mul(self, rhs: $Large) -> $Large {
                <$Large>::from(self).mul(&rhs)
            }
        }
        #[cfg(all(feature = $feat_small, feature = $feat_large))]
        impl Mul<$Small> for $Large {
            type Output = $Large;
            #[inline(always)]
            fn mul(self, rhs: $Small) -> $Large {
                self.mul(&<$Large>::from(rhs))
            }
        }

        #[cfg(all(feature = $feat_small, feature = $feat_large))]
        impl Div<$Large> for $Small {
            type Output = $Large;
            #[inline(always)]
            fn div(self, rhs: $Large) -> $Large {
                <$Large>::from(self).div(&rhs)
            }
        }
        #[cfg(all(feature = $feat_small, feature = $feat_large))]
        impl Div<$Small> for $Large {
            type Output = $Large;
            #[inline(always)]
            fn div(self, rhs: $Small) -> $Large {
                self.div(&<$Large>::from(rhs))
            }
        }

        #[cfg(all(feature = $feat_small, feature = $feat_large))]
        impl Rem<$Large> for $Small {
            type Output = $Large;
            #[inline(always)]
            fn rem(self, rhs: $Large) -> $Large {
                <$Large>::from(self).rem(&rhs)
            }
        }
        #[cfg(all(feature = $feat_small, feature = $feat_large))]
        impl Rem<$Small> for $Large {
            type Output = $Large;
            #[inline(always)]
            fn rem(self, rhs: $Small) -> $Large {
                self.rem(&<$Large>::from(rhs))
            }
        }
    };
}

#[cfg(all(feature = "dec8", feature = "dec32"))]
impl_cross_ops!(crate::ancdec8::AncDec8, crate::ancdec32::AncDec32, "dec8", "dec32");

#[cfg(all(feature = "dec8", feature = "dec64"))]
impl_cross_ops!(crate::ancdec8::AncDec8, crate::ancdec::AncDec, "dec8", "dec64");

#[cfg(all(feature = "dec8", feature = "dec128"))]
impl_cross_ops!(crate::ancdec8::AncDec8, crate::ancdec128::AncDec128, "dec8", "dec128");

#[cfg(all(feature = "dec32", feature = "dec64"))]
impl_cross_ops!(crate::ancdec32::AncDec32, crate::ancdec::AncDec, "dec32", "dec64");

#[cfg(all(feature = "dec32", feature = "dec128"))]
impl_cross_ops!(crate::ancdec32::AncDec32, crate::ancdec128::AncDec128, "dec32", "dec128");

#[cfg(all(feature = "dec64", feature = "dec128"))]
impl_cross_ops!(crate::ancdec::AncDec, crate::ancdec128::AncDec128, "dec64", "dec128");
