use crate::ByteStr;
use serde::{de, de::Visitor, Deserialize, Serialize};
impl Serialize for ByteStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.as_bytes())
    }
}

struct ByteStrVisitor;

impl<'de> Visitor<'de> for ByteStrVisitor {
    type Value = ByteStr;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Self::Value::from(v))
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

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        String::from_utf8(v).map(|v| v.into()).map_err(|error| {
            de::Error::invalid_value(de::Unexpected::Bytes(&error.into_bytes()), &self)
        })
    }
}

impl<'de> Deserialize<'de> for ByteStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(ByteStrVisitor)
    }
}
