use salvo::{handler, oapi::extract::JsonBody, writing::Json,Writer, Request,Response};
use sea_orm::{sqlx::types::{chrono::Utc, uuid::timestamp::context}, EntityTrait, Set};
use web_app::connect_db;
use serde::Deserialize;
use crate::entity::posts;
use salvo::http::StatusCode;

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
        published_at: Set(Some(Utc::now().naive_utc())),
        ..Default::default()
    };

    let _ = posts::Entity::insert(post).exec(&db).await.unwrap();
}

#[handler]
pub async fn del_post(req: &mut Request, res: &mut Response) {
    let db = connect_db().await;
    let id = req.param::<i32>("id").unwrap();

    let _ = posts::Entity::delete_by_id(id).exec(&db).await;
    res.status_code(StatusCode::OK);
}
