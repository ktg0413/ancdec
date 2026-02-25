// tests/ancdec8_tests.rs
#![cfg(feature = "dec8")]

use ancdec::{AncDec8, RoundMode};

// ============ Parsing ============
#[test]
fn test_parse_integer() {
    let a: AncDec8 = "12".parse().unwrap();
    assert_eq!(a.int(), 12);
    assert_eq!(a.frac(), 0);
    assert_eq!(a.scale(), 0);
    assert!(!a.is_neg());
}

#[test]
fn test_parse_decimal() {
    let a: AncDec8 = "1.23".parse().unwrap();
    assert_eq!(a.int(), 1);
    assert_eq!(a.frac(), 23);
    assert_eq!(a.scale(), 2);
}

#[test]
fn test_parse_negative() {
    let a: AncDec8 = "-9.5".parse().unwrap();
    assert_eq!(a.int(), 9);
    assert_eq!(a.frac(), 5);
    assert_eq!(a.scale(), 1);
    assert!(a.is_neg());
}

#[test]
fn test_parse_leading_zero_frac() {
    let a: AncDec8 = "1.05".parse().unwrap();
    assert_eq!(a.int(), 1);
    assert_eq!(a.frac(), 5);
    assert_eq!(a.scale(), 2);
}

#[test]
fn test_parse_invalid() {
    assert!("".parse::<AncDec8>().is_err());
    assert!("-".parse::<AncDec8>().is_err());
    assert!("abc".parse::<AncDec8>().is_err());
    assert!("12.34.56".parse::<AncDec8>().is_err());
}

// ============ Display ============
#[test]
fn test_display_integer() {
    let a: AncDec8 = "42".parse().unwrap();
    assert_eq!(format!("{}", a), "42");
}

#[test]
fn test_display_decimal() {
    let a: AncDec8 = "1.23".parse().unwrap();
    assert_eq!(format!("{}", a), "1.23");
}

#[test]
fn test_display_negative() {
    let a: AncDec8 = "-9.05".parse().unwrap();
    assert_eq!(format!("{}", a), "-9.05");
}

#[test]
fn test_display_precision() {
    let a: AncDec8 = "1.23".parse().unwrap();
    assert_eq!(format!("{:.1}", a), "1.2");
    assert_eq!(format!("{:.4}", a), "1.2300");
    assert_eq!(format!("{:.0}", a), "1");
}

// ============ Comparison ============
#[test]
fn test_eq() {
    let a: AncDec8 = "1.2".parse().unwrap();
    let b: AncDec8 = "1.20".parse().unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_ord() {
    let a: AncDec8 = "10".parse().unwrap();
    let b: AncDec8 = "9.99".parse().unwrap();
    assert!(a > b);
}

#[test]
fn test_ord_negative() {
    let a: AncDec8 = "-5".parse().unwrap();
    let b: AncDec8 = "-10".parse().unwrap();
    assert!(a > b);
}

#[test]
fn test_ord_mixed_sign() {
    let a: AncDec8 = "1".parse().unwrap();
    let b: AncDec8 = "-100".parse().unwrap();
    assert!(a > b);
}

// ============ Addition ============
#[test]
fn test_add_simple() {
    let a: AncDec8 = "1.5".parse().unwrap();
    let b: AncDec8 = "2.5".parse().unwrap();
    assert_eq!(a + b, "4".parse().unwrap());
}

#[test]
fn test_add_different_scale() {
    let a: AncDec8 = "1.1".parse().unwrap();
    let b: AncDec8 = "2.22".parse().unwrap();
    assert_eq!(a + b, "3.32".parse().unwrap());
}

#[test]
fn test_add_with_carry() {
    let a: AncDec8 = "0.9".parse().unwrap();
    let b: AncDec8 = "0.2".parse().unwrap();
    assert_eq!(a + b, "1.1".parse().unwrap());
}

#[test]
fn test_add_negative() {
    let a: AncDec8 = "10".parse().unwrap();
    let b: AncDec8 = "-3".parse().unwrap();
    assert_eq!(a + b, "7".parse().unwrap());
}

// ============ Subtraction ============
#[test]
fn test_sub_simple() {
    let a: AncDec8 = "5.5".parse().unwrap();
    let b: AncDec8 = "2.3".parse().unwrap();
    assert_eq!(a - b, "3.2".parse().unwrap());
}

#[test]
fn test_sub_with_borrow() {
    let a: AncDec8 = "1.0".parse().unwrap();
    let b: AncDec8 = "0.3".parse().unwrap();
    assert_eq!(a - b, "0.7".parse().unwrap());
}

#[test]
fn test_sub_result_negative() {
    let a: AncDec8 = "3".parse().unwrap();
    let b: AncDec8 = "5".parse().unwrap();
    assert_eq!(a - b, "-2".parse().unwrap());
}

// ============ Multiplication ============
#[test]
fn test_mul_simple() {
    let a: AncDec8 = "2".parse().unwrap();
    let b: AncDec8 = "3".parse().unwrap();
    assert_eq!(a * b, "6".parse().unwrap());
}

#[test]
fn test_mul_decimal() {
    let a: AncDec8 = "1.5".parse().unwrap();
    let b: AncDec8 = "2".parse().unwrap();
    assert_eq!(a * b, "3".parse().unwrap());
}

#[test]
fn test_mul_negative() {
    let a: AncDec8 = "-3".parse().unwrap();
    let b: AncDec8 = "4".parse().unwrap();
    assert_eq!(a * b, "-12".parse().unwrap());
}

#[test]
fn test_mul_both_negative() {
    let a: AncDec8 = "-3".parse().unwrap();
    let b: AncDec8 = "-4".parse().unwrap();
    assert_eq!(a * b, "12".parse().unwrap());
}

// ============ Division ============
#[test]
fn test_div_simple() {
    let a: AncDec8 = "10".parse().unwrap();
    let b: AncDec8 = "2".parse().unwrap();
    assert_eq!(a / b, "5".parse().unwrap());
}

#[test]
fn test_div_decimal_result() {
    let a: AncDec8 = "1".parse().unwrap();
    let b: AncDec8 = "4".parse().unwrap();
    assert_eq!(a / b, "0.25".parse().unwrap());
}

#[test]
fn test_div_negative() {
    let a: AncDec8 = "-10".parse().unwrap();
    let b: AncDec8 = "4".parse().unwrap();
    assert_eq!(a / b, "-2.5".parse().unwrap());
}

// ============ Remainder ============
#[test]
fn test_rem_simple() {
    let a: AncDec8 = "10".parse().unwrap();
    let b: AncDec8 = "3".parse().unwrap();
    assert_eq!(a % b, "1".parse().unwrap());
}

#[test]
fn test_rem_decimal() {
    let a: AncDec8 = "5.5".parse().unwrap();
    let b: AncDec8 = "2".parse().unwrap();
    assert_eq!(a % b, "1.5".parse().unwrap());
}

// ============ Negation ============
#[test]
fn test_neg() {
    let a: AncDec8 = "5".parse().unwrap();
    assert_eq!(-a, "-5".parse().unwrap());
}

#[test]
fn test_neg_negative() {
    let a: AncDec8 = "-5".parse().unwrap();
    assert_eq!(-a, "5".parse().unwrap());
}

// ============ Assign Ops ============
#[test]
fn test_add_assign() {
    let mut a: AncDec8 = "5".parse().unwrap();
    a += "3".parse::<AncDec8>().unwrap();
    assert_eq!(a, "8".parse().unwrap());
}

#[test]
fn test_sub_assign() {
    let mut a: AncDec8 = "5".parse().unwrap();
    a -= "3".parse::<AncDec8>().unwrap();
    assert_eq!(a, "2".parse().unwrap());
}

#[test]
fn test_mul_assign() {
    let mut a: AncDec8 = "5".parse().unwrap();
    a *= "3".parse::<AncDec8>().unwrap();
    assert_eq!(a, "15".parse().unwrap());
}

#[test]
fn test_div_assign() {
    let mut a: AncDec8 = "15".parse().unwrap();
    a /= "3".parse::<AncDec8>().unwrap();
    assert_eq!(a, "5".parse().unwrap());
}

// ============ Reference Ops ============
#[test]
fn test_ref_add() {
    let a: AncDec8 = "1".parse().unwrap();
    let b: AncDec8 = "2".parse().unwrap();
    assert_eq!(&a + &b, "3".parse().unwrap());
    assert_eq!(a + &b, "3".parse().unwrap());
    assert_eq!(&a + b, "3".parse().unwrap());
}

// ============ From Integer ============
#[test]
fn test_from_u8() {
    let a = AncDec8::from(42u8);
    assert_eq!(a.int(), 42);
    assert!(!a.is_neg());
}

#[test]
fn test_from_i8() {
    let a = AncDec8::from(-42i8);
    assert_eq!(a.int(), 42);
    assert!(a.is_neg());
}

// ============ TryFrom Float ============
#[test]
fn test_try_from_f64() {
    let a = AncDec8::try_from(3.14f64).unwrap();
    assert_eq!(a.int(), 3);
    assert_eq!(a.frac(), 14);
    assert_eq!(a.scale(), 2);
}

#[test]
fn test_try_from_f64_nan() {
    assert!(AncDec8::try_from(f64::NAN).is_err());
}

#[test]
fn test_try_from_f64_infinity() {
    assert!(AncDec8::try_from(f64::INFINITY).is_err());
    assert!(AncDec8::try_from(f64::NEG_INFINITY).is_err());
}

// ============ Basic Methods ============
#[test]
fn test_abs() {
    let a: AncDec8 = "-5.5".parse().unwrap();
    assert_eq!(a.abs(), "5.5".parse().unwrap());
}

#[test]
fn test_signum() {
    assert_eq!("10".parse::<AncDec8>().unwrap().signum(), AncDec8::ONE);
    assert_eq!(
        "-10".parse::<AncDec8>().unwrap().signum(),
        "-1".parse().unwrap()
    );
    assert_eq!(AncDec8::ZERO.signum(), AncDec8::ZERO);
}

#[test]
fn test_is_zero() {
    assert!(AncDec8::ZERO.is_zero());
    assert!(!AncDec8::ONE.is_zero());
}

#[test]
fn test_is_positive() {
    assert!("5".parse::<AncDec8>().unwrap().is_positive());
    assert!(!"-5".parse::<AncDec8>().unwrap().is_positive());
    assert!(!AncDec8::ZERO.is_positive());
}

#[test]
fn test_is_negative() {
    assert!("-5".parse::<AncDec8>().unwrap().is_negative());
    assert!(!"5".parse::<AncDec8>().unwrap().is_negative());
    assert!(!AncDec8::ZERO.is_negative());
}

// ============ Min/Max/Clamp ============
#[test]
fn test_min() {
    let a: AncDec8 = "5".parse().unwrap();
    let b: AncDec8 = "3".parse().unwrap();
    assert_eq!(a.min(b), b);
}

#[test]
fn test_max() {
    let a: AncDec8 = "5".parse().unwrap();
    let b: AncDec8 = "3".parse().unwrap();
    assert_eq!(a.max(b), a);
}

#[test]
fn test_clamp() {
    let a: AncDec8 = "10".parse().unwrap();
    let min: AncDec8 = "0".parse().unwrap();
    let max: AncDec8 = "5".parse().unwrap();
    assert_eq!(a.clamp(min, max), max);
}

// ============ Rounding ============
#[test]
fn test_round_half_up() {
    let a: AncDec8 = "2.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfUp), "3".parse().unwrap());
}

#[test]
fn test_round_half_down() {
    let a: AncDec8 = "2.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfDown), "2".parse().unwrap());
}

#[test]
fn test_round_half_even() {
    let a: AncDec8 = "2.5".parse().unwrap();
    let b: AncDec8 = "3.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfEven), "2".parse().unwrap());
    assert_eq!(b.round(0, RoundMode::HalfEven), "4".parse().unwrap());
}

#[test]
fn test_round_truncate() {
    let a: AncDec8 = "2.9".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::Truncate), "2".parse().unwrap());
}

#[test]
fn test_round_floor() {
    let a: AncDec8 = "2.9".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::Floor), "2".parse().unwrap());
    let b: AncDec8 = "-2.1".parse().unwrap();
    assert_eq!(b.round(0, RoundMode::Floor), "-3".parse().unwrap());
}

#[test]
fn test_round_ceil() {
    let a: AncDec8 = "2.1".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::Ceil), "3".parse().unwrap());
    let b: AncDec8 = "-2.9".parse().unwrap();
    assert_eq!(b.round(0, RoundMode::Ceil), "-2".parse().unwrap());
}

#[test]
fn test_round_decimal_places() {
    let a: AncDec8 = "1.25".parse().unwrap();
    assert_eq!(a.round(1, RoundMode::HalfUp), "1.3".parse().unwrap());
}

// ============ Floor/Ceil/Trunc/Fract ============
#[test]
fn test_floor() {
    assert_eq!(
        "2.9".parse::<AncDec8>().unwrap().floor(),
        "2".parse().unwrap()
    );
    assert_eq!(
        "-2.1".parse::<AncDec8>().unwrap().floor(),
        "-3".parse().unwrap()
    );
}

#[test]
fn test_ceil() {
    assert_eq!(
        "2.1".parse::<AncDec8>().unwrap().ceil(),
        "3".parse().unwrap()
    );
    assert_eq!(
        "-2.9".parse::<AncDec8>().unwrap().ceil(),
        "-2".parse().unwrap()
    );
}

#[test]
fn test_trunc() {
    assert_eq!(
        "2.9".parse::<AncDec8>().unwrap().trunc(),
        "2".parse().unwrap()
    );
    assert_eq!(
        "-2.9".parse::<AncDec8>().unwrap().trunc(),
        "-2".parse().unwrap()
    );
}

#[test]
fn test_fract() {
    let a: AncDec8 = "3.14".parse().unwrap();
    let f = a.fract();
    assert_eq!(f.int(), 0);
    assert_eq!(f.frac(), 14);
}

// ============ Power ============
#[test]
fn test_pow_positive() {
    let a: AncDec8 = "2".parse().unwrap();
    assert_eq!(a.pow(2), "4".parse().unwrap());
}

#[test]
fn test_pow_zero() {
    let a: AncDec8 = "5".parse().unwrap();
    assert_eq!(a.pow(0), AncDec8::ONE);
}

#[test]
fn test_pow_negative() {
    let a: AncDec8 = "2".parse().unwrap();
    assert_eq!(a.pow(-1), "0.5".parse().unwrap());
}

// ============ Square Root ============
#[test]
fn test_sqrt_four() {
    let four: AncDec8 = "4".parse().unwrap();
    let result = four.sqrt();
    assert_eq!(result.int(), 2);
    assert_eq!(result.frac(), 0);
}

#[test]
fn test_sqrt_one() {
    let one: AncDec8 = "1".parse().unwrap();
    let result = one.sqrt();
    assert_eq!(result.int(), 1);
    assert_eq!(result.frac(), 0);
}

#[test]
fn test_sqrt_zero() {
    assert_eq!(AncDec8::ZERO.sqrt(), AncDec8::ZERO);
}

#[test]
#[should_panic(expected = "square root of negative")]
fn test_sqrt_negative_panics() {
    let neg: AncDec8 = "-4".parse().unwrap();
    neg.sqrt();
}

// ============ Conversion ============
#[test]
fn test_to_f64() {
    let a: AncDec8 = "3.14".parse().unwrap();
    assert!((a.to_f64() - 3.14).abs() < 0.01);
}

#[test]
fn test_to_i64() {
    let a: AncDec8 = "-42.99".parse().unwrap();
    assert_eq!(a.to_i64(), -42);
}

#[test]
fn test_to_i128() {
    let a: AncDec8 = "-99.5".parse().unwrap();
    assert_eq!(a.to_i128(), -99);
}

// ============ Default ============
#[test]
fn test_default() {
    let a: AncDec8 = Default::default();
    assert_eq!(a, AncDec8::ZERO);
}

// ============ Hash ============
#[test]
fn test_hash_equal_values() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a: AncDec8 = "1.2".parse().unwrap();
    let b: AncDec8 = "1.20".parse().unwrap();

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
        "1".parse::<AncDec8>().unwrap(),
        "2".parse::<AncDec8>().unwrap(),
        "3".parse::<AncDec8>().unwrap(),
    ];
    let sum: AncDec8 = v.into_iter().sum();
    assert_eq!(sum, "6".parse().unwrap());
}

#[test]
fn test_product() {
    let v = vec![
        "2".parse::<AncDec8>().unwrap(),
        "3".parse::<AncDec8>().unwrap(),
        "4".parse::<AncDec8>().unwrap(),
    ];
    let prod: AncDec8 = v.into_iter().product();
    assert_eq!(prod, "24".parse().unwrap());
}

// ============ Constants ============
#[test]
fn test_constants() {
    assert_eq!(AncDec8::ZERO.int(), 0);
    assert_eq!(AncDec8::ONE.int(), 1);
    assert_eq!(AncDec8::TWO.int(), 2);
    assert_eq!(AncDec8::TEN.int(), 10);
}

// ============ Edge Cases ============
#[test]
fn test_zero_operations() {
    let zero = AncDec8::ZERO;
    let one = AncDec8::ONE;

    assert_eq!(zero + one, one);
    assert_eq!(one - one, zero);
    assert_eq!(zero * one, zero);
}

#[test]
fn test_int_overflow_returns_error() {
    // u8::MAX = 255; parsing a larger number should return Overflow error
    let result: Result<AncDec8, _> = "999".parse();
    assert!(result.is_err());
}

#[test]
fn test_frac_truncates_at_2() {
    // Fractional part truncates at 2 digits
    let a: AncDec8 = "0.123456".parse().unwrap();
    assert_eq!(a.scale(), 2);
    assert_eq!(a.frac(), 12);
}

#[test]
#[should_panic]
fn test_div_by_zero_panics() {
    let a: AncDec8 = "5".parse().unwrap();
    let _ = a / AncDec8::ZERO;
}

#[test]
fn test_negative_zero_equals_zero() {
    let neg_zero: AncDec8 = "-0".parse().unwrap();
    assert_eq!(neg_zero, AncDec8::ZERO);
}

// ============ Primitive Ops ============
#[test]
fn test_add_primitive() {
    let a: AncDec8 = "10".parse().unwrap();
    assert_eq!(a + 5u8, "15".parse().unwrap());
    assert_eq!(5u8 + a, "15".parse().unwrap());
}

#[test]
fn test_mul_primitive() {
    let a: AncDec8 = "10".parse().unwrap();
    assert_eq!(a * 2u8, "20".parse().unwrap());
    assert_eq!(2u8 * a, "20".parse().unwrap());
}

// ============ Cross-type Ops with AncDec32 ============
#[cfg(feature = "dec32")]
#[test]
fn test_cross_ops_with_ancdec32() {
    let a: AncDec8 = "5".parse().unwrap();
    let b: ancdec::AncDec32 = "3.14".parse().unwrap();
    let result = a + b; // AncDec32
    assert_eq!(result, "8.14".parse().unwrap());
}

// ============ Serde ============
#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let a: AncDec8 = "1.23".parse().unwrap();
    let json = serde_json::to_string(&a).unwrap();
    assert_eq!(json, "\"1.23\"");
    let b: AncDec8 = serde_json::from_str(&json).unwrap();
    assert_eq!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_negative() {
    let a: AncDec8 = "-9.99".parse().unwrap();
    let json = serde_json::to_string(&a).unwrap();
    let b: AncDec8 = serde_json::from_str(&json).unwrap();
    assert_eq!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_invalid() {
    let result: Result<AncDec8, _> = serde_json::from_str("\"abc\"");
    assert!(result.is_err());
}
