use super::AncDec8;
use crate::util::pow10_u8;
use core::cmp::Ordering;

/// Compare absolute values
#[inline(always)]
pub(crate) fn cmp_abs_8(a: &AncDec8, b: &AncDec8) -> Ordering {
    if a.int != b.int {
        return a.int.cmp(&b.int);
    }

    let (a_frac, b_frac) = if a.scale == b.scale {
        (a.frac, b.frac)
    } else if a.scale > b.scale {
        // SAFETY: frac < 10^scale by invariant, product < 10^2 <= u8::MAX
        (a.frac, b.frac * pow10_u8(a.scale - b.scale))
    } else {
        (a.frac * pow10_u8(b.scale - a.scale), b.frac)
    };
    a_frac.cmp(&b_frac)
}

/// Ord trait
impl Ord for AncDec8 {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        // 0 == -0
        if self.is_zero() && other.is_zero() {
            return Ordering::Equal;
        }

        match (self.neg, other.neg) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => cmp_abs_8(self, other),
            (true, true) => cmp_abs_8(self, other).reverse(),
        }
    }
}

impl PartialOrd for AncDec8 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AncDec8 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for AncDec8 {}
