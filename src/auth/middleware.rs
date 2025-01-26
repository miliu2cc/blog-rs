use salvo::prelude::*;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;

pub struct JwtMiddleware;

impl Middleware for JwtMiddleware {
    fn handle(&self, req: &mut Request, res: &mut Response) -> Result<Response, _> {
        let authorization = req.headers().get("Authorization").and_then(|s| s.parse::<HeaderValue>().ok());

        if let Some(authorization) = authorization {
            if let Some(token) = authorization.to_str().ok() {
                if token.starts_with("Bearer ") {
                    let token = &token[7..];
                    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not set");

                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret_key.as_bytes()),
                        &Validation::default(),
                    ) {
                        Ok(_) => {
                            // 令牌有效，继续处理请求
                            return Ok(res.respond());
                        }
                        Err(_) => {
                            return Ok(res.status(401).body("Unauthorized").into());
                        }
                    }
                }
            }
        }

        Ok(res.status(401).body("Unauthorized").into())
    }
}
