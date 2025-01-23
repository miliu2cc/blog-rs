use salvo::{handler, oapi::extract::JsonBody, writing::Json, Request, Response, Writer};
use sea_orm::{sea_query::table, sqlx:: Database, ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter, Select, Set, TryIntoModel};
use web_app::connect_db;
use serde::Deserialize;


use crate::entity::articles;

#[derive(Debug, Deserialize)]
pub struct CreateArticleDto {
    pub title: String,
    pub content: String,
    pub cover_url: Option<String>,
    pub category_id: Option<i32>,
    pub is_top: Option<i8>,
    pub is_published: Option<i8>,
}

#[handler]
pub async fn article_list(res: &mut Response) {
    let db = connect_db().await;

    let list : Vec<articles::Model> = articles::Entity::find()
        .filter(articles::Column::IsTop.eq(0))
        .all(&db).await.unwrap_or_default();
    res.render(Json(list));
}



#[handler]
pub async fn top_list(res: &mut Response) {
    let db = connect_db().await;

    let list : Vec<articles::Model> = articles::Entity::find()
        .filter(articles::Column::IsTop.eq(1))
        .all(&db)
        .await
        .unwrap_or_default();
    res.render(Json(list));

}

#[handler]
pub async fn article_info(req : &mut Request ,res : &mut Response) {
    let db = connect_db().await;
    let id = req.param::<i64>("id").unwrap();

    let info : Option<articles::Model> = articles::Entity::find()
        .filter(articles::Column::Id.eq(id))
        .one(&db)
        .await
    .unwrap_or_default();
    res.render(Json(info));
}

#[handler]
pub async fn create_artilce(json : JsonBody<CreateArticleDto>) {
    let db = connect_db().await;

    let article_dto  = json.into_inner();
    let article = articles::ActiveModel {
        id: NotSet,
        title: Set(article_dto.title),
        cover_url: Set(article_dto.cover_url),
        content: Set(article_dto.content),
        user_id: Set(1),
        created_at: NotSet,
        updated_at:NotSet,
        is_top: Set(article_dto.is_top),
        is_published: Set(article_dto.is_published),
        category_id: Set(article_dto.category_id),
        is_deleted: Set(Some(0)),
    };

    //article.insert(&db).await;
    let _ = articles::Entity::insert(article).exec(&db).await.unwrap();

}
