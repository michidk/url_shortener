#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::{fairing, Build, Rocket};
use rocket::{http::Status, response, routes, Request};

use rocket::fs::{relative, FileServer};
use rocket::response::Responder;
use rocket_dyn_templates::Template;

use migration::{DbErr, MigratorTrait};
use sea_orm_rocket::Database;

use pool::Db;
use serde_json::json;

mod pool;
mod routes;
mod template;
mod utils;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,
    #[error("DbError: {0}")]
    DbErr(#[from] DbErr),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Status::InternalServerError.respond_to(req)
    }
}

pub type EndpointResult<T> = Result<T, AppError>;

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        json! ({
            "uri": req.uri()
        }),
    )
}

async fn custom_bootstrap(rocket: Rocket<Build>) -> Rocket<Build> {
    // initializes the sqlite database, if it does not exist
    let url: Result<String, rocket::figment::Error> =
        rocket.figment().extract_inner("databases.sea_orm.url");
    if let Ok(path) = url {
        if let Some(path) = path.strip_prefix("sqlite://") {
            let path = std::path::Path::new(&path);
            if !path.exists() {
                std::fs::File::create(path).expect("Could not bootstrap sqlite database");
            }
        }
    }

    rocket
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
pub fn rocket() -> _ {
    #[allow(unused_variables)]
    rocket::build()
        .attach(AdHoc::on_ignite("Bootstrapping", custom_bootstrap))
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", FileServer::from(relative!("static")))
        .mount(
            "/",
            routes![
                crate::routes::index::index,
                crate::routes::index::new,
                crate::routes::redirect::redirect,
                crate::routes::list::list
            ],
        )
        .register("/", catchers![not_found])
        .attach(Template::try_custom(|engines| {
            Ok(())
        }))
}
