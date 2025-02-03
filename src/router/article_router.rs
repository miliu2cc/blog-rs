use salvo::Router;
use sea_orm::DatabaseConnection;

use crate::handler::article_handler::{self, *};


pub fn init_router() -> Router {
    let router = Router::new()
        .push(
            Router::with_path("articles")
            .get(article_list)
            .push(
                    Router::with_path("top")
                    .get(top_list)
            )
            .push(
                    Router::with_path("{id}")
                    .get(article_info)
            )
        );

    router

}
