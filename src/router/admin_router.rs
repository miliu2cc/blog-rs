use salvo::Router;

use crate::{auth::jwt::{jwt_middleware}, handler::{article_handler::{create_artilce, del_article}, user_handler::{login, registin}}};

pub fn init_router() -> Router{
    let sky = "abcdefg";
    
    let router = Router::new()
        .push(
            Router::with_path("login")
            .post(login)
        )
        .push(
            Router::with_path("registin")
            .post(registin)
        )
        .push(
            Router::with_hoop(jwt_middleware)
            .push(
                Router::with_path("create-article")
                .post(create_artilce)
            )
            .push(
                    Router::with_path("del-article")
                    .push(Router::with_path("{id}").get(del_article))
            )
        );
    router
    
}
