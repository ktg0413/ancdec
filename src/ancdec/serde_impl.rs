use super::AncDec;
use core::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Serialize as string "123.45"
impl Serialize for AncDec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// Deserialize from string
impl<'de> Deserialize<'de> for AncDec {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> de::Visitor<'de> for V {
            type Value = AncDec;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("decimal string")
            }
            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                AncDec::parse_str(s).map_err(|e| E::custom(e))
            }
        }
        deserializer.deserialize_str(V)
    }
}
