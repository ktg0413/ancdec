// Wide arithmetic helpers (internal use only)

// ============ u256 Arithmetic (dec64 + dec128) ============

/// u128 * u128 -> (high, low)
#[cfg(any(feature = "dec64", feature = "dec128"))]
#[inline]
pub(crate) fn mul_wide(a: u128, b: u128) -> (u128, u128) {
    let a_lo = a as u64 as u128;
    let a_hi = a >> 64;
    let b_lo = b as u64 as u128;
    let b_hi = b >> 64;

    let ll = a_lo * b_lo;
    let hl = a_hi * b_lo;
    let lh = a_lo * b_hi;
    let hh = a_hi * b_hi;

    let mid = hl.wrapping_add(lh);
    let mid_carry = (mid < hl) as u128;

    let (low, carry) = ll.overflowing_add(mid << 64);
    let high = hh + (mid >> 64) + (mid_carry << 64) + carry as u128;

    (high, low)
}

/// Shift u256 left by `shift` bits (shift < 128)
#[cfg(any(feature = "dec64", feature = "dec128"))]
#[inline]
fn shl_u256(high: u128, low: u128, shift: u32) -> (u64, u64, u64, u64) {
    assert!(shift < 128, "shl_u256: shift must be < 128");
    if shift == 0 {
        return (
            (high >> 64) as u64,
            high as u64,
            (low >> 64) as u64,
            low as u64,
        );
    }

    let high_shifted = (high << shift) | (low >> (128 - shift));
    let low_shifted = low << shift;

    (
        (high_shifted >> 64) as u64,
        high_shifted as u64,
        (low_shifted >> 64) as u64,
        low_shifted as u64,
    )
}

/// u128 / u64 -> (quotient, remainder)
#[cfg(any(feature = "dec64", feature = "dec128"))]
#[inline]
fn div_u128_by_u64(n: u128, d: u64) -> (u128, u128) {
    (n / (d as u128), n % (d as u128))
}

/// Divide u256 by u64 -> u128 quotient
#[cfg(any(feature = "dec64", feature = "dec128"))]
#[inline]
fn div_wide_by_u64(high: u128, low: u128, divisor: u64) -> u128 {
    let n3 = (high >> 64) as u64;
    let n2 = high as u64;
    let n1 = (low >> 64) as u64;
    let n0 = low as u64;

    let (q3, r3) = div_u128_by_u64(n3 as u128, divisor);
    let (q2, r2) = div_u128_by_u64((r3 << 64) | (n2 as u128), divisor);
    let (q1, r1) = div_u128_by_u64((r2 << 64) | (n1 as u128), divisor);
    let (q0, _) = div_u128_by_u64((r1 << 64) | (n0 as u128), divisor);

    assert!(q3 == 0 && q2 == 0);
    (q1 << 64) | q0
}

/// Core: divide (n2, n1, n0) by (d1, d0) where values are normalized
#[cfg(any(feature = "dec64", feature = "dec128"))]
#[inline]
fn div_3by2(n2: u64, n1: u64, n0: u64, d1: u64, d0: u64) -> (u64, u128) {
    let n_hi = ((n2 as u128) << 64) | (n1 as u128);
    let mut q_hat = if n2 >= d1 {
        u64::MAX
    } else {
        (n_hi / (d1 as u128)) as u64
    };
    let mut r_hat = n_hi - (q_hat as u128) * (d1 as u128);

    // Unrolled refinement: Knuth guarantees at most 2 iterations with normalized divisor
    if r_hat <= u64::MAX as u128 {
        let check = (q_hat as u128) * (d0 as u128);
        let right = (r_hat << 64) | (n0 as u128);
        if check > right {
            q_hat -= 1;
            r_hat += d1 as u128;
            if r_hat <= u64::MAX as u128 {
                let check = (q_hat as u128) * (d0 as u128);
                let right = (r_hat << 64) | (n0 as u128);
                if check > right {
                    q_hat -= 1;
                }
            }
        }
    }

    let product = (q_hat as u128) * (d0 as u128);
    let product_hi = (q_hat as u128) * (d1 as u128) + (product >> 64);

    let (sub_lo, borrow1) = (n0 as u128).overflowing_sub(product & ((1u128 << 64) - 1));
    let (sub_mid, borrow2) =
        (n1 as u128).overflowing_sub((product_hi & ((1u128 << 64) - 1)) + borrow1 as u128);
    let sub_hi = (n2 as u128).wrapping_sub((product_hi >> 64) + borrow2 as u128);

    let (rem_hi, q_final) = if sub_hi > n2 as u128 {
        let add_lo = sub_lo.wrapping_add(d0 as u128);
        let carry = (add_lo < sub_lo) as u128;
        let add_mid = sub_mid.wrapping_add((d1 as u128) + carry);
        ((add_mid << 64) | (add_lo & ((1u128 << 64) - 1)), q_hat - 1)
    } else {
        (
            ((sub_mid & ((1u128 << 64) - 1)) << 64) | (sub_lo & ((1u128 << 64) - 1)),
            q_hat,
        )
    };

    (q_final, rem_hi)
}

/// Full 4-by-2 division: (n3,n2,n1,n0) / (d1,d0) -> (q1, q0)
#[cfg(any(feature = "dec64", feature = "dec128"))]
#[inline]
fn div_4by2(n3: u64, n2: u64, n1: u64, n0: u64, d1: u64, d0: u64) -> (u64, u64) {
    let (q1, rem1) = div_3by2(n3, n2, n1, d1, d0);
    let r1_hi = (rem1 >> 64) as u64;
    let r1_lo = rem1 as u64;
    let (q0, _) = div_3by2(r1_hi, r1_lo, n0, d1, d0);
    (q1, q0)
}

/// Knuth Algorithm D: (u256) / (u128) -> u128
#[cfg(any(feature = "dec64", feature = "dec128"))]
#[inline]
pub(crate) fn div_wide(high: u128, low: u128, divisor: u128) -> u128 {
    assert!(divisor != 0, "division by zero");

    if high == 0 {
        return low / divisor;
    }

    assert!(high < divisor, "quotient overflow");

    let d_hi = (divisor >> 64) as u64;

    if d_hi == 0 {
        return div_wide_by_u64(high, low, divisor as u64);
    }

    let shift = divisor.leading_zeros();
    let divisor_norm = divisor << shift;
    let d1 = (divisor_norm >> 64) as u64;
    let d0 = divisor_norm as u64;

    let (n3, n2, n1, n0) = shl_u256(high, low, shift);
    let (q1, q0) = div_4by2(n3, n2, n1, n0, d1, d0);

    ((q1 as u128) << 64) | (q0 as u128)
}

// ============ u256/u512 Arithmetic for AncDec128 ============

/// u256 * u256 -> u512 (represented as 4 u128 limbs: w3..w0, highest first)
#[cfg(feature = "dec128")]
#[inline]
pub(crate) fn mul_u256(a: (u128, u128), b: (u128, u128)) -> (u128, u128, u128, u128) {
    let (a_hi, a_lo) = a;
    let (b_hi, b_lo) = b;

    // Schoolbook: 4 partial products of u128*u128->u256
    let (ll_h, ll_l) = mul_wide(a_lo, b_lo);
    let (hl_h, hl_l) = mul_wide(a_hi, b_lo);
    let (lh_h, lh_l) = mul_wide(a_lo, b_hi);
    let (hh_h, hh_l) = mul_wide(a_hi, b_hi);

    // w0 = ll_l
    let w0 = ll_l;

    // w1 = ll_h + hl_l + lh_l (with carries into w2)
    let (w1, c1) = ll_h.overflowing_add(hl_l);
    let (w1, c2) = w1.overflowing_add(lh_l);
    let carry1 = c1 as u128 + c2 as u128;

    // w2 = hl_h + lh_h + hh_l + carry1
    let (w2, c3) = hl_h.overflowing_add(lh_h);
    let (w2, c4) = w2.overflowing_add(hh_l);
    let (w2, c5) = w2.overflowing_add(carry1);
    let carry2 = c3 as u128 + c4 as u128 + c5 as u128;

    // w3 = hh_h + carry2
    let w3 = hh_h + carry2;

    (w3, w2, w1, w0)
}

/// u512 / u128 -> u256 quotient
/// Used for mul reduction where divisor is pow10(scale) which fits in u128
#[cfg(feature = "dec128")]
#[inline]
pub(crate) fn div_u512_by_u128(
    w3: u128,
    w2: u128,
    w1: u128,
    w0: u128,
    divisor: u128,
) -> (u128, u128) {
    assert!(divisor != 0, "division by zero");

    // Long division: process two u128 digits at a time using div_wide
    // Each step: (remainder, next_digit) / divisor -> (quotient_digit, remainder)
    let (q3, r3) = if w3 >= divisor {
        (w3 / divisor, w3 % divisor)
    } else {
        (0, w3)
    };
    let q2 = div_wide(r3, w2, divisor);
    let r2_full = {
        let (ph, pl) = mul_wide(q2, divisor);
        // (r3, w2) - q2 * divisor
        let (sub_lo, borrow) = w2.overflowing_sub(pl);
        let sub_hi = r3.wrapping_sub(ph).wrapping_sub(borrow as u128);
        assert!(sub_hi == 0);
        sub_lo
    };
    let q1 = div_wide(r2_full, w1, divisor);
    let r1_full = {
        let (ph, pl) = mul_wide(q1, divisor);
        let (sub_lo, borrow) = w1.overflowing_sub(pl);
        let sub_hi = r2_full.wrapping_sub(ph).wrapping_sub(borrow as u128);
        assert!(sub_hi == 0);
        sub_lo
    };
    let q0 = div_wide(r1_full, w0, divisor);

    if q3 != 0 || q2 != 0 {
        return (u128::MAX, u128::MAX); // saturate: quotient overflows u256
    }
    (q1, q0)
}

/// u256 divmod u128 -> (quotient_u256, remainder_u128)
/// Used by from_combined to split u256 by pow10_128(scale)
#[cfg(feature = "dec128")]
#[inline]
pub(crate) fn divmod_u256(
    hi: u128,
    lo: u128,
    divisor: u128,
) -> ((u128, u128), u128) {
    assert!(divisor != 0, "division by zero");

    if hi == 0 {
        return ((0, lo / divisor), lo % divisor);
    }

    // hi < divisor: quotient fits in u128
    if hi < divisor {
        let q = div_wide(hi, lo, divisor);
        let (ph, pl) = mul_wide(q, divisor);
        let (rem_lo, borrow) = lo.overflowing_sub(pl);
        let rem_hi = hi.wrapping_sub(ph).wrapping_sub(borrow as u128);
        assert!(rem_hi == 0);
        return ((0, q), rem_lo);
    }

    // hi >= divisor: quotient is u256
    let q_hi = hi / divisor;
    let r_hi = hi % divisor;
    let q_lo = div_wide(r_hi, lo, divisor);
    let (ph, pl) = mul_wide(q_lo, divisor);
    let (rem_lo, borrow) = lo.overflowing_sub(pl);
    let rem_hi = r_hi.wrapping_sub(ph).wrapping_sub(borrow as u128);
    assert!(rem_hi == 0);
    ((q_hi, q_lo), rem_lo)
}

/// Shift u512 left by `shift` bits (shift < 128)
/// Input/output: (w3, w2, w1, w0) with w3 being the most significant
#[cfg(feature = "dec128")]
#[inline]
fn shl_u512(
    w3: u128,
    w2: u128,
    w1: u128,
    w0: u128,
    shift: u32,
) -> (u128, u128, u128, u128) {
    assert!(shift < 128, "shl_u512: shift must be < 128");
    if shift == 0 {
        return (w3, w2, w1, w0);
    }
    let s3 = (w3 << shift) | (w2 >> (128 - shift));
    let s2 = (w2 << shift) | (w1 >> (128 - shift));
    let s1 = (w1 << shift) | (w0 >> (128 - shift));
    let s0 = w0 << shift;
    (s3, s2, s1, s0)
}

/// Core: divide 3 u128-digits by 2 u128-digits (normalized)
/// (n2, n1, n0) / (d1, d0) -> (quotient_u128, remainder_u256)
#[cfg(feature = "dec128")]
#[inline]
fn div_3by2_128(n2: u128, n1: u128, n0: u128, d1: u128, d0: u128) -> (u128, (u128, u128)) {
    // Estimate q_hat = (n2, n1) / d1
    let mut q_hat = if n2 >= d1 {
        u128::MAX
    } else {
        div_wide(n2, n1, d1)
    };

    // r_hat = (n2, n1) - q_hat * d1
    let (ph, pl) = mul_wide(q_hat, d1);
    let (mut r_lo, borrow) = n1.overflowing_sub(pl);
    let mut r_hi = n2.wrapping_sub(ph).wrapping_sub(borrow as u128);

    // Unrolled refinement: Knuth guarantees at most 2 iterations with normalized divisor
    if r_hi == 0 {
        let (ch, cl) = mul_wide(q_hat, d0);
        if ch > r_lo || (ch == r_lo && cl > n0) {
            q_hat -= 1;
            let (new_r, carry) = r_lo.overflowing_add(d1);
            r_lo = new_r;
            r_hi += carry as u128;
            // Second refinement (only if r still fits)
            if r_hi == 0 {
                let (ch2, cl2) = mul_wide(q_hat, d0);
                if ch2 > r_lo || (ch2 == r_lo && cl2 > n0) {
                    q_hat -= 1;
                }
            }
        }
    }

    // Compute n - q_hat * d
    let (qd1_h, qd1_l) = mul_wide(q_hat, d1);
    let (qd0_h, qd0_l) = mul_wide(q_hat, d0);

    // q_hat * d as a 3-limb value: (qd1_h, qd1_l + qd0_h, qd0_l)
    let (mid, mid_carry) = qd1_l.overflowing_add(qd0_h);
    let top = qd1_h + mid_carry as u128;

    // Subtract: (n2, n1, n0) - (top, mid, qd0_l)
    let (s0, b0) = n0.overflowing_sub(qd0_l);
    let (s1, b1) = n1.overflowing_sub(mid);
    let (s1, b1b) = s1.overflowing_sub(b0 as u128);
    let s2 = n2.wrapping_sub(top).wrapping_sub(b1 as u128).wrapping_sub(b1b as u128);

    // If underflow (s2 wrapped), add back divisor
    if s2 > n2 {
        let (a0, c0) = s0.overflowing_add(d0);
        let (a1, c1) = s1.overflowing_add(d1);
        let a1 = a1.wrapping_add(c0 as u128);
        let _ = c1; // carry absorbed
        (q_hat - 1, (a1, a0))
    } else {
        (q_hat, (s1, s0))
    }
}

/// u512 / u256 -> u256 quotient
/// Used for division: (combined_a * scale_factor) / combined_b
#[cfg(feature = "dec128")]
#[inline]
pub(crate) fn div_u512_by_u256(
    w3: u128,
    w2: u128,
    w1: u128,
    w0: u128,
    d_hi: u128,
    d_lo: u128,
) -> (u128, u128) {
    assert!(d_hi != 0 || d_lo != 0, "division by zero");

    // If divisor fits in u128 (d_hi == 0), use simpler path
    if d_hi == 0 {
        return div_u512_by_u128(w3, w2, w1, w0, d_lo);
    }

    // Normalize: shift divisor so d1 has its MSB set
    let shift = d_hi.leading_zeros();
    let (d1, d0) = if shift == 0 {
        (d_hi, d_lo)
    } else {
        ((d_hi << shift) | (d_lo >> (128 - shift)), d_lo << shift)
    };

    let (n3, n2, n1, n0) = shl_u512(w3, w2, w1, w0, shift);

    // 4-by-2 division using 3-by-2 as building block
    let (q1, rem1) = div_3by2_128(n3, n2, n1, d1, d0);
    let (r1_hi, r1_lo) = rem1;
    let (q0, _) = div_3by2_128(r1_hi, r1_lo, n0, d1, d0);

    (q1, q0)
}

// ============ Integer Square Root ============

/// Integer square root of u128
/// Returns floor(sqrt(n))
#[inline]
pub(crate) fn isqrt_u128(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }

    let bits = 128 - n.leading_zeros();
    let mut x = 1u128 << bits.div_ceil(2);

    loop {
        let q = n / x;
        let x_new = (x >> 1) + (q >> 1) + (x & q & 1);
        if x_new >= x {
            break;
        }
        x = x_new;
    }

    if x.checked_mul(x).is_none_or(|sq| sq > n) {
        x - 1
    } else {
        x
    }
}

/// Integer square root of u256 (n_hi, n_lo) → u128
/// Returns floor(sqrt(N)) where N = n_hi * 2^128 + n_lo
#[cfg(any(feature = "dec64", feature = "dec128"))]
#[inline]
pub(crate) fn isqrt_u256(n_hi: u128, n_lo: u128) -> u128 {
    if n_hi == 0 {
        return isqrt_u128(n_lo);
    }

    let total_bits = 256 - n_hi.leading_zeros();
    let shift = total_bits.div_ceil(2);
    let mut x: u128 = if shift >= 128 { u128::MAX } else { 1u128 << shift };

    loop {
        let n_div_x = if n_hi >= x {
            u128::MAX
        } else {
            div_wide(n_hi, n_lo, x)
        };
        let x_new = (x >> 1) + (n_div_x >> 1) + (x & n_div_x & 1);
        if x_new >= x {
            break;
        }
        x = x_new;
    }

    let (sq_hi, sq_lo) = mul_wide(x, x);
    if (sq_hi, sq_lo) > (n_hi, n_lo) {
        x -= 1;
    }

    x
}

/// Integer square root of u512 (w3, w2, w1, w0) → u256
/// Returns floor(sqrt(N))
#[cfg(feature = "dec128")]
#[inline]
pub(crate) fn isqrt_u512(w3: u128, w2: u128, w1: u128, w0: u128) -> (u128, u128) {
    if w3 == 0 && w2 == 0 {
        return (0, isqrt_u256(w1, w0));
    }

    let total_bits = if w3 != 0 {
        384 + (128 - w3.leading_zeros())
    } else {
        256 + (128 - w2.leading_zeros())
    };

    let shift = total_bits.div_ceil(2);
    let mut x: (u128, u128) = if shift >= 256 {
        (u128::MAX, u128::MAX)
    } else if shift >= 128 {
        (1u128 << (shift - 128), 0)
    } else {
        (0, 1u128 << shift)
    };

    loop {
        let n_div_x = div_u512_by_u256(w3, w2, w1, w0, x.0, x.1);

        let (sum_lo, c1) = x.1.overflowing_add(n_div_x.1);
        let (sum_hi, c2) = x.0.overflowing_add(n_div_x.0);
        let (sum_hi, c3) = sum_hi.overflowing_add(c1 as u128);
        let carry = (c2 || c3) as u128;

        let x_new = (
            (carry << 127) | (sum_hi >> 1),
            ((sum_hi & 1) << 127) | (sum_lo >> 1),
        );

        if x_new.0 > x.0 || (x_new.0 == x.0 && x_new.1 >= x.1) {
            break;
        }
        x = x_new;
    }

    let (sq3, sq2, sq1, sq0) = mul_u256(x, x);
    if (sq3, sq2, sq1, sq0) > (w3, w2, w1, w0) {
        if x.1 == 0 {
            x.0 -= 1;
            x.1 = u128::MAX;
        } else {
            x.1 -= 1;
        }
    }

    x
}
