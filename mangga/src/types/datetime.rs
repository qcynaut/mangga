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

#[async_graphql::Scalar]
impl async_graphql::ScalarType for DateTime {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(s) = &value {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&s) {
                return Ok(Self(bson::DateTime::from(dt)));
            }
        }

        Err(async_graphql::InputValueError::expected_type(value))
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(chrono::DateTime::from(self.0).to_rfc3339())
    }
}
