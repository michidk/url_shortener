#![feature(proc_macro_hygiene, decl_macro, try_trait)]

mod database;
mod routes;
mod template;
mod utils;
use crate::database::Database;
use rocket::routes;
use rocket_contrib::{serve::StaticFiles, templates::Template};
use sled_extensions::DbExt;
use std::option::NoneError;

#[derive(thiserror::Error, Debug)]
pub(crate) enum AppError {
    #[error("Sled Error: {0}")]
    SledError(#[from] sled_extensions::Error),
    #[error("Resource not found")]
    NotFound,
}

impl From<NoneError> for AppError {
    fn from(_: NoneError) -> Self {
        AppError::NotFound
    }
}

pub(crate) type EndpointResult<T> = Result<T, AppError>;

fn main() {
    let db = database::setup();
    rocket::ignite()
        .manage(Database {
            urls: db
                .open_bincode_tree("urls")
                .expect("failed to open user tree"),
        })
        .mount(
            "/",
            routes![
                crate::routes::index,
                crate::routes::new,
                crate::routes::redirect
            ],
        )
        .mount("/", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();
}
