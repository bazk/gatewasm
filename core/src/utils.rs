pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type GenericResult<T> = std::result::Result<T, GenericError>;

pub mod serde_base64 {
    use serde::de;
    use serde::{Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<S>(data: &Vec<u8>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&base64::encode(data))
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Base64Visitor;

        impl<'de> de::Visitor<'de> for Base64Visitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing base64 encoded data")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                base64::decode(v).map_err(E::custom)
            }
        }

        d.deserialize_str(Base64Visitor)
    }
}
