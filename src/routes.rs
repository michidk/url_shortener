use crate::pool::Db;
use crate::{context, template::render, AppError, EndpointResult};
use rocket::form::Form;
use rocket::{
    get, post,
    response::{Flash, Redirect},
};
use rocket_dyn_templates::Template;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
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

#[get("/s/<slug>")]
pub async fn perform_redirect(conn: Connection<'_, Db>, slug: String) -> EndpointResult<Redirect> {
    let db = conn.into_inner();

    // look up uri to uuid
    let res = entity::redirect::Entity::find()
        .filter(entity::redirect::Column::Slug.eq(slug))
        .one(db)
        .await?;
    if let Some(model) = res {
        Ok(Redirect::to(model.target))
    } else {
        Err(AppError::NotFound)
    }
}

#[get("/")]
pub(crate) fn index() -> EndpointResult<Template> {
    Ok(render("index", context! {}))
}
