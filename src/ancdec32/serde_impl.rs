use super::AncDec32;
use core::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for AncDec32 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for AncDec32 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> de::Visitor<'de> for V {
            type Value = AncDec32;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("decimal string")
            }
            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                AncDec32::parse_str(s).map_err(|e| E::custom(e))
            }
        }
        deserializer.deserialize_str(V)
    }
}
