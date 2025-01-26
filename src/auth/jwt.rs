use salvo::{jwt_auth::{ConstDecoder, HeaderFinder}, prelude::JwtAuth};



pub struct JwtClaims {
    username: String,
    exp: i64,
}

pub async fn create_jwt(secret_key: String) {
    JwtAuth::new(ConstDecoder::from_secret(secret_key.as_bytes()))
    .finders(vec![
        Box::new(HeaderFinder::new())
    ])
    .force_passed(true);
}
