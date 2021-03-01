use rocket::request::FromForm;
use serde::{Deserialize, Serialize};
use sled_extensions::{bincode::Tree, Db};

pub(crate) struct Database {
    pub(crate) urls: Tree<Entry>,
}

#[derive(Deserialize, Serialize, Clone, FromForm)]
pub(crate) struct Entry {
    pub(crate) slug: String,
    pub(crate) target: String,
}

pub(crate) fn setup() -> Db {
    sled_extensions::Config::default()
        .path("./db")
        .open()
        .expect("Failed to open sled db")
}
