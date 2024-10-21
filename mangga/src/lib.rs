mod db;
pub(crate) mod operations;
mod traits;
mod types;

pub use types::{Error, Result};

pub mod prelude {
    pub use crate::{
        db::{connect_database, get_database},
        operations::Filter,
        traits::*,
        types::{is_id, ID, DateTime},
    };
    pub use bson;
    pub use mangga_macro::Model;
    pub use mongodb;
}
