use salvo::{handler, http::StatusCode,  http::headers::SetCookie, oapi::extract::JsonBody, writing::Json, Request, Response, Writer};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use web_app::connect_db;


use crate::entity::photos;

#[derive(Debug, Deserialize)]
pub struct CreatephotoDto {
    album_id : i32,
    image_url : String,
}

#[handler]
pub async fn photo_list(req: & mut Request, res: &mut  Response) {
    let db = connect_db().await;
    let id = req.param::<i64>("id").unwrap();

    let list = photos::Entity::find().filter(
        photos::Column::AlbumId.eq(id)
    ).all(&db).await.unwrap_or_default();

    res.render(Json(list));
}

#[handler]
pub async fn create_photo(json: JsonBody<CreatephotoDto>) {
    let db = connect_db().await;

    let photo_dto = json.into_inner();
    let photo = photos::ActiveModel {
        album_id: Set(photo_dto.album_id),
        image_url: Set(photo_dto.image_url),
        ..Default::default()
    };

    let _ = photos::Entity::insert(photo).exec(&db).await.unwrap();
    
}

#[handler]
pub async fn del_photo(req: &mut Request, res: &mut Response) {
    let db = connect_db().await;
    let id = req.param::<i32>("id").unwrap();

    let _ = photos::Entity::delete_by_id(id).exec(&db).await;
    res.status_code(StatusCode::OK);
}
