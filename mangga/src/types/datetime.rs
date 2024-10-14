use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// DateTime type
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime(chrono::DateTime<Utc>);

impl DateTime {
    /// Get now
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl From<chrono::DateTime<Utc>> for DateTime {
    fn from(dt: chrono::DateTime<Utc>) -> Self {
        Self(dt)
    }
}

impl Deref for DateTime {
    type Target = chrono::DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<chrono::DateTime<Utc>> for DateTime {
    fn as_ref(&self) -> &chrono::DateTime<Utc> {
        &self.0
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_rfc3339())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = chrono::DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
        Ok(Self(dt.to_utc()))
    }
}

#[async_graphql::Scalar]
impl async_graphql::ScalarType for DateTime {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(s) = &value {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&s) {
                return Ok(Self(dt.to_utc()));
            }
        }

        Err(async_graphql::InputValueError::expected_type(value))
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.to_rfc3339())
    }
}
