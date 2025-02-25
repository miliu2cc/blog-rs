use salvo::Router;

use crate::handler::post_handler::*;

pub fn init_router() -> Router {
    let router = Router::new()
        .push(
            Router::with_path("posts")
            .get(post_list)
        );
    router
}
