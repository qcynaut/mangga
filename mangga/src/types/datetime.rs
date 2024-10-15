use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// DateTime type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime(bson::DateTime);

impl Default for DateTime {
    fn default() -> Self {
        Self::now()
    }
}

impl DateTime {
    /// Get now
    pub fn now() -> Self {
        Self(bson::DateTime::from(Utc::now()))
    }
}

impl Deref for DateTime {
    type Target = bson::DateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        bson::DateTime::deserialize(deserializer).map(Self)
    }
}

impl From<chrono::DateTime<Utc>> for DateTime {
    fn from(dt: chrono::DateTime<Utc>) -> Self {
        Self(bson::DateTime::from(dt))
    }
}

impl From<DateTime> for chrono::DateTime<Utc> {
    fn from(dt: DateTime) -> Self {
        chrono::DateTime::from(dt.0)
    }
}

#[async_graphql::Scalar]
impl async_graphql::ScalarType for DateTime {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(s) = &value {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&s) {
                return Ok(Self(bson::DateTime::from_chrono(dt)));
            }
        }

        Err(async_graphql::InputValueError::expected_type(value))
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(chrono::DateTime::from(self.0).to_rfc3339())
    }
}
