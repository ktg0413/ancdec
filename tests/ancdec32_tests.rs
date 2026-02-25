// tests/ancdec32_tests.rs
#![cfg(feature = "dec32")]

use ancdec::{AncDec32, RoundMode};

// ============ Parsing ============
#[test]
fn test_parse_integer() {
    let a: AncDec32 = "123".parse().unwrap();
    assert_eq!(a.int(), 123);
    assert_eq!(a.frac(), 0);
    assert_eq!(a.scale(), 0);
    assert!(!a.is_neg());
}

#[test]
fn test_parse_decimal() {
    let a: AncDec32 = "123.456789".parse().unwrap();
    assert_eq!(a.int(), 123);
    assert_eq!(a.frac(), 456789);
    assert_eq!(a.scale(), 6);
}

#[test]
fn test_parse_negative() {
    let a: AncDec32 = "-99.05".parse().unwrap();
    assert_eq!(a.int(), 99);
    assert_eq!(a.frac(), 5);
    assert_eq!(a.scale(), 2);
    assert!(a.is_neg());
}

#[test]
fn test_parse_leading_zero_frac() {
    let a: AncDec32 = "1.005".parse().unwrap();
    assert_eq!(a.int(), 1);
    assert_eq!(a.frac(), 5);
    assert_eq!(a.scale(), 3);
}

#[test]
fn test_parse_high_precision() {
    let a: AncDec32 = "1.123456789".parse().unwrap();
    assert_eq!(a.int(), 1);
    assert_eq!(a.frac(), 123456789);
    assert_eq!(a.scale(), 9);
}

#[test]
fn test_parse_invalid() {
    assert!("".parse::<AncDec32>().is_err());
    assert!("-".parse::<AncDec32>().is_err());
    assert!("abc".parse::<AncDec32>().is_err());
    assert!("12.34.56".parse::<AncDec32>().is_err());
}

// ============ Display ============
#[test]
fn test_display_integer() {
    let a: AncDec32 = "42".parse().unwrap();
    assert_eq!(format!("{}", a), "42");
}

#[test]
fn test_display_decimal() {
    let a: AncDec32 = "123.456".parse().unwrap();
    assert_eq!(format!("{}", a), "123.456");
}

#[test]
fn test_display_negative() {
    let a: AncDec32 = "-99.05".parse().unwrap();
    assert_eq!(format!("{}", a), "-99.05");
}

#[test]
fn test_display_precision() {
    let a: AncDec32 = "123.456".parse().unwrap();
    assert_eq!(format!("{:.2}", a), "123.45");
    assert_eq!(format!("{:.5}", a), "123.45600");
    assert_eq!(format!("{:.0}", a), "123");
}

// ============ Comparison ============
#[test]
fn test_eq() {
    let a: AncDec32 = "123.45".parse().unwrap();
    let b: AncDec32 = "123.450".parse().unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_ord() {
    let a: AncDec32 = "100".parse().unwrap();
    let b: AncDec32 = "99.99".parse().unwrap();
    assert!(a > b);
}

#[test]
fn test_ord_negative() {
    let a: AncDec32 = "-10".parse().unwrap();
    let b: AncDec32 = "-20".parse().unwrap();
    assert!(a > b);
}

#[test]
fn test_ord_mixed_sign() {
    let a: AncDec32 = "1".parse().unwrap();
    let b: AncDec32 = "-1000".parse().unwrap();
    assert!(a > b);
}

// ============ Addition ============
#[test]
fn test_add_simple() {
    let a: AncDec32 = "1.5".parse().unwrap();
    let b: AncDec32 = "2.5".parse().unwrap();
    assert_eq!(a + b, "4".parse().unwrap());
}

#[test]
fn test_add_different_scale() {
    let a: AncDec32 = "1.1".parse().unwrap();
    let b: AncDec32 = "2.22".parse().unwrap();
    assert_eq!(a + b, "3.32".parse().unwrap());
}

#[test]
fn test_add_with_carry() {
    let a: AncDec32 = "0.9".parse().unwrap();
    let b: AncDec32 = "0.2".parse().unwrap();
    assert_eq!(a + b, "1.1".parse().unwrap());
}

#[test]
fn test_add_negative() {
    let a: AncDec32 = "10".parse().unwrap();
    let b: AncDec32 = "-3".parse().unwrap();
    assert_eq!(a + b, "7".parse().unwrap());
}

#[test]
fn test_add_large() {
    let a: AncDec32 = "1000000".parse().unwrap();
    let b: AncDec32 = "2000000".parse().unwrap();
    assert_eq!(a + b, "3000000".parse().unwrap());
}

// ============ Subtraction ============
#[test]
fn test_sub_simple() {
    let a: AncDec32 = "5.5".parse().unwrap();
    let b: AncDec32 = "2.3".parse().unwrap();
    assert_eq!(a - b, "3.2".parse().unwrap());
}

#[test]
fn test_sub_with_borrow() {
    let a: AncDec32 = "1.0".parse().unwrap();
    let b: AncDec32 = "0.3".parse().unwrap();
    assert_eq!(a - b, "0.7".parse().unwrap());
}

#[test]
fn test_sub_result_negative() {
    let a: AncDec32 = "3".parse().unwrap();
    let b: AncDec32 = "5".parse().unwrap();
    assert_eq!(a - b, "-2".parse().unwrap());
}

// ============ Multiplication ============
#[test]
fn test_mul_simple() {
    let a: AncDec32 = "2".parse().unwrap();
    let b: AncDec32 = "3".parse().unwrap();
    assert_eq!(a * b, "6".parse().unwrap());
}

#[test]
fn test_mul_decimal() {
    let a: AncDec32 = "1.5".parse().unwrap();
    let b: AncDec32 = "2".parse().unwrap();
    assert_eq!(a * b, "3".parse().unwrap());
}

#[test]
fn test_mul_negative() {
    let a: AncDec32 = "-3".parse().unwrap();
    let b: AncDec32 = "4".parse().unwrap();
    assert_eq!(a * b, "-12".parse().unwrap());
}

#[test]
fn test_mul_both_negative() {
    let a: AncDec32 = "-3".parse().unwrap();
    let b: AncDec32 = "-4".parse().unwrap();
    assert_eq!(a * b, "12".parse().unwrap());
}

#[test]
fn test_mul_large() {
    let a: AncDec32 = "1000".parse().unwrap();
    let b: AncDec32 = "1000".parse().unwrap();
    assert_eq!(a * b, "1000000".parse().unwrap());
}

// ============ Division ============
#[test]
fn test_div_simple() {
    let a: AncDec32 = "10".parse().unwrap();
    let b: AncDec32 = "2".parse().unwrap();
    assert_eq!(a / b, "5".parse().unwrap());
}

#[test]
fn test_div_decimal_result() {
    let a: AncDec32 = "1".parse().unwrap();
    let b: AncDec32 = "4".parse().unwrap();
    assert_eq!(a / b, "0.25".parse().unwrap());
}

#[test]
fn test_div_negative() {
    let a: AncDec32 = "-10".parse().unwrap();
    let b: AncDec32 = "4".parse().unwrap();
    assert_eq!(a / b, "-2.5".parse().unwrap());
}

// ============ Remainder ============
#[test]
fn test_rem_simple() {
    let a: AncDec32 = "10".parse().unwrap();
    let b: AncDec32 = "3".parse().unwrap();
    assert_eq!(a % b, "1".parse().unwrap());
}

#[test]
fn test_rem_decimal() {
    let a: AncDec32 = "5.5".parse().unwrap();
    let b: AncDec32 = "2".parse().unwrap();
    assert_eq!(a % b, "1.5".parse().unwrap());
}

// ============ Negation ============
#[test]
fn test_neg() {
    let a: AncDec32 = "5".parse().unwrap();
    assert_eq!(-a, "-5".parse().unwrap());
}

#[test]
fn test_neg_negative() {
    let a: AncDec32 = "-5".parse().unwrap();
    assert_eq!(-a, "5".parse().unwrap());
}

// ============ Assign Ops ============
#[test]
fn test_add_assign() {
    let mut a: AncDec32 = "5".parse().unwrap();
    a += "3".parse::<AncDec32>().unwrap();
    assert_eq!(a, "8".parse().unwrap());
}

#[test]
fn test_sub_assign() {
    let mut a: AncDec32 = "5".parse().unwrap();
    a -= "3".parse::<AncDec32>().unwrap();
    assert_eq!(a, "2".parse().unwrap());
}

#[test]
fn test_mul_assign() {
    let mut a: AncDec32 = "5".parse().unwrap();
    a *= "3".parse::<AncDec32>().unwrap();
    assert_eq!(a, "15".parse().unwrap());
}

#[test]
fn test_div_assign() {
    let mut a: AncDec32 = "15".parse().unwrap();
    a /= "3".parse::<AncDec32>().unwrap();
    assert_eq!(a, "5".parse().unwrap());
}

// ============ Reference Ops ============
#[test]
fn test_ref_add() {
    let a: AncDec32 = "1".parse().unwrap();
    let b: AncDec32 = "2".parse().unwrap();
    assert_eq!(&a + &b, "3".parse().unwrap());
    assert_eq!(a + &b, "3".parse().unwrap());
    assert_eq!(&a + b, "3".parse().unwrap());
}

// ============ From Integer ============
#[test]
fn test_from_u8() {
    let a = AncDec32::from(42u8);
    assert_eq!(a.int(), 42);
    assert!(!a.is_neg());
}

#[test]
fn test_from_u16() {
    let a = AncDec32::from(1000u16);
    assert_eq!(a.int(), 1000);
    assert!(!a.is_neg());
}

#[test]
fn test_from_u32() {
    let a = AncDec32::from(100000u32);
    assert_eq!(a.int(), 100000);
    assert!(!a.is_neg());
}

#[test]
fn test_from_i8() {
    let a = AncDec32::from(-42i8);
    assert_eq!(a.int(), 42);
    assert!(a.is_neg());
}

#[test]
fn test_from_i16() {
    let a = AncDec32::from(-1000i16);
    assert_eq!(a.int(), 1000);
    assert!(a.is_neg());
}

#[test]
fn test_from_i32() {
    let a = AncDec32::from(-100000i32);
    assert_eq!(a.int(), 100000);
    assert!(a.is_neg());
}

// ============ TryFrom Float ============
#[test]
fn test_try_from_f64() {
    let a = AncDec32::try_from(3.14f64).unwrap();
    assert_eq!(a.int(), 3);
    assert_eq!(a.frac(), 14);
    assert_eq!(a.scale(), 2);
}

#[test]
fn test_try_from_f64_nan() {
    assert!(AncDec32::try_from(f64::NAN).is_err());
}

#[test]
fn test_try_from_f64_infinity() {
    assert!(AncDec32::try_from(f64::INFINITY).is_err());
    assert!(AncDec32::try_from(f64::NEG_INFINITY).is_err());
}

// ============ Basic Methods ============
#[test]
fn test_abs() {
    let a: AncDec32 = "-5.5".parse().unwrap();
    assert_eq!(a.abs(), "5.5".parse().unwrap());
}

#[test]
fn test_signum() {
    assert_eq!("10".parse::<AncDec32>().unwrap().signum(), AncDec32::ONE);
    assert_eq!(
        "-10".parse::<AncDec32>().unwrap().signum(),
        "-1".parse().unwrap()
    );
    assert_eq!(AncDec32::ZERO.signum(), AncDec32::ZERO);
}

#[test]
fn test_is_zero() {
    assert!(AncDec32::ZERO.is_zero());
    assert!(!AncDec32::ONE.is_zero());
}

#[test]
fn test_is_positive() {
    assert!("5".parse::<AncDec32>().unwrap().is_positive());
    assert!(!"-5".parse::<AncDec32>().unwrap().is_positive());
    assert!(!AncDec32::ZERO.is_positive());
}

#[test]
fn test_is_negative() {
    assert!("-5".parse::<AncDec32>().unwrap().is_negative());
    assert!(!"5".parse::<AncDec32>().unwrap().is_negative());
    assert!(!AncDec32::ZERO.is_negative());
}

// ============ Min/Max/Clamp ============
#[test]
fn test_min() {
    let a: AncDec32 = "5".parse().unwrap();
    let b: AncDec32 = "3".parse().unwrap();
    assert_eq!(a.min(b), b);
}

#[test]
fn test_max() {
    let a: AncDec32 = "5".parse().unwrap();
    let b: AncDec32 = "3".parse().unwrap();
    assert_eq!(a.max(b), a);
}

#[test]
fn test_clamp() {
    let a: AncDec32 = "10".parse().unwrap();
    let min: AncDec32 = "0".parse().unwrap();
    let max: AncDec32 = "5".parse().unwrap();
    assert_eq!(a.clamp(min, max), max);
}

// ============ Rounding ============
#[test]
fn test_round_half_up() {
    let a: AncDec32 = "2.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfUp), "3".parse().unwrap());
}

#[test]
fn test_round_half_down() {
    let a: AncDec32 = "2.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfDown), "2".parse().unwrap());
}

#[test]
fn test_round_half_even() {
    let a: AncDec32 = "2.5".parse().unwrap();
    let b: AncDec32 = "3.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfEven), "2".parse().unwrap());
    assert_eq!(b.round(0, RoundMode::HalfEven), "4".parse().unwrap());
}

#[test]
fn test_round_truncate() {
    let a: AncDec32 = "2.9".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::Truncate), "2".parse().unwrap());
}

#[test]
fn test_round_floor() {
    let a: AncDec32 = "2.9".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::Floor), "2".parse().unwrap());
    let b: AncDec32 = "-2.1".parse().unwrap();
    assert_eq!(b.round(0, RoundMode::Floor), "-3".parse().unwrap());
}

#[test]
fn test_round_ceil() {
    let a: AncDec32 = "2.1".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::Ceil), "3".parse().unwrap());
    let b: AncDec32 = "-2.9".parse().unwrap();
    assert_eq!(b.round(0, RoundMode::Ceil), "-2".parse().unwrap());
}

#[test]
fn test_round_decimal_places() {
    let a: AncDec32 = "3.14159".parse().unwrap();
    assert_eq!(a.round(2, RoundMode::HalfUp), "3.14".parse().unwrap());
    assert_eq!(a.round(3, RoundMode::HalfUp), "3.142".parse().unwrap());
}

// ============ Floor/Ceil/Trunc/Fract ============
#[test]
fn test_floor() {
    assert_eq!(
        "2.9".parse::<AncDec32>().unwrap().floor(),
        "2".parse().unwrap()
    );
    assert_eq!(
        "-2.1".parse::<AncDec32>().unwrap().floor(),
        "-3".parse().unwrap()
    );
}

#[test]
fn test_ceil() {
    assert_eq!(
        "2.1".parse::<AncDec32>().unwrap().ceil(),
        "3".parse().unwrap()
    );
    assert_eq!(
        "-2.9".parse::<AncDec32>().unwrap().ceil(),
        "-2".parse().unwrap()
    );
}

#[test]
fn test_trunc() {
    assert_eq!(
        "2.9".parse::<AncDec32>().unwrap().trunc(),
        "2".parse().unwrap()
    );
    assert_eq!(
        "-2.9".parse::<AncDec32>().unwrap().trunc(),
        "-2".parse().unwrap()
    );
}

#[test]
fn test_fract() {
    let a: AncDec32 = "3.14".parse().unwrap();
    let f = a.fract();
    assert_eq!(f.int(), 0);
    assert_eq!(f.frac(), 14);
}

// ============ Power ============
#[test]
fn test_pow_positive() {
    let a: AncDec32 = "2".parse().unwrap();
    assert_eq!(a.pow(3), "8".parse().unwrap());
}

#[test]
fn test_pow_zero() {
    let a: AncDec32 = "5".parse().unwrap();
    assert_eq!(a.pow(0), AncDec32::ONE);
}

#[test]
fn test_pow_negative() {
    let a: AncDec32 = "2".parse().unwrap();
    assert_eq!(a.pow(-1), "0.5".parse().unwrap());
}

// ============ Square Root ============
#[test]
fn test_sqrt_four() {
    let four: AncDec32 = "4".parse().unwrap();
    let result = four.sqrt();
    assert_eq!(result.int(), 2);
    assert_eq!(result.to_string(), "2.00000000");
}

#[test]
fn test_sqrt_nine() {
    let nine: AncDec32 = "9".parse().unwrap();
    let result = nine.sqrt();
    assert_eq!(result.int(), 3);
    assert_eq!(result.to_string(), "3.00000000");
}

#[test]
fn test_sqrt_two_precision() {
    let two: AncDec32 = "2".parse().unwrap();
    let result = two.sqrt();
    // sqrt(2) ~= 1.41421356...
    assert_eq!(result.int(), 1);
    // Verify first several digits of fractional part
    let s = result.to_string();
    assert!(s.starts_with("1.41421356"));
}

#[test]
fn test_sqrt_fractional() {
    let val: AncDec32 = "0.25".parse().unwrap();
    let result = val.sqrt();
    assert_eq!(result.to_string(), "0.50000000");
}

#[test]
fn test_sqrt_zero() {
    assert_eq!(AncDec32::ZERO.sqrt(), AncDec32::ZERO);
}

#[test]
#[should_panic(expected = "square root of negative")]
fn test_sqrt_negative_panics() {
    let neg: AncDec32 = "-4".parse().unwrap();
    neg.sqrt();
}

// ============ Conversion ============
#[test]
fn test_to_f64() {
    let a: AncDec32 = "3.14".parse().unwrap();
    assert!((a.to_f64() - 3.14).abs() < 0.0001);
}

#[test]
fn test_to_i64() {
    let a: AncDec32 = "-42.99".parse().unwrap();
    assert_eq!(a.to_i64(), -42);
}

#[test]
fn test_to_i128() {
    let a: AncDec32 = "-1000000.5".parse().unwrap();
    assert_eq!(a.to_i128(), -1000000);
}

// ============ Default ============
#[test]
fn test_default() {
    let a: AncDec32 = Default::default();
    assert_eq!(a, AncDec32::ZERO);
}

// ============ Hash ============
#[test]
fn test_hash_equal_values() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a: AncDec32 = "123.45".parse().unwrap();
    let b: AncDec32 = "123.450".parse().unwrap();

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
        "1".parse::<AncDec32>().unwrap(),
        "2".parse::<AncDec32>().unwrap(),
        "3".parse::<AncDec32>().unwrap(),
    ];
    let sum: AncDec32 = v.into_iter().sum();
    assert_eq!(sum, "6".parse().unwrap());
}

#[test]
fn test_product() {
    let v = vec![
        "2".parse::<AncDec32>().unwrap(),
        "3".parse::<AncDec32>().unwrap(),
        "4".parse::<AncDec32>().unwrap(),
    ];
    let prod: AncDec32 = v.into_iter().product();
    assert_eq!(prod, "24".parse().unwrap());
}

// ============ Constants ============
#[test]
fn test_constants() {
    assert_eq!(AncDec32::ZERO.int(), 0);
    assert_eq!(AncDec32::ONE.int(), 1);
    assert_eq!(AncDec32::TWO.int(), 2);
    assert_eq!(AncDec32::TEN.int(), 10);
}

// ============ Edge Cases ============
#[test]
fn test_zero_operations() {
    let zero = AncDec32::ZERO;
    let one = AncDec32::ONE;

    assert_eq!(zero + one, one);
    assert_eq!(one - one, zero);
    assert_eq!(zero * one, zero);
}

#[test]
fn test_int_overflow_returns_error() {
    // u32::MAX = 4294967295; parsing a larger number should return Overflow error
    let result: Result<AncDec32, _> = "99999999999".parse();
    assert!(result.is_err());
}

#[test]
fn test_frac_truncates_at_9() {
    // Fractional part truncates at 9 digits
    let a: AncDec32 = "0.12345678901234".parse().unwrap();
    assert_eq!(a.scale(), 9);
    assert_eq!(a.frac(), 123456789);
}

#[test]
fn test_large_numbers() {
    let a: AncDec32 = "4294967295".parse().unwrap();
    assert_eq!(a.int(), u32::MAX);
    // Adding two large u32 numbers that don't overflow
    let c: AncDec32 = "1000000".parse().unwrap();
    let d: AncDec32 = "2000000".parse().unwrap();
    assert_eq!(c + d, "3000000".parse().unwrap());
}

#[test]
#[should_panic]
fn test_div_by_zero_panics() {
    let a: AncDec32 = "5".parse().unwrap();
    let _ = a / AncDec32::ZERO;
}

#[test]
fn test_negative_zero_equals_zero() {
    let neg_zero: AncDec32 = "-0".parse().unwrap();
    assert_eq!(neg_zero, AncDec32::ZERO);
}

// ============ Primitive Ops ============
#[test]
fn test_add_primitive() {
    let a: AncDec32 = "10".parse().unwrap();
    assert_eq!(a + 5i32, "15".parse().unwrap());
    assert_eq!(5i32 + a, "15".parse().unwrap());
}

#[test]
fn test_mul_primitive() {
    let a: AncDec32 = "10".parse().unwrap();
    assert_eq!(a * 2u32, "20".parse().unwrap());
    assert_eq!(2u32 * a, "20".parse().unwrap());
}

#[test]
fn test_div_with_f64() {
    let a: AncDec32 = "10".parse().unwrap();
    let b = AncDec32::try_from(4.0f64).unwrap();
    assert_eq!(a / b, "2.5".parse().unwrap());
}

// ============ Cross-type Ops with AncDec (dec64) ============
#[cfg(feature = "dec64")]
#[test]
fn test_cross_ops_with_ancdec() {
    let a: AncDec32 = "100".parse().unwrap();
    let b: ancdec::AncDec = "50.5".parse().unwrap();
    let result = a + b; // AncDec (u64)
    assert_eq!(result, "150.5".parse().unwrap());
}

// ============ From AncDec8 Widening ============
#[cfg(feature = "dec8")]
#[test]
fn test_from_ancdec8() {
    let a: ancdec::AncDec8 = "1.23".parse().unwrap();
    let b = AncDec32::from(a);
    assert_eq!(b.int(), 1);
    assert_eq!(b.frac(), 23);
    assert_eq!(b.scale(), 2);
}

// ============ Serde ============
#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let a: AncDec32 = "123.456".parse().unwrap();
    let json = serde_json::to_string(&a).unwrap();
    assert_eq!(json, "\"123.456\"");
    let b: AncDec32 = serde_json::from_str(&json).unwrap();
    assert_eq!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_negative() {
    let a: AncDec32 = "-99.99".parse().unwrap();
    let json = serde_json::to_string(&a).unwrap();
    let b: AncDec32 = serde_json::from_str(&json).unwrap();
    assert_eq!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_invalid() {
    let result: Result<AncDec32, _> = serde_json::from_str("\"abc\"");
    assert!(result.is_err());
}
