use rocket_dyn_templates::Template;

use crate::{context, template::render, EndpointResult};

#[get("/list")]
pub(crate) fn list() -> EndpointResult<Template> {
    Ok(render("list", context! {}))
}
