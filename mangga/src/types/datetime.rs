use async_graphql::ScalarType;
use chrono::{Days, FixedOffset, Months, TimeDelta};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

/// DateTime type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl DateTime {
    /// Get now
    pub fn now() -> Self {
        DateTime(chrono::Utc::now())
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::now()
    }
}

impl Deref for DateTime {
    type Target = chrono::DateTime<chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        bson::serde_helpers::chrono_datetime_as_bson_datetime::serialize(self, serializer)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        bson::serde_helpers::chrono_datetime_as_bson_datetime::deserialize(deserializer)
            .map(Into::into)
    }
}

#[async_graphql::Scalar]
impl ScalarType for DateTime {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(s) = &value {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
                return Ok(Self(dt.to_utc()));
            }
        }

        Err(async_graphql::InputValueError::expected_type(value))
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.to_rfc3339())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }
}

impl From<DateTime> for chrono::DateTime<chrono::Utc> {
    fn from(value: DateTime) -> Self {
        value.0
    }
}

impl Add<TimeDelta> for DateTime {
    type Output = DateTime;

    #[inline]
    fn add(self, rhs: TimeDelta) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<std::time::Duration> for DateTime {
    type Output = DateTime;

    #[inline]
    fn add(self, rhs: std::time::Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<TimeDelta> for DateTime {
    #[inline]
    fn add_assign(&mut self, rhs: TimeDelta) {
        self.0 += rhs;
    }
}

impl AddAssign<std::time::Duration> for DateTime {
    #[inline]
    fn add_assign(&mut self, rhs: std::time::Duration) {
        self.0 += rhs;
    }
}

impl Add<FixedOffset> for DateTime {
    type Output = DateTime;

    #[inline]
    fn add(self, rhs: FixedOffset) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<Months> for DateTime {
    type Output = DateTime;

    fn add(self, rhs: Months) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<TimeDelta> for DateTime {
    type Output = DateTime;

    #[inline]
    fn sub(self, rhs: TimeDelta) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<std::time::Duration> for DateTime {
    type Output = DateTime;

    #[inline]
    fn sub(self, rhs: std::time::Duration) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<TimeDelta> for DateTime {
    #[inline]
    fn sub_assign(&mut self, rhs: TimeDelta) {
        self.0 -= rhs;
    }
}

impl SubAssign<std::time::Duration> for DateTime {
    #[inline]
    fn sub_assign(&mut self, rhs: std::time::Duration) {
        self.0 -= rhs;
    }
}

impl Sub<FixedOffset> for DateTime {
    type Output = DateTime;

    #[inline]
    fn sub(self, rhs: FixedOffset) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<Months> for DateTime {
    type Output = DateTime;

    fn sub(self, rhs: Months) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<DateTime> for DateTime {
    type Output = TimeDelta;

    #[inline]
    fn sub(self, rhs: DateTime) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<&DateTime> for DateTime {
    type Output = TimeDelta;

    #[inline]
    fn sub(self, rhs: &DateTime) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Add<Days> for DateTime {
    type Output = DateTime;

    fn add(self, rhs: Days) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<Days> for DateTime {
    type Output = DateTime;

    fn sub(self, rhs: Days) -> Self::Output {
        Self(self.0 - rhs)
    }
}