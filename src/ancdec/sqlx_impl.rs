use crate::error::ParseError;
use crate::util::pow10;
use super::AncDec;
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef},
    Decode, Encode, Postgres, Type, TypeInfo,
};
use std::boxed::Box;

// PostgreSQL NUMERIC signs
const NUMERIC_POS: u16 = 0x0000;
const NUMERIC_NEG: u16 = 0x4000;

impl Type<Postgres> for AncDec {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("NUMERIC")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == Self::type_info() || ty.name() == "NUMERIC" || ty.name() == "DECIMAL"
    }
}

impl PgHasArrayType for AncDec {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_NUMERIC")
    }
}

impl Encode<'_, Postgres> for AncDec {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        if self.int == 0 && self.frac == 0 {
            // Zero: ndigits=0, weight=0, sign=0, dscale=0
            buf.extend_from_slice(&0u16.to_be_bytes()); // ndigits
            buf.extend_from_slice(&0i16.to_be_bytes()); // weight
            buf.extend_from_slice(&NUMERIC_POS.to_be_bytes()); // sign
            buf.extend_from_slice(&0u16.to_be_bytes()); // dscale
            return Ok(IsNull::No);
        }

        // Convert to base-10000 digits
        let mut digits: std::vec::Vec<i16> = std::vec::Vec::new();

        // Integer part
        let mut int_val = self.int;
        let mut int_digits: std::vec::Vec<i16> = std::vec::Vec::new();
        if int_val > 0 {
            while int_val > 0 {
                int_digits.push((int_val % 10000) as i16);
                int_val /= 10000;
            }
            int_digits.reverse();
        }

        // Fractional part - pad to scale, then group by 4
        let mut frac_val = self.frac;
        let mut frac_digits: std::vec::Vec<i16> = std::vec::Vec::new();
        if self.scale > 0 && frac_val > 0 {
            // Pad frac to multiple of 4 digits
            let padded_scale = self.scale.div_ceil(4) * 4;
            let pad_mult = pow10(padded_scale - self.scale);
            frac_val *= pad_mult;

            let num_frac_groups = (padded_scale / 4) as usize;
            for _ in 0..num_frac_groups {
                frac_digits.push((frac_val % 10000) as i16);
                frac_val /= 10000;
            }
            frac_digits.reverse();
        }

        // Combine digits
        digits.extend(int_digits.iter());
        digits.extend(frac_digits.iter());

        // Remove trailing zeros from frac part
        while !digits.is_empty() && digits.last() == Some(&0) {
            digits.pop();
        }

        // Weight = number of base-10000 digits before decimal - 1
        let weight = if self.int > 0 {
            int_digits.len() as i16 - 1
        } else {
            // Find first non-zero frac digit
            let mut w: i16 = -1;
            for (i, &d) in frac_digits.iter().enumerate() {
                if d != 0 {
                    w = -(i as i16 + 1);
                    break;
                }
            }
            w
        };

        // Remove leading zeros
        while !digits.is_empty() && digits.first() == Some(&0) {
            digits.remove(0);
        }

        let ndigits = digits.len() as u16;
        let sign = if self.neg { NUMERIC_NEG } else { NUMERIC_POS };
        let dscale = self.scale as u16;

        buf.extend_from_slice(&ndigits.to_be_bytes());
        buf.extend_from_slice(&weight.to_be_bytes());
        buf.extend_from_slice(&sign.to_be_bytes());
        buf.extend_from_slice(&dscale.to_be_bytes());
        for d in &digits {
            buf.extend_from_slice(&d.to_be_bytes());
        }

        Ok(IsNull::No)
    }

    fn size_hint(&self) -> usize {
        8 + 20 // header + max digits
    }
}

impl Decode<'_, Postgres> for AncDec {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        match value.format() {
            PgValueFormat::Text => {
                let s = <&str as Decode<Postgres>>::decode(value)?;
                s.parse::<AncDec>().map_err(|e| Box::new(e) as BoxDynError)
            }
            PgValueFormat::Binary => {
                let bytes = value.as_bytes()?;
                if bytes.len() < 8 {
                    return Err("invalid numeric".into());
                }

                let ndigits = u16::from_be_bytes([bytes[0], bytes[1]]) as usize;
                let weight = i16::from_be_bytes([bytes[2], bytes[3]]);
                let sign = u16::from_be_bytes([bytes[4], bytes[5]]);
                let dscale = u16::from_be_bytes([bytes[6], bytes[7]]);

                if ndigits == 0 {
                    return Ok(AncDec::ZERO);
                }

                let neg = sign == NUMERIC_NEG;

                // Read base-10000 digits
                let mut digits: std::vec::Vec<i16> = std::vec::Vec::with_capacity(ndigits);
                for i in 0..ndigits {
                    let offset = 8 + i * 2;
                    let d = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
                    digits.push(d);
                }

                // Convert to decimal string and parse
                let mut s = std::string::String::new();
                if neg {
                    s.push('-');
                }

                // Integer part
                let int_digits = (weight + 1).max(0) as usize;
                if int_digits == 0 {
                    s.push('0');
                } else {
                    for i in 0..int_digits {
                        let d = if i < digits.len() { digits[i] } else { 0 };
                        if i == 0 {
                            s.push_str(&std::format!("{}", d));
                        } else {
                            s.push_str(&std::format!("{:04}", d));
                        }
                    }
                }

                // Fractional part
                if dscale > 0 {
                    s.push('.');
                    let frac_start = int_digits;
                    let mut frac_str = std::string::String::new();

                    // Leading zeros for negative weight
                    if weight < -1 {
                        for _ in 0..(-(weight + 1)) {
                            frac_str.push_str("0000");
                        }
                    }

                    for d in digits.iter().skip(frac_start) {
                        frac_str.push_str(&std::format!("{:04}", d));
                    }

                    // Truncate to dscale
                    let frac_chars: std::string::String =
                        frac_str.chars().take(dscale as usize).collect();
                    s.push_str(&frac_chars);
                }

                s.parse::<AncDec>().map_err(|e| Box::new(e) as BoxDynError)
            }
        }
    }
}

impl std::error::Error for ParseError {}
