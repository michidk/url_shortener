#[macro_use]
extern crate rocket;

mod database;
mod routes;
mod template;
mod utils;
use crate::database::Database;
use rocket::{http::Status, response, routes, Request};
use sled_extensions::DbExt;

use rocket::fs::{relative, FileServer};
use rocket::response::Responder;
use rocket_dyn_templates::Template;

#[derive(thiserror::Error, Debug)]
pub(crate) enum AppError {
    #[error("Sled Error: {0}")]
    SledError(#[from] sled_extensions::Error),
    #[error("Resource not found")]
    NotFound,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Status::InternalServerError.respond_to(req)
    }
}

pub(crate) type EndpointResult<T> = Result<T, AppError>;

#[launch]
fn rocket() -> _ {
    let db = database::setup();
    rocket::build()
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
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
