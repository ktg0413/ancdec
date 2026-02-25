use super::AncDec128;
use crate::util::pow10_128;
use core::cmp::Ordering;

/// Compare absolute values
#[inline(always)]
pub(crate) fn cmp_abs_128(a: &AncDec128, b: &AncDec128) -> Ordering {
    if a.int != b.int {
        return a.int.cmp(&b.int);
    }

    let (a_frac, b_frac) = if a.scale == b.scale {
        (a.frac, b.frac)
    } else if a.scale > b.scale {
        // SAFETY: frac < 10^scale by invariant, product < 10^38 < u128::MAX
        (a.frac, b.frac * pow10_128(a.scale - b.scale))
    } else {
        (a.frac * pow10_128(b.scale - a.scale), b.frac)
    };
    a_frac.cmp(&b_frac)
}

/// Ord trait
impl Ord for AncDec128 {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        // 0 == -0
        if self.is_zero() && other.is_zero() {
            return Ordering::Equal;
        }

        match (self.neg, other.neg) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => cmp_abs_128(self, other),
            (true, true) => cmp_abs_128(self, other).reverse(),
        }
    }
}

impl PartialOrd for AncDec128 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AncDec128 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for AncDec128 {}
