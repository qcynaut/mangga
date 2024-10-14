use bson::oid::ObjectId;
pub use datetime::DateTime;

mod datetime;

/// ID type
pub type ID = ObjectId;
