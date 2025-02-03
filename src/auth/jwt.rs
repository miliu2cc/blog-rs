use jsonwebtoken::{decode, DecodingKey, Validation};
use salvo::{handler, http::StatusCode, jwt_auth::{ConstDecoder, HeaderFinder, JwtError}, oapi::security::Password, prelude::JwtAuth, Depot, FlowCtrl, Request, Response, Writer};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{de::value, Deserialize, Serialize};
use tracing_subscriber::reload::Error;
use web_app::connect_db;

use crate::entity::users;


#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
    pub exp: i64,
}

#[handler]
pub async fn jwt_middleware(req: &mut Request,res: &mut Response,depot: &mut Depot, ctrl : &mut FlowCtrl) {
    /*if let Some(auth_header) = req.header("Authorization") {
        let token: &str = auth_header.split_whitespace().last().unwrap();
    
        match valiadate_token(token).await {
            Ok(JwtClaims) => {
                ctrl.call_next(req, depot, res).await;
            }
            Err(_) => {
                    res.status_code(StatusCode::FORBIDDEN);
            }
        }

    }*/
    let token = req.headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

        match token {
            Some(token) => {
                // 验证 Token
                match validate_token(token).await {
                       Ok(claims) => {
                            // 将用户信息附加到请求上下文中
                            ctrl.call_next(req, depot, res).await;
                        }
                        Err(_) => {
                            // Token 无效，返回 401 Unauthorized
                            res.status_code(StatusCode::UNAUTHORIZED);
                            res.render("Invalid token");
                        }
                    }
                }
                None => {
                    // Token 不存在，返回 401 Unauthorized
                    res.status_code(StatusCode::UNAUTHORIZED);
                    res.render("Token not found");
                }
            }
        
    
        
    
}



pub async fn validate(username: &str, password: &str) -> bool {
    let db = connect_db().await;
    let pwd_result= users::Entity::find().filter(users::Column::Username.eq(username)).one(&db).await;
    match pwd_result {
        Ok(Some(users)) => {
           return  users.password == password;
        },
        Ok(None) => false,
        Err(e) => false
    }
}

pub async fn validate_token(token: &str) -> Result<JwtClaims, JwtError> {
    decode::<JwtClaims>(token, &DecodingKey::from_secret(b"abcdefg"), &Validation::default()).map(|data| data.claims)
}
