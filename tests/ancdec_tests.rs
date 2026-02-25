// tests/ancdec_tests.rs
#![cfg(feature = "dec64")]

use ancdec::{AncDec, RoundMode};

// ============ Parsing ============
#[test]
fn test_parse_integer() {
    let a: AncDec = "123".parse().unwrap();
    assert_eq!(a.int, 123);
    assert_eq!(a.frac, 0);
    assert_eq!(a.scale, 0);
    assert!(!a.neg);
}

#[test]
fn test_parse_decimal() {
    let a: AncDec = "123.456".parse().unwrap();
    assert_eq!(a.int, 123);
    assert_eq!(a.frac, 456);
    assert_eq!(a.scale, 3);
    assert!(!a.neg);
}

#[test]
fn test_parse_negative() {
    let a: AncDec = "-99.05".parse().unwrap();
    assert_eq!(a.int, 99);
    assert_eq!(a.frac, 5);
    assert_eq!(a.scale, 2);
    assert!(a.neg);
}

#[test]
fn test_parse_leading_zero_frac() {
    let a: AncDec = "1.005".parse().unwrap();
    assert_eq!(a.int, 1);
    assert_eq!(a.frac, 5);
    assert_eq!(a.scale, 3);
}

#[test]
fn test_parse_invalid() {
    assert!("".parse::<AncDec>().is_err());
    assert!("-".parse::<AncDec>().is_err());
    assert!("abc".parse::<AncDec>().is_err());
    assert!("12.34.56".parse::<AncDec>().is_err());
}

// ============ Display ============
#[test]
fn test_display_integer() {
    let a: AncDec = "42".parse().unwrap();
    assert_eq!(format!("{}", a), "42");
}

#[test]
fn test_display_decimal() {
    let a: AncDec = "123.456".parse().unwrap();
    assert_eq!(format!("{}", a), "123.456");
}

#[test]
fn test_display_negative() {
    let a: AncDec = "-99.05".parse().unwrap();
    assert_eq!(format!("{}", a), "-99.05");
}

#[test]
fn test_display_precision() {
    let a: AncDec = "123.456".parse().unwrap();
    assert_eq!(format!("{:.2}", a), "123.45");
    assert_eq!(format!("{:.5}", a), "123.45600");
    assert_eq!(format!("{:.0}", a), "123");
}

// ============ Comparison ============
#[test]
fn test_eq() {
    let a: AncDec = "123.45".parse().unwrap();
    let b: AncDec = "123.450".parse().unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_ord() {
    let a: AncDec = "100".parse().unwrap();
    let b: AncDec = "99.99".parse().unwrap();
    assert!(a > b);
}

#[test]
fn test_ord_negative() {
    let a: AncDec = "-10".parse().unwrap();
    let b: AncDec = "-20".parse().unwrap();
    assert!(a > b);
}

#[test]
fn test_ord_mixed_sign() {
    let a: AncDec = "1".parse().unwrap();
    let b: AncDec = "-1000".parse().unwrap();
    assert!(a > b);
}

// ============ Addition ============
#[test]
fn test_add_simple() {
    let a: AncDec = "1.5".parse().unwrap();
    let b: AncDec = "2.5".parse().unwrap();
    assert_eq!(a + b, "4".parse().unwrap());
}

#[test]
fn test_add_different_scale() {
    let a: AncDec = "1.1".parse().unwrap();
    let b: AncDec = "2.22".parse().unwrap();
    assert_eq!(a + b, "3.32".parse().unwrap());
}

#[test]
fn test_add_with_carry() {
    let a: AncDec = "0.9".parse().unwrap();
    let b: AncDec = "0.2".parse().unwrap();
    assert_eq!(a + b, "1.1".parse().unwrap());
}

#[test]
fn test_add_negative() {
    let a: AncDec = "10".parse().unwrap();
    let b: AncDec = "-3".parse().unwrap();
    assert_eq!(a + b, "7".parse().unwrap());
}

// ============ Subtraction ============
#[test]
fn test_sub_simple() {
    let a: AncDec = "5.5".parse().unwrap();
    let b: AncDec = "2.3".parse().unwrap();
    assert_eq!(a - b, "3.2".parse().unwrap());
}

#[test]
fn test_sub_with_borrow() {
    let a: AncDec = "1.0".parse().unwrap();
    let b: AncDec = "0.3".parse().unwrap();
    assert_eq!(a - b, "0.7".parse().unwrap());
}

#[test]
fn test_sub_result_negative() {
    let a: AncDec = "3".parse().unwrap();
    let b: AncDec = "5".parse().unwrap();
    assert_eq!(a - b, "-2".parse().unwrap());
}

// ============ Multiplication ============
#[test]
fn test_mul_simple() {
    let a: AncDec = "2".parse().unwrap();
    let b: AncDec = "3".parse().unwrap();
    assert_eq!(a * b, "6".parse().unwrap());
}

#[test]
fn test_mul_decimal() {
    let a: AncDec = "1.5".parse().unwrap();
    let b: AncDec = "2".parse().unwrap();
    assert_eq!(a * b, "3".parse().unwrap());
}

#[test]
fn test_mul_negative() {
    let a: AncDec = "-3".parse().unwrap();
    let b: AncDec = "4".parse().unwrap();
    assert_eq!(a * b, "-12".parse().unwrap());
}

#[test]
fn test_mul_both_negative() {
    let a: AncDec = "-3".parse().unwrap();
    let b: AncDec = "-4".parse().unwrap();
    assert_eq!(a * b, "12".parse().unwrap());
}

// ============ Division ============
#[test]
fn test_div_simple() {
    let a: AncDec = "10".parse().unwrap();
    let b: AncDec = "2".parse().unwrap();
    assert_eq!(a / b, "5".parse().unwrap());
}

#[test]
fn test_div_decimal_result() {
    let a: AncDec = "1".parse().unwrap();
    let b: AncDec = "4".parse().unwrap();
    assert_eq!(a / b, "0.25".parse().unwrap());
}

#[test]
fn test_div_negative() {
    let a: AncDec = "-10".parse().unwrap();
    let b: AncDec = "4".parse().unwrap();
    assert_eq!(a / b, "-2.5".parse().unwrap());
}

// ============ Remainder ============
#[test]
fn test_rem_simple() {
    let a: AncDec = "10".parse().unwrap();
    let b: AncDec = "3".parse().unwrap();
    assert_eq!(a % b, "1".parse().unwrap());
}

#[test]
fn test_rem_decimal() {
    let a: AncDec = "5.5".parse().unwrap();
    let b: AncDec = "2".parse().unwrap();
    assert_eq!(a % b, "1.5".parse().unwrap());
}

// ============ Negation ============
#[test]
fn test_neg() {
    let a: AncDec = "5".parse().unwrap();
    assert_eq!(-a, "-5".parse().unwrap());
}

#[test]
fn test_neg_negative() {
    let a: AncDec = "-5".parse().unwrap();
    assert_eq!(-a, "5".parse().unwrap());
}

// ============ Assign Ops ============
#[test]
fn test_add_assign() {
    let mut a: AncDec = "5".parse().unwrap();
    a += "3".parse::<AncDec>().unwrap();
    assert_eq!(a, "8".parse().unwrap());
}

#[test]
fn test_sub_assign() {
    let mut a: AncDec = "5".parse().unwrap();
    a -= "3".parse::<AncDec>().unwrap();
    assert_eq!(a, "2".parse().unwrap());
}

#[test]
fn test_mul_assign() {
    let mut a: AncDec = "5".parse().unwrap();
    a *= "3".parse::<AncDec>().unwrap();
    assert_eq!(a, "15".parse().unwrap());
}

#[test]
fn test_div_assign() {
    let mut a: AncDec = "15".parse().unwrap();
    a /= "3".parse::<AncDec>().unwrap();
    assert_eq!(a, "5".parse().unwrap());
}

// ============ Reference Ops ============
#[test]
fn test_ref_add() {
    let a: AncDec = "1".parse().unwrap();
    let b: AncDec = "2".parse().unwrap();
    assert_eq!(&a + &b, "3".parse().unwrap());
    assert_eq!(a + &b, "3".parse().unwrap());
    assert_eq!(&a + b, "3".parse().unwrap());
}

// ============ Primitive Ops ============
#[test]
fn test_add_primitive() {
    let a: AncDec = "10".parse().unwrap();
    assert_eq!(a + 5i32, "15".parse().unwrap());
    assert_eq!(5i32 + a, "15".parse().unwrap());
}

#[test]
fn test_mul_primitive() {
    let a: AncDec = "10".parse().unwrap();
    assert_eq!(a * 2u64, "20".parse().unwrap());
    assert_eq!(2u64 * a, "20".parse().unwrap());
}

#[test]
fn test_div_with_f64() {
    let a: AncDec = "10".parse().unwrap();
    let b = AncDec::try_from(4.0f64).unwrap();
    assert_eq!(a / b, "2.5".parse().unwrap());
}

// ============ From Integer ============
#[test]
fn test_from_i32() {
    let a = AncDec::from(-42i32);
    assert_eq!(a.int, 42);
    assert!(a.neg);
}

#[test]
fn test_from_u64() {
    let a = AncDec::from(100u64);
    assert_eq!(a.int, 100);
    assert!(!a.neg);
}

// ============ From Float ============

// 변경 후
#[test]
fn test_try_from_f64() {
    let a = AncDec::try_from(3.14f64).unwrap();
    assert_eq!(a.int, 3);
    assert_eq!(a.frac, 14);
    assert_eq!(a.scale, 2);
}

#[test]
fn test_try_from_f64_nan() {
    assert!(AncDec::try_from(f64::NAN).is_err());
}

#[test]
fn test_try_from_f64_infinity() {
    assert!(AncDec::try_from(f64::INFINITY).is_err());
    assert!(AncDec::try_from(f64::NEG_INFINITY).is_err());
}

// ============ Basic Methods ============
#[test]
fn test_abs() {
    let a: AncDec = "-5.5".parse().unwrap();
    assert_eq!(a.abs(), "5.5".parse().unwrap());
}

#[test]
fn test_signum() {
    assert_eq!("10".parse::<AncDec>().unwrap().signum(), AncDec::ONE);
    assert_eq!(
        "-10".parse::<AncDec>().unwrap().signum(),
        "-1".parse().unwrap()
    );
    assert_eq!(AncDec::ZERO.signum(), AncDec::ZERO);
}

#[test]
fn test_is_zero() {
    assert!(AncDec::ZERO.is_zero());
    assert!(!AncDec::ONE.is_zero());
}

#[test]
fn test_is_positive() {
    assert!("5".parse::<AncDec>().unwrap().is_positive());
    assert!(!"-5".parse::<AncDec>().unwrap().is_positive());
    assert!(!AncDec::ZERO.is_positive());
}

#[test]
fn test_is_negative() {
    assert!("-5".parse::<AncDec>().unwrap().is_negative());
    assert!(!"5".parse::<AncDec>().unwrap().is_negative());
    assert!(!AncDec::ZERO.is_negative());
}

// ============ Min/Max/Clamp ============
#[test]
fn test_min() {
    let a: AncDec = "5".parse().unwrap();
    let b: AncDec = "3".parse().unwrap();
    assert_eq!(a.min(b), b);
}

#[test]
fn test_max() {
    let a: AncDec = "5".parse().unwrap();
    let b: AncDec = "3".parse().unwrap();
    assert_eq!(a.max(b), a);
}

#[test]
fn test_clamp() {
    let a: AncDec = "10".parse().unwrap();
    let min: AncDec = "0".parse().unwrap();
    let max: AncDec = "5".parse().unwrap();
    assert_eq!(a.clamp(min, max), max);
}

// ============ Rounding ============
#[test]
fn test_round_half_up() {
    let a: AncDec = "2.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfUp), "3".parse().unwrap());
}

#[test]
fn test_round_half_down() {
    let a: AncDec = "2.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfDown), "2".parse().unwrap());
}

#[test]
fn test_round_half_even() {
    let a: AncDec = "2.5".parse().unwrap();
    let b: AncDec = "3.5".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::HalfEven), "2".parse().unwrap());
    assert_eq!(b.round(0, RoundMode::HalfEven), "4".parse().unwrap());
}

#[test]
fn test_round_truncate() {
    let a: AncDec = "2.9".parse().unwrap();
    assert_eq!(a.round(0, RoundMode::Truncate), "2".parse().unwrap());
}

#[test]
fn test_round_decimal_places() {
    let a: AncDec = "3.14159".parse().unwrap();
    assert_eq!(a.round(2, RoundMode::HalfUp), "3.14".parse().unwrap());
    assert_eq!(a.round(3, RoundMode::HalfUp), "3.142".parse().unwrap());
}

// ============ Floor/Ceil/Trunc/Fract ============
#[test]
fn test_floor() {
    assert_eq!(
        "2.9".parse::<AncDec>().unwrap().floor(),
        "2".parse().unwrap()
    );
    assert_eq!(
        "-2.1".parse::<AncDec>().unwrap().floor(),
        "-3".parse().unwrap()
    );
}

#[test]
fn test_ceil() {
    assert_eq!(
        "2.1".parse::<AncDec>().unwrap().ceil(),
        "3".parse().unwrap()
    );
    assert_eq!(
        "-2.9".parse::<AncDec>().unwrap().ceil(),
        "-2".parse().unwrap()
    );
}

#[test]
fn test_trunc() {
    assert_eq!(
        "2.9".parse::<AncDec>().unwrap().trunc(),
        "2".parse().unwrap()
    );
    assert_eq!(
        "-2.9".parse::<AncDec>().unwrap().trunc(),
        "-2".parse().unwrap()
    );
}

#[test]
fn test_fract() {
    let a: AncDec = "3.14".parse().unwrap();
    let f = a.fract();
    assert_eq!(f.int, 0);
    assert_eq!(f.frac, 14);
}

// ============ Power ============
#[test]
fn test_pow_positive() {
    let a: AncDec = "2".parse().unwrap();
    assert_eq!(a.pow(3), "8".parse().unwrap());
}

#[test]
fn test_pow_zero() {
    let a: AncDec = "5".parse().unwrap();
    assert_eq!(a.pow(0), AncDec::ONE);
}

#[test]
fn test_pow_negative() {
    let a: AncDec = "2".parse().unwrap();
    assert_eq!(a.pow(-1), "0.5".parse().unwrap());
}

// ============ Square Root ============
#[test]
fn test_sqrt_perfect_squares() {
    assert_eq!(AncDec::ZERO.sqrt(), AncDec::ZERO);
    let one: AncDec = "1".parse().unwrap();
    assert_eq!(one.sqrt().to_string(), "1.000000000000000000");
    let four: AncDec = "4".parse().unwrap();
    assert_eq!(four.sqrt().to_string(), "2.000000000000000000");
    let nine: AncDec = "9".parse().unwrap();
    assert_eq!(nine.sqrt().to_string(), "3.000000000000000000");
    let hundred: AncDec = "100".parse().unwrap();
    assert_eq!(hundred.sqrt().to_string(), "10.000000000000000000");
}

#[test]
fn test_sqrt_two() {
    let two: AncDec = "2".parse().unwrap();
    let result = two.sqrt();
    // sqrt(2) = 1.41421356237309504880...
    // floor(sqrt(2) * 10^18) = 1414213562373095048
    assert_eq!(result.to_string(), "1.414213562373095048");
}

#[test]
fn test_sqrt_fractional() {
    let val: AncDec = "0.25".parse().unwrap();
    assert_eq!(val.sqrt().to_string(), "0.500000000000000000");
    let val: AncDec = "0.01".parse().unwrap();
    assert_eq!(val.sqrt().to_string(), "0.100000000000000000");
}

#[test]
fn test_sqrt_roundtrip() {
    // floor(sqrt(val))^2 <= val (guaranteed by isqrt)
    // mul truncates downward, so sq <= mathematical result <= val
    let val: AncDec = "7".parse().unwrap();
    let root = val.sqrt();
    let sq = root * root;
    assert!(sq <= val);
}

#[test]
fn test_sqrt_large() {
    let val: AncDec = "1000000000".parse().unwrap();
    let root = val.sqrt();
    assert_eq!(root.int, 31622);
    let sq = root * root;
    assert!(sq <= val);
}

#[test]
#[should_panic(expected = "square root of negative")]
fn test_sqrt_negative_panics() {
    let neg: AncDec = "-4".parse().unwrap();
    neg.sqrt();
}

// ============ Conversion ============
#[test]
fn test_to_f64() {
    let a: AncDec = "3.14".parse().unwrap();
    assert!((a.to_f64() - 3.14).abs() < 0.0001);
}

#[test]
fn test_to_i64() {
    let a: AncDec = "-42.99".parse().unwrap();
    assert_eq!(a.to_i64(), -42);
}

#[test]
fn test_to_i128() {
    let a: AncDec = "-1000000000000000000.5".parse().unwrap();
    assert_eq!(a.to_i128(), -1000000000000000000);
}

// ============ Default ============
#[test]
fn test_default() {
    let a: AncDec = Default::default();
    assert_eq!(a, AncDec::ZERO);
}

// ============ Hash ============
#[test]
fn test_hash_equal_values() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a: AncDec = "123.45".parse().unwrap();
    let b: AncDec = "123.450".parse().unwrap();

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
        "1".parse::<AncDec>().unwrap(),
        "2".parse::<AncDec>().unwrap(),
        "3".parse::<AncDec>().unwrap(),
    ];
    let sum: AncDec = v.into_iter().sum();
    assert_eq!(sum, "6".parse().unwrap());
}

#[test]
fn test_product() {
    let v = vec![
        "2".parse::<AncDec>().unwrap(),
        "3".parse::<AncDec>().unwrap(),
        "4".parse::<AncDec>().unwrap(),
    ];
    let prod: AncDec = v.into_iter().product();
    assert_eq!(prod, "24".parse().unwrap());
}

// ============ Serde (only with feature) ============
#[cfg(feature = "serde")]
#[test]
fn test_serde_roundtrip() {
    let a: AncDec = "123.456".parse().unwrap();
    let json = serde_json::to_string(&a).unwrap();
    assert_eq!(json, "\"123.456\"");

    let b: AncDec = serde_json::from_str(&json).unwrap();
    assert_eq!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_negative() {
    let a: AncDec = "-99.99".parse().unwrap();
    let json = serde_json::to_string(&a).unwrap();
    let b: AncDec = serde_json::from_str(&json).unwrap();
    assert_eq!(a, b);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_invalid() {
    let result: Result<AncDec, _> = serde_json::from_str("\"abc\"");
    assert!(result.is_err());
}

// ============ Constants ============
#[test]
fn test_constants() {
    assert_eq!(AncDec::ZERO.int, 0);
    assert_eq!(AncDec::ONE.int, 1);
    assert_eq!(AncDec::TWO.int, 2);
    assert_eq!(AncDec::TEN.int, 10);
}

// ============ Edge Cases ============
#[test]
fn test_zero_operations() {
    let zero = AncDec::ZERO;
    let one = AncDec::ONE;

    assert_eq!(zero + one, one);
    assert_eq!(one - one, zero);
    assert_eq!(zero * one, zero);
}

#[test]
fn test_large_numbers() {
    let a: AncDec = "9999999999999999999".parse().unwrap();
    let b: AncDec = "1".parse().unwrap();
    let result = a + b;
    assert_eq!(result, "10000000000000000000".parse().unwrap());
}

#[test]
fn test_high_precision() {
    let a: AncDec = "0.123456789012345678".parse().unwrap();
    assert_eq!(a.scale, 18);
}

#[test]
#[should_panic]
fn test_div_by_zero_panics() {
    let a: AncDec = "5".parse().unwrap();
    let _ = a / AncDec::ZERO;
}

#[test]
fn test_negative_zero_equals_zero() {
    let neg_zero: AncDec = "-0".parse().unwrap();
    assert_eq!(neg_zero, AncDec::ZERO);
}

// ============ Overflow/Truncation ============
#[test]
fn test_int_overflow_returns_error() {
    // u64::MAX = 18446744073709551615 (20 digits); parsing a larger number should return Overflow error
    let result: Result<AncDec, _> = "99999999999999999999".parse();
    assert!(result.is_err());
}

#[test]
fn test_frac_truncates_at_19() {
    let a: AncDec = "0.12345678901234567890123".parse().unwrap();
    assert_eq!(a.scale, 19);
    assert_eq!(a.frac, 1234567890123456789);
}

#[cfg(feature = "sqlx")]
mod tests {
    use ancdec::AncDec;
    use sqlx::{PgPool, Row};

    async fn setup_pool() -> PgPool {
        let url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://test:test@localhost:5432/ancdec_test".to_string());
        PgPool::connect(&url).await.expect("Failed to connect")
    }

    #[tokio::test]
    async fn test_insert_select() {
        let pool = setup_pool().await;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS test_ancdec (id SERIAL PRIMARY KEY, value NUMERIC)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let original: AncDec = "12345.6789012345678".parse().unwrap();
        sqlx::query("INSERT INTO test_ancdec (value) VALUES ($1)")
            .bind(&original)
            .execute(&pool)
            .await
            .unwrap();

        let row = sqlx::query("SELECT value FROM test_ancdec ORDER BY id DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        let retrieved: AncDec = row.get("value");
        assert_eq!(original, retrieved);

        sqlx::query("DROP TABLE test_ancdec")
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_negative() {
        let pool = setup_pool().await;

        sqlx::query("CREATE TABLE IF NOT EXISTS test_neg (id SERIAL PRIMARY KEY, value NUMERIC)")
            .execute(&pool)
            .await
            .unwrap();

        let original: AncDec = "-9999.123456789".parse().unwrap();
        sqlx::query("INSERT INTO test_neg (value) VALUES ($1)")
            .bind(&original)
            .execute(&pool)
            .await
            .unwrap();

        let row = sqlx::query("SELECT value FROM test_neg ORDER BY id DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        let retrieved: AncDec = row.get("value");
        assert_eq!(original, retrieved);

        sqlx::query("DROP TABLE test_neg")
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_zero() {
        let pool = setup_pool().await;

        sqlx::query("CREATE TABLE IF NOT EXISTS test_zero (id SERIAL PRIMARY KEY, value NUMERIC)")
            .execute(&pool)
            .await
            .unwrap();

        let original = AncDec::ZERO;
        sqlx::query("INSERT INTO test_zero (value) VALUES ($1)")
            .bind(&original)
            .execute(&pool)
            .await
            .unwrap();

        let row = sqlx::query("SELECT value FROM test_zero LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        let retrieved: AncDec = row.get("value");
        assert!(retrieved.is_zero());

        sqlx::query("DROP TABLE test_zero")
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_large_precision() {
        let pool = setup_pool().await;

        sqlx::query("CREATE TABLE IF NOT EXISTS test_prec (id SERIAL PRIMARY KEY, value NUMERIC)")
            .execute(&pool)
            .await
            .unwrap();

        let original: AncDec = "1234567890123456789.1234567890123456789".parse().unwrap();
        sqlx::query("INSERT INTO test_prec (value) VALUES ($1)")
            .bind(&original)
            .execute(&pool)
            .await
            .unwrap();

        let row = sqlx::query("SELECT value FROM test_prec LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();

        let retrieved: AncDec = row.get("value");
        assert_eq!(original, retrieved);

        sqlx::query("DROP TABLE test_prec")
            .execute(&pool)
            .await
            .unwrap();
    }
}
