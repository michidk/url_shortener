use crate::pool::Db;
use crate::{context, template::render, EndpointResult};
use rocket::form::Form;
use rocket::{
    get, post,
    response::{Flash, Redirect},
};
use rocket_dyn_templates::Template;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm_rocket::Connection;

use entity::redirect;

#[post("/new", data = "<post_form>")]
pub async fn new(
    conn: Connection<'_, Db>,
    post_form: Form<redirect::Model>,
) -> EndpointResult<Flash<Redirect>> {
    let db = conn.into_inner();

    if post_form.slug.is_empty() || post_form.target.is_empty() {
        Ok(Flash::error(Redirect::to("/"), "Cannot be empty."))
    } else {
        let form = post_form.into_inner();

        redirect::ActiveModel {
            slug: Set(form.slug.to_owned()),
            target: Set(form.target.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
        .expect("could not insert redirect");

        Ok(Flash::success(Redirect::to("/"), "Task added."))
    }
}

#[get("/")]
pub(crate) fn index() -> EndpointResult<Template> {
    Ok(render("index", context! {}))
}
