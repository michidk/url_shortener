use crate::{
    context,
    database::{Database, Entry},
    template::render,
    utils::gen_rnd_id,
    EndpointResult,
};
use rocket::{
    get, post,
    request::Form,
    response::{Flash, Redirect},
    State,
};
use rocket_contrib::templates::Template;

#[post("/new", data = "<task>")]
pub(crate) fn new(db: State<Database>, task: Form<Entry>) -> EndpointResult<Flash<Redirect>> {
    if task.slug.is_empty() || task.target.is_empty() {
        Ok(Flash::error(Redirect::to("/"), "Cannot be empty."))
    } else {
        let key = task.slug.as_bytes();
        db.urls.insert(key, task.clone())?;
        Ok(Flash::success(Redirect::to("/"), "Task added."))
    }
}

#[get("/s/<url>")]
pub(crate) fn redirect(db: State<Database>, url: String) -> EndpointResult<Redirect> {
    let url = db.urls.get(url)??;
    // look up uri to uuid
    Ok(Redirect::to(url.target))
}

#[get("/")]
pub(crate) fn index(db: State<Database>) -> EndpointResult<Template> {
    Ok(render(
        "index",
        context! {
            slug_suggestion: gen_rnd_id(db)?,
        },
    ))
}
