use crate::{
    context,
    database::{Database, Entry},
    template::render,
    utils::gen_rnd_id,
    AppError, EndpointResult,
};
use rocket::form::Form;
use rocket::{
    get, post,
    response::{Flash, Redirect},
    State,
};
use rocket_dyn_templates::Template;

#[post("/new", data = "<task>")]
pub(crate) fn new(db: &State<Database>, task: Form<Entry>) -> EndpointResult<Flash<Redirect>> {
    if task.slug.is_empty() || task.target.is_empty() {
        Ok(Flash::error(Redirect::to("/"), "Cannot be empty."))
    } else {
        let key = task.slug.as_bytes();
        db.urls.insert(key, task.clone())?;
        Ok(Flash::success(Redirect::to("/"), "Task added."))
    }
}

#[get("/s/<url>")]
pub(crate) fn redirect(db: &State<Database>, url: String) -> EndpointResult<Redirect> {
    let url = db.urls.get(url)?.ok_or(AppError::NotFound)?;
    // look up uri to uuid
    Ok(Redirect::to(url.target))
}

#[get("/")]
pub(crate) fn index(db: &State<Database>) -> EndpointResult<Template> {
    Ok(render(
        "index",
        context! {
            slug_suggestion: gen_rnd_id(db)?,
        },
    ))
}
