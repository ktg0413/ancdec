use crate::util::pow10;
use super::AncDec;
use core::cmp::Ordering;

/// Compare absolute values
#[inline(always)]
pub(crate) fn cmp_abs(a: &AncDec, b: &AncDec) -> Ordering {
    if a.int != b.int {
        return a.int.cmp(&b.int);
    }

    let (a_frac, b_frac) = if a.scale == b.scale {
        (a.frac, b.frac)
    } else if a.scale > b.scale {
        // SAFETY: frac < 10^scale by invariant, product < 10^19 < u64::MAX
        (a.frac, b.frac * pow10(a.scale - b.scale))
    } else {
        (a.frac * pow10(b.scale - a.scale), b.frac)
    };
    a_frac.cmp(&b_frac)
}

/// Ord trait: enables `<`, `>`, `sort()`, `min()`, `max()`
impl Ord for AncDec {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        // 0 == -0
        if self.is_zero() && other.is_zero() {
            return Ordering::Equal;
        }

        match (self.neg, other.neg) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => cmp_abs(self, other),
            (true, true) => cmp_abs(self, other).reverse(),
        }
    }
}

/// PartialOrd trait: enables `<`, `>` comparisons
impl PartialOrd for AncDec {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// PartialEq trait: enables `==`, `!=`
impl PartialEq for AncDec {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

/// Eq trait: marker for total equality (no NaN)
impl Eq for AncDec {}
