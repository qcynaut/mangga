use bson::{oid::ObjectId, Bson};
pub use datetime::DateTime;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

mod datetime;

/// ID type
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ID(ObjectId);

impl Debug for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_hex())
    }
}

impl Deref for ID {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<ObjectId> for ID {
    fn as_ref(&self) -> &ObjectId {
        &self.0
    }
}

impl From<ObjectId> for ID {
    fn from(id: ObjectId) -> Self {
        Self(id)
    }
}

impl From<ID> for Bson {
    fn from(value: ID) -> Self {
        Bson::ObjectId(value.0)
    }
}

impl Serialize for ID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        ObjectId::deserialize(deserializer).map(Self)
    }
}

#[async_graphql::Scalar]
impl async_graphql::ScalarType for ID {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(s) = &value {
            if let Ok(id) = ObjectId::parse_str(s) {
                return Ok(Self(id));
            }
        }

        Err(async_graphql::InputValueError::expected_type(value))
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.to_string())
    }
}
