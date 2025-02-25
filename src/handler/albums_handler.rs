use salvo::{handler, http::StatusCode, oapi::extract::JsonBody, writing::Json, Request, Response, Writer};
use sea_orm::{ColumnTrait, DeleteResult, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use web_app::connect_db;

use crate::entity::albums;


#[derive(Debug, Deserialize)]
pub struct CreatealbumDto {
    pub coverurl: Option<String>,
    pub description: Option<String>,
}

#[handler]
pub async fn album_list(res : &mut Response) {
    let db  = connect_db().await;

    let list : Vec<albums::Model> = albums::Entity::find().all(&db).await.unwrap_or_default();

    res.render(Json(list));
}

#[handler]
pub async fn album_info(req : &mut Request, res: &mut Response) {
    let db = connect_db().await;

    let id = req.param::<i64>("id").unwrap(); 

    let info : Option<albums::Model> = albums::Entity::find()
        .filter(albums::Column::Id.eq(id))
        .one(&db)
        .await
    .unwrap_or_default();
    res.render(Json(info));
    
}

#[handler]
pub async fn create_album(json : JsonBody<CreatealbumDto>, res : &mut Response) {
    let db = connect_db().await;

    let album_dto = json.into_inner();
    let album = albums::ActiveModel {
        cover_url: Set(album_dto.coverurl),
        description: Set(album_dto.description),
        ..Default::default()
    };

    let _ = albums::Entity::insert(album).exec(&db).await.unwrap();
    res.status_code(StatusCode::OK);
}

#[handler]
pub async fn update_album(req: &mut Request, json : JsonBody<CreatealbumDto>, res: &mut Response) {
    let db = connect_db().await;
    let id = req.param::<i32>("id").unwrap();
    let album_dto = json.into_inner();

    let album = albums::ActiveModel {
        id: Set(id),
        cover_url: Set(album_dto.coverurl),
        description: Set(album_dto.description),
        ..Default::default()
    };
    let _ = albums::Entity::update(album).exec(&db).await;
    res.render(StatusCode::OK);
}

#[handler]
pub async fn del_album(req : &mut Request,res : &mut Response) {
    let db = connect_db().await;
    let id = req.param::<i32>("id").unwrap();

    let _ = albums::Entity::delete_by_id(id).exec(&db).await;
    res.status_code(StatusCode::OK);
}
