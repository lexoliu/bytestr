use crate::ByteStr;
use alloc::borrow::ToOwned;
use core::fmt;
use serde::{Deserialize, Serialize, de, de::Visitor};

impl Serialize for ByteStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

struct ByteStrVisitor;

impl Visitor<'_> for ByteStrVisitor {
    type Value = ByteStr;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Self::Value::from(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Self::Value::from_utf8(v.to_owned())
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Bytes(v), &self))
    }
}

impl<'de> Deserialize<'de> for ByteStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ByteStrVisitor)
    }
}
