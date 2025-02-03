use jsonwebtoken::EncodingKey;
use salvo::{handler, Writer, http::StatusCode, oapi::extract::JsonBody, Request, Response};
use sea_orm::{EntityTrait, Set};
use serde::Deserialize;
use time::{Duration, OffsetDateTime};
use web_app::connect_db;

use crate::{auth::jwt::{validate, JwtClaims}, entity::users};

#[derive(Debug, Deserialize)]
pub struct UerDto {
    pub username: String,
    pub password: String,
}


#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    //test key
    let SECRET_KEY = "abcdefg";
    let login_data = req.parse_json::<UerDto>().await.unwrap_or(UerDto{
        username: "".to_string(),
        password: "".to_string(),
    });


    if !validate(&login_data.username, &login_data.password).await {
        res.status_code(StatusCode::NOT_FOUND);
        return;
    }

    let exp = OffsetDateTime::now_utc() + Duration::days(7);
    let claim = JwtClaims {
        username: login_data.username,
        exp: exp.unix_timestamp(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()));
    match token {
    Ok(token) => {
        res.render(token); // `String` 实现了 `Scribe`
    }
    Err(error) => {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR); // 或其他适当的错误状态码
        res.render("Failed to generate token"); // 渲染错误信息
    }
}
}

#[handler]
pub async fn registin(json: JsonBody<UerDto>, res: &mut Response) {
    let db = connect_db().await;
    let userdto = json.into_inner();

    let user = users::ActiveModel {
        username: Set(userdto.username),
        password: Set(userdto.password),
        ..Default::default()
    };

    let _ = users::Entity::insert(user).exec(&db).await.unwrap();
    res.status_code(StatusCode::OK);

}
