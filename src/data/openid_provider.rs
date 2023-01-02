use std::fmt::Display;

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpenIDProvider {
    Google,
}

impl OpenIDProvider {
    pub fn parse(str: &str) -> Result<Self, ()> {
        match str {
            "google" => Ok(Self::Google),
            _ => Err(()),
        }
    }
}

impl Display for OpenIDProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Google => "google",
        };
        write!(f, "{str}")
    }
}

struct OpenIDProviderVisitor;

impl<'de> Visitor<'de> for OpenIDProviderVisitor {
    type Value = OpenIDProvider;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let result = OpenIDProvider::parse(v);
        match result {
            Ok(value) => Ok(value),
            Err(_) => Err(E::custom("invalid value")),
        }
    }
}

impl<'de> Deserialize<'de> for OpenIDProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(OpenIDProviderVisitor)
    }
}

impl Serialize for OpenIDProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
