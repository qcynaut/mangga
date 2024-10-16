mod db;
mod traits;
mod types;
pub(crate) mod operations;

pub use types::{Error, Result};

pub mod prelude {
    pub use crate::{
        db::{connect_database, get_database},
        traits::*,
        types::{is_id, ID},
    };
    pub use bson;
    pub use mangga_macro::Model;
    pub use mongodb;
}
