use salvo::Router;

use crate::handler::{albums_handler::album_list, photo_handler::album_info};

pub fn init_router() -> Router {
    let router = Router::new()
        .push(
            Router::with_path("albums")
            .get(album_list)
            .push(
                Router::with_path("{id}")
                .get(album_info)
            )
        );
    router
}
