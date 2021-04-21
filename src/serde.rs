use serde::{
    de::{Error, Visitor},
    Deserialize, Serialize,
};
#[cfg(test)]
use serde_test::{assert_tokens, Token};

use crate::id::Flake;

impl Serialize for Flake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Flake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(FlakeVisitor)
    }
}

struct FlakeVisitor;

impl<'de> Visitor<'de> for FlakeVisitor {
    type Value = Flake;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a base64 Flake ID")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let decoded_bytes = data_encoding::BASE64.decode(v.as_bytes()).unwrap();
        let mut bytes = [0u8; 16];
        for (i, byte) in decoded_bytes.iter().enumerate() {
            bytes[i] = *byte;
        }
        let value = u128::from_be_bytes(bytes);
        Ok(Flake::new(value))
    }
}

#[test]
fn test_serde() {
    let id = Flake::new(29866156537351941961353716432896);
    assert_tokens(&id, &[Token::String("AAABePbBqL900Cue9CYAAA==")]);
}
