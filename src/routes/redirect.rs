use rocket::response::Redirect;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm_rocket::Connection;

use crate::{pool::Db, AppError, EndpointResult};

#[get("/s/<slug>")]
pub async fn redirect(conn: Connection<'_, Db>, slug: String) -> EndpointResult<Redirect> {
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
