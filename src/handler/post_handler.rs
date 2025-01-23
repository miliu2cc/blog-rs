use salvo::{handler, oapi::extract::JsonBody, writing::Json,Writer, Response};
use sea_orm::{sqlx::types::uuid::timestamp::context, EntityTrait, Set};
use web_app::connect_db;
use serde::Deserialize;
use crate::entity::posts;

#[derive(Deserialize,Debug)]
pub struct CrateeatepostDto{
    pub content : String,
}

#[handler]
pub async fn post_list(res : &mut Response) {
    let db  = connect_db().await;

    let list : Vec<posts::Model> = posts::Entity::find().all(&db).await.unwrap_or_default();
    res.render(Json(list));
}

#[handler]
pub async fn create_post(json : JsonBody<CrateeatepostDto>) {
    let db = connect_db().await;

    let post_dto = json.into_inner();
    let post = posts::ActiveModel{
        content: Set(post_dto.content),
        ..Default::default()
    };

    let _ = posts::Entity::insert(post).exec(&db).await.unwrap();
}
