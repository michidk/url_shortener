#![feature(proc_macro_hygiene, decl_macro, try_trait)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket::State;
use rocket::{request::Form, response::Flash};

use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use sled::IVec;
use sled_extensions::bincode::Tree;
use sled_extensions::DbExt;
use std::option::NoneError;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("sled db error")]
    SledError(#[from] sled_extensions::Error),
    #[error("resource not found")]
    NotFound,
}

impl From<NoneError> for ServerError {
    fn from(_: NoneError) -> Self {
        ServerError::NotFound
    }
}

type EndpointResult<T> = Result<T, ServerError>;

struct Database {
    urls: Tree<Url>,
}

#[derive(Deserialize, Serialize, Clone, FromForm)]
struct Url {
    name: String,
    target: String,
}

#[derive(serde::Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        &TemplateContext {
            title: "Hello",
            name: Some("test".into()),
            items: vec!["One", "Two", "Three"],
        },
    )
}

//https://github.com/marcusbuffett/rocket_sled_tutorial/blob/tutorial/src/main.rs
#[post("/new", data = "<task>")]
fn new(db: State<Database>, task: Form<Url>) -> Flash<Redirect> {
    if task.name.is_empty() || task.target.is_empty() {
        Flash::error(Redirect::to("/"), "Cannot be empty.")
    } else {
        let key = task.name.as_bytes();
        db.urls.insert(key, task.clone());
        Flash::success(Redirect::to("/"), "Task added.")
    }
}

#[get("/s/<url>")]
fn redirect(db: State<Database>, url: String) -> EndpointResult<Redirect> {
    let url = db.urls.get(url)??;
    // look up uri to uuid
    Ok(Redirect::to(url.target))
}

fn main() {
    let db = sled_extensions::Config::default()
        .path("./db")
        .open()
        .expect("Failed to open sled db");
    rocket::ignite()
        .manage(Database {
            urls: db
                .open_bincode_tree("urls")
                .expect("failed to open user tree"),
        })
        .mount("/", routes![index, new, redirect])
        .attach(Template::fairing())
        .launch();
}
