use async_graphql::ScalarType;
use bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};
use std::{ops::{Deref, DerefMut}, str::FromStr};

/// Type alias for id
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ID(ObjectId);

impl ID {
    /// Create a new leading zero id
    pub fn zeros() -> Self {
        Self(ObjectId::from([0; 12]))
    }
}

impl std::fmt::Debug for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ID {
    type Err = bson::oid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ObjectId::parse_str(s) {
            Ok(id) => Ok(ID(id)),
            Err(e) => Err(e),
        }
    }
}

impl Deref for ID {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

impl From<ID> for ObjectId {
    fn from(id: ID) -> Self {
        id.0
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
        let id = ObjectId::deserialize(deserializer)?;
        Ok(ID(id))
    }
}

/// IsID
///
/// Represents if the type is an id
pub trait IsID {}

impl IsID for ID {}
impl IsID for ObjectId {}

/// Function to check if the type is an id
pub const fn is_id<T: IsID>() {}

#[async_graphql::Scalar]
impl ScalarType for ID {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(s) = &value {
            if let Ok(id) = ID::from_str(s) {
                return Ok(id);
            }
        }

        Err(async_graphql::InputValueError::expected_type(value))
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.to_string())
    }
}