// tests/ancdec128_tests.rs
#![cfg(feature = "dec128")]

use ancdec::{AncDec128, RoundMode};

#[cfg(feature = "dec64")]
use ancdec::AncDec;

// ============ Parsing ============
#[test]
fn test_parse_integer() {
    let a: AncDec128 = "123".parse().unwrap();
    assert_eq!(a.int(), 123);
    assert_eq!(a.frac(), 0);
    assert_eq!(a.scale(), 0);
    assert!(!a.is_neg());
}

#[test]
fn test_parse_decimal() {
    let a: AncDec128 = "123.456".parse().unwrap();
    assert_eq!(a.int(), 123);
    assert_eq!(a.frac(), 456);
    assert_eq!(a.scale(), 3);
    assert!(!a.is_neg());
}

#[test]
fn test_parse_negative() {
    let a: AncDec128 = "-99.05".parse().unwrap();
    assert_eq!(a.int(), 99);
    assert_eq!(a.frac(), 5);
    assert_eq!(a.scale(), 2);
    assert!(a.is_neg());
}

#[test]
fn test_parse_leading_zero_frac() {
    let a: AncDec128 = "1.005".parse().unwrap();
    assert_eq!(a.int(), 1);
    assert_eq!(a.frac(), 5);
    assert_eq!(a.scale(), 3);
}

#[test]
fn test_parse_invalid() {
    assert!("".parse::<AncDec128>().is_err());
    assert!("-".parse::<AncDec128>().is_err());
    assert!("abc".parse::<AncDec128>().is_err());
    assert!("12.34.56".parse::<AncDec128>().is_err());
}

// ============ High-Precision Parsing (exceeds AncDec u64 capacity) ============
#[test]
fn test_parse_high_precision_frac() {
    // 38-digit fractional part
    let a: AncDec128 = "0.12345678901234567890123456789012345678".parse().unwrap();
    assert_eq!(a.scale(), 38);
    assert_eq!(a.frac(), 12345678901234567890123456789012345678u128);
}

#[test]
fn test_parse_large_integer() {
    // Larger than u64::MAX
    let a: AncDec128 = "99999999999999999999".parse().unwrap();
    assert_eq!(a.int(), 99999999999999999999u128);
    assert!(!a.is_neg());
}

#[test]
fn test_parse_very_large_integer() {
    let a: AncDec128 = "340282366920938463463374607431768211455".parse().unwrap();
    assert_eq!(a.int(), u128::MAX);
}

// ============ Display ============
#[test]
fn test_display_integer() {
    let a: AncDec128 = "42".parse().unwrap();
    assert_eq!(format!("{}", a), "42");
}

#[test]
fn test_display_decimal() {
    let a: AncDec128 = "123.456".parse().unwrap();
    assert_eq!(format!("{}", a), "123.456");
}

#[test]
fn test_display_negative() {
    let a: AncDec128 = "-99.05".parse().unwrap();
    assert_eq!(format!("{}", a), "-99.05");
}

#[test]
fn test_display_precision() {
    let a: AncDec128 = "123.456".parse().unwrap();
    assert_eq!(format!("{:.2}", a), "123.45");
    assert_eq!(format!("{:.5}", a), "123.45600");
    assert_eq!(format!("{:.0}", a), "123");
}

#[test]
fn test_display_high_precision() {
    let a: AncDec128 = "1.12345678901234567890".parse().unwrap();
    assert_eq!(format!("{}", a), "1.12345678901234567890");
}

// ============ Comparison ============
#[test]
fn test_eq() {
    let a: AncDec128 = "123.45".parse().unwrap();
    let b: AncDec128 = "123.450".parse().unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_ord() {
    let a: AncDec128 = "100".parse().unwrap();
    let b: AncDec128 = "99.99".parse().unwrap();
    assert!(a > b);
}

#[test]
fn test_ord_negative() {
    let a: AncDec128 = "-10".parse().unwrap();
    let b: AncDec128 = "-20".parse().unwrap();
    assert!(a > b);
}

#[test]
fn test_ord_mixed_sign() {
    let a: AncDec128 = "1".parse().unwrap();
    let b: AncDec128 = "-1000".parse().unwrap();
    assert!(a > b);
}

// ============ Addition ============
#[test]
fn test_add_simple() {
    let a: AncDec128 = "1.5".parse().unwrap();
    let b: AncDec128 = "2.5".parse().unwrap();
    assert_eq!(a + b, "4".parse().unwrap());
}

#[test]
fn test_add_different_scale() {
    let a: AncDec128 = "1.1".parse().unwrap();
    let b: AncDec128 = "2.22".parse().unwrap();
    assert_eq!(a + b, "3.32".parse().unwrap());
}

#[test]
fn test_add_with_carry() {
    let a: AncDec128 = "0.9".parse().unwrap();
    let b: AncDec128 = "0.2".parse().unwrap();
    assert_eq!(a + b, "1.1".parse().unwrap());
}

#[test]
fn test_add_negative() {
    let a: AncDec128 = "10".parse().unwrap();
    let b: AncDec128 = "-3".parse().unwrap();
    assert_eq!(a + b, "7".parse().unwrap());
}

// ============ Subtraction ============
#[test]
fn test_sub_simple() {
    let a: AncDec128 = "5.5".parse().unwrap();
    let b: AncDec128 = "2.3".parse().unwrap();
    assert_eq!(a - b, "3.2".parse().unwrap());
}

#[test]
fn test_sub_with_borrow() {
    let a: AncDec128 = "1.0".parse().unwrap();
    let b: AncDec128 = "0.3".parse().unwrap();
    assert_eq!(a - b, "0.7".parse().unwrap());
}

#[test]
fn test_sub_result_negative() {
    let a: AncDec128 = "3".parse().unwrap();
    let b: AncDec128 = "5".parse().unwrap();
    assert_eq!(a - b, "-2".parse().unwrap());
}

// ============ Multiplication ============
#[test]
fn test_mul_simple() {
    let a: AncDec128 = "2".parse().unwrap();
    let b: AncDec128 = "3".parse().unwrap();
    assert_eq!(a * b, "6".parse().unwrap());
}

#[test]
fn test_mul_decimal() {
    let a: AncDec128 = "1.5".parse().unwrap();
    let b: AncDec128 = "2".parse().unwrap();
    assert_eq!(a * b, "3".parse().unwrap());
}

#[test]
fn test_mul_negative() {
    let a: AncDec128 = "-3".parse().unwrap();
    let b: AncDec128 = "4".parse().unwrap();
    assert_eq!(a * b, "-12".parse().unwrap());
}

#[test]
fn test_mul_both_negative() {
    let a: AncDec128 = "-3".parse().unwrap();
    let b: AncDec128 = "-4".parse().unwrap();
    assert_eq!(a * b, "12".parse().unwrap());
}

// ============ Division ============
#[test]
fn test_div_simple() {
    let a: AncDec128 = "10".parse().unwrap();
    let b: AncDec128 = "2".parse().unwrap();
    assert_eq!(a / b, "5".parse().unwrap());
}

#[test]
fn test_div_decimal_result() {
    let a: AncDec128 = "1".parse().unwrap();
    let b: AncDec128 = "4".parse().unwrap();
    assert_eq!(a / b, "0.25".parse().unwrap());
}

#[test]
fn test_div_negative() {
    let a: AncDec128 = "-10".parse().unwrap();
    let b: AncDec128 = "4".parse().unwrap();
    assert_eq!(a / b, "-2.5".parse().unwrap());
}

// ============ Remainder ============
#[test]
fn test_rem_simple() {
    let a: AncDec128 = "10".parse().unwrap();
    let b: AncDec128 = "3".parse().unwrap();
    assert_eq!(a % b, "1".parse().unwrap());
}

#[test]
fn test_rem_decimal() {
    let a: AncDec128 = "5.5".parse().unwrap();
    let b: AncDec128 = "2".parse().unwrap();
    assert_eq!(a % b, "1.5".parse().unwrap());
}

// ============ Negation ============
#[test]
fn test_neg() {
    let a: AncDec128 = "5".parse().unwrap();
    assert_eq!(-a, "-5".parse().unwrap());
}

#[test]
fn test_neg_negative() {
    let a: AncDec128 = "-5".parse().unwrap();
    assert_eq!(-a, "5".parse().unwrap());
}

// ============ Assign Ops ============
#[test]
fn test_add_assign() {
    let mut a: AncDec128 = "5".parse().unwrap();
    a += "3".parse::<AncDec128>().unwrap();
    assert_eq!(a, "8".parse().unwrap());
}

#[test]
fn test_sub_assign() {
    let mut a: AncDec128 = "5".parse().unwrap();
    a -= "3".parse::<AncDec128>().unwrap();
    assert_eq!(a, "2".parse().unwrap());
}

#[test]
fn test_mul_assign() {
    let mut a: AncDec128 = "5".parse().unwrap();
    a *= "3".parse::<AncDec128>().unwrap();
    assert_eq!(a, "15".parse().unwrap());
}

#[test]
fn test_div_assign() {
    let mut a: AncDec128 = "15".parse().unwrap();
    a /= "3".parse::<AncDec128>().unwrap();
    assert_eq!(a, "5".parse().unwrap());
}

// ============ Reference Ops ============
#[test]
fn test_ref_add() {
    let a: AncDec128 = "1".parse().unwrap();
    let b: AncDec128 = "2".parse().unwrap();
    assert_eq!(&a + &b, "3".parse().unwrap());
    assert_eq!(a + &b, "3".parse().unwrap());
    assert_eq!(&a + b, "3".parse().unwrap());
}

// ============ Primitive Ops ============
#[test]
fn test_add_primitive() {
    let a: AncDec128 = "10".parse().unwrap();
    assert_eq!(a + 5i32, "15".parse().unwrap());
    assert_eq!(5i32 + a, "15".parse().unwrap());
}

#[test]
fn test_mul_primitive() {
    let a: AncDec128 = "10".parse().unwrap();
    assert_eq!(a * 2u64, "20".parse().unwrap());
    assert_eq!(2u64 * a, "20".parse().unwrap());
}

#[test]
fn test_div_with_f64() {
    let a: AncDec128 = "10".parse().unwrap();
    let b = AncDec128::try_from(4.0f64).unwrap();
    assert_eq!(a / b, "2.5".parse().unwrap());
}

// ============ From Integer ============
#[test]
fn test_from_i32() {
    let a = AncDec128::from(-42i32);
    assert_eq!(a.int(), 42);
    assert!(a.is_neg());
}

#[test]
fn test_from_u64() {
    let a = AncDec128::from(100u64);
    assert_eq!(a.int(), 100);
    assert!(!a.is_neg());
}

#[test]
fn test_from_u128() {
    let a = AncDec128::from(u128::MAX);
    assert_eq!(a.int(), u128::MAX);
    assert!(!a.is_neg());
}

// ============ From Float ============
#[test]
fn test_try_from_f64() {
    let a = AncDec128::try_from(3.14f64).unwrap();
    assert_eq!(a.int(), 3);
    assert_eq!(a.frac(), 14);
    assert_eq!(a.scale(), 2);
}

#[test]
fn test_try_from_f64_nan() {
    assert!(AncDec128::try_from(f64::NAN).is_err());
}

#[test]
fn test_try_from_f64_infinity() {
    assert!(AncDec128::try_from(f64::INFINITY).is_err());
    assert!(AncDec128::try_from(f64::NEG_INFINITY).is_err());
}

// ============ From AncDec (widening) ============
#[cfg(feature = "dec64")]
#[test]
fn test_from_ancdec() {
    let a: AncDec = "123.456".parse().unwrap();
    let b = AncDec128::from(a);
    assert_eq!(b.int(), 123);
    assert_eq!(b.frac(), 456);
    assert_eq!(b.scale(), 3);
    assert!(!b.is_neg());
}

#[cfg(feature = "dec64")]
#[test]
fn test_from_ancdec_negative() {
    let a: AncDec = "-99.05".parse().unwrap();
    let b = AncDec128::from(a);
    assert_eq!(b.int(), 99);
    assert_eq!(b.frac(), 5);
    assert_eq!(b.scale(), 2);
    assert!(b.is_neg());
}

// ============ Basic Methods ============
#[test]
fn test_abs() {
    let a: AncDec128 = "-5.5".parse().unwrap();
    assert_eq!(a.abs(), "5.5".parse().unwrap());
}

#[test]
fn test_signum() {
    assert_eq!("10".parse::<AncDec128>().unwrap().signum(), AncDec128::ONE);
    assert_eq!(
        "-10".parse::<AncDec128>().unwrap().signum(),
        "-1".parse().unwrap()
    );
    assert_eq!(AncDec128::ZERO.signum(), AncDec128::ZERO);
}

#[test]
fn test_is_zero() {
    assert!(AncDec128::ZERO.is_zero());
    assert!(!AncDec128::ONE.is_zero());
}

#[test]
fn test_is_positive() {
    assert!("5".parse::<AncDec128>().unwrap().is_positive());
    assert!(!"-5".parse::<AncDec128>().unwrap().is_positive());
    assert!(!AncDec128::ZERO.is_positive());
}

#[test]
fn test_is_negative() {
    assert!("-5".parse::<AncDec128>().unwrap().is_negative());
    assert!(!"5".parse::<AncDec128>().unwrap().is_negative());
    assert!(!AncDec128::ZERO.is_negative());
}

// ============ Min/Max/Clamp ============
#[test]
fn test_min() {
    let a: AncDec128 = "5".parse().unwrap();
    let b: AncDec128 = "3".parse().unwrap();
    assert_eq!(a.min(b), b);
}

#[test]
fn test_max() {
    let a: AncDec128 = "5".parse().unwrap();
    let b: AncDec128 = "3".parse().unwrap();
    assert_eq!(a.max(b), a);
}

#[test]
fn test_clamp() {
    let a: AncDec128 = "10".parse().unwrap();
    let min: AncDec128 = "0".parse().unwrap();
    let max: AncDec128 = "5".parse().unwrap();
    assert_eq!(a.clamp(min, max), max);
}

// ============ Rounding ============
#[test]
fn test_round_half_up() {
    let a: AncDec128 = "2.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfUp), "3".parse().unwrap());
}

#[test]
fn test_round_half_down() {
    let a: AncDec128 = "2.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfDown), "2".parse().unwrap());
}

#[test]
fn test_round_half_even() {
    let a: AncDec128 = "2.5".parse().unwrap();
    let b: AncDec128 = "3.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfEven), "2".parse().unwrap());
    assert_eq!(b.round(0, RoundMode::HalfEven), "4".parse().unwrap());
}

#[test]
fn test_round_truncate() {
    let a: AncDec128 = "2.9".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::Truncate), "2".parse().unwrap());
}

#[test]
fn test_round_decimal_places() {
    let a: AncDec128 = "3.14159".parse().unwrap();
    assert_eq!(a.round(2, RoundMode::HalfUp), "3.14".parse().unwrap());
    assert_eq!(a.round(3, RoundMode::HalfUp), "3.142".parse().unwrap());
}

// ============ Floor/Ceil/Trunc/Fract ============
#[test]
fn test_floor() {
    assert_eq!(
        "2.9".parse::<AncDec128>().unwrap().floor(),
        "2".parse().unwrap()
    );
    assert_eq!(
        "-2.1".parse::<AncDec128>().unwrap().floor(),
        "-3".parse().unwrap()
    );
}

#[test]
fn test_ceil() {
    assert_eq!(
        "2.1".parse::<AncDec128>().unwrap().ceil(),
        "3".parse().unwrap()
    );
    assert_eq!(
        "-2.9".parse::<AncDec128>().unwrap().ceil(),
        "-2".parse().unwrap()
    );
}

#[test]
fn test_trunc() {
    assert_eq!(
        "2.9".parse::<AncDec128>().unwrap().trunc(),
        "2".parse().unwrap()
    );
    assert_eq!(
        "-2.9".parse::<AncDec128>().unwrap().trunc(),
        "-2".parse().unwrap()
    );
}

#[test]
fn test_fract() {
    let a: AncDec128 = "3.14".parse().unwrap();
    let f = a.fract();
    assert_eq!(f.int(), 0);
    assert_eq!(f.frac(), 14);
}

// ============ Power ============
#[test]
fn test_pow_positive() {
    let a: AncDec128 = "2".parse().unwrap();
    assert_eq!(a.pow(3), "8".parse().unwrap());
}

#[test]
fn test_pow_zero() {
    let a: AncDec128 = "5".parse().unwrap();
    assert_eq!(a.pow(0), AncDec128::ONE);
}

#[test]
fn test_pow_negative() {
    let a: AncDec128 = "2".parse().unwrap();
    assert_eq!(a.pow(-1), "0.5".parse().unwrap());
}

// ============ Square Root ============
#[test]
fn test_sqrt_perfect_squares_128() {
    assert_eq!(AncDec128::ZERO.sqrt(), AncDec128::ZERO);
    let one: AncDec128 = "1".parse().unwrap();
    assert_eq!(one.sqrt().int(), 1);
    assert_eq!(one.sqrt().frac(), 0);
    let four: AncDec128 = "4".parse().unwrap();
    assert_eq!(four.sqrt().int(), 2);
    assert_eq!(four.sqrt().frac(), 0);
    let nine: AncDec128 = "9".parse().unwrap();
    assert_eq!(nine.sqrt().int(), 3);
    assert_eq!(nine.sqrt().frac(), 0);
}

#[test]
fn test_sqrt_two_128() {
    let two: AncDec128 = "2".parse().unwrap();
    let result = two.sqrt();
    assert_eq!(result.int(), 1);
    // sqrt(2) = 1.4142135623730950488016887242096980785...
    // floor(sqrt(2) * 10^37) mod 10^37
    assert_eq!(result.frac(), 4142135623730950488016887242096980785);
}

#[test]
fn test_sqrt_fractional_128() {
    let val: AncDec128 = "0.25".parse().unwrap();
    let r = val.sqrt();
    assert_eq!(r.int(), 0);
    assert_eq!(r.frac(), 5000000000000000000000000000000000000); // 0.5 * 10^37
    let val: AncDec128 = "0.01".parse().unwrap();
    let r = val.sqrt();
    assert_eq!(r.int(), 0);
    assert_eq!(r.frac(), 1000000000000000000000000000000000000); // 0.1 * 10^37
}

#[test]
fn test_sqrt_roundtrip_128() {
    let val: AncDec128 = "7".parse().unwrap();
    let root = val.sqrt();
    let sq = root * root;
    assert!(sq <= val);
}

#[test]
fn test_sqrt_large_128() {
    let val: AncDec128 = "1000000000000000000000000000000".parse().unwrap();
    let r = val.sqrt();
    // sqrt(10^30) = 10^15
    assert_eq!(r.int(), 1000000000000000);
    assert_eq!(r.frac(), 0);
}

#[test]
#[should_panic(expected = "square root of negative")]
fn test_sqrt_negative_panics_128() {
    let neg: AncDec128 = "-4".parse().unwrap();
    neg.sqrt();
}

// ============ Conversion ============
#[test]
fn test_to_f64() {
    let a: AncDec128 = "3.14".parse().unwrap();
    assert!((a.to_f64() - 3.14).abs() < 0.0001);
}

#[test]
fn test_to_i64() {
    let a: AncDec128 = "-42.99".parse().unwrap();
    assert_eq!(a.to_i64(), -42);
}

#[test]
fn test_to_i128() {
    let a: AncDec128 = "-42.99".parse().unwrap();
    assert_eq!(a.to_i128(), -42);
}

// ============ Default ============
#[test]
fn test_default() {
    let a: AncDec128 = Default::default();
    assert_eq!(a, AncDec128::ZERO);
}

// ============ Hash ============
#[test]
fn test_hash_equal_values() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a: AncDec128 = "123.45".parse().unwrap();
    let b: AncDec128 = "123.450".parse().unwrap();

    let mut h1 = DefaultHasher::new();
    let mut h2 = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);

    assert_eq!(h1.finish(), h2.finish());
}

// ============ Iterator ============
#[test]
fn test_sum() {
    let v = vec![
        "1".parse::<AncDec128>().unwrap(),
        "2".parse::<AncDec128>().unwrap(),
        "3".parse::<AncDec128>().unwrap(),
    ];
    let sum: AncDec128 = v.into_iter().sum();
    assert_eq!(sum, "6".parse().unwrap());
}

#[test]
fn test_product() {
    let v = vec![
        "2".parse::<AncDec128>().unwrap(),
        "3".parse::<AncDec128>().unwrap(),
        "4".parse::<AncDec128>().unwrap(),
    ];
    let prod: AncDec128 = v.into_iter().product();
    assert_eq!(prod, "24".parse().unwrap());
}

// ============ Constants ============
#[test]
fn test_constants() {
    assert_eq!(AncDec128::ZERO.int(), 0);
    assert_eq!(AncDec128::ONE.int(), 1);
    assert_eq!(AncDec128::TWO.int(), 2);
    assert_eq!(AncDec128::TEN.int(), 10);
}

// ============ Edge Cases ============
#[test]
fn test_zero_operations() {
    let zero = AncDec128::ZERO;
    let one = AncDec128::ONE;

    assert_eq!(zero + one, one);
    assert_eq!(one - one, zero);
    assert_eq!(zero * one, zero);
}

#[test]
fn test_high_precision() {
    let a: AncDec128 = "0.12345678901234567890123456789012345678".parse().unwrap();
    assert_eq!(a.scale(), 38);
}

// ============ Overflow/Truncation ============
#[test]
fn test_int_overflow_returns_error() {
    // u128::MAX = 340282366920938463463374607431768211455 (39 digits); parsing larger should return Overflow error
    let result: Result<AncDec128, _> = "9999999999999999999999999999999999999999".parse();
    assert!(result.is_err());
}

#[test]
fn test_frac_truncates_at_38() {
    let a: AncDec128 = "0.123456789012345678901234567890123456789999".parse().unwrap();
    assert_eq!(a.scale(), 38);
}

#[test]
#[should_panic]
fn test_div_by_zero_panics() {
    let a: AncDec128 = "5".parse().unwrap();
    let _ = a / AncDec128::ZERO;
}

#[test]
fn test_negative_zero_equals_zero() {
    let neg_zero: AncDec128 = "-0".parse().unwrap();
    assert_eq!(neg_zero, AncDec128::ZERO);
}

// ============ Serde (only with feature) ============
#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let a: AncDec128 = "123.456".parse().unwrap();
    let json = serde_json::to_string(&a).unwrap();
    assert_eq!(json, "\"123.456\"");

    let b: AncDec128 = serde_json::from_str(&json).unwrap();
    assert_eq!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_negative() {
    let a: AncDec128 = "-99.99".parse().unwrap();
    let json = serde_json::to_string(&a).unwrap();
    let b: AncDec128 = serde_json::from_str(&json).unwrap();
    assert_eq!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_invalid() {
    let result: Result<AncDec128, _> = serde_json::from_str("\"abc\"");
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_high_precision() {
    let a: AncDec128 = "12345678901234567890.12345678901234567890".parse().unwrap();
    let json = serde_json::to_string(&a).unwrap();
    let b: AncDec128 = serde_json::from_str(&json).unwrap();
    assert_eq!(a, b);
}
