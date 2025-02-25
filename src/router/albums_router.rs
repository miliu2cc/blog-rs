use salvo::Router;

use crate::handler::{albums_handler::{album_info, album_list}, photo_handler::photo_list};

pub fn init_router() -> Router {
    let router = Router::new()
        .push(
            Router::with_path("albums")
            .get(album_list)
            .push(
                Router::with_path("{id}")
                .get(album_info)
            )
        )
            .push(
            Router::with_path("photo")
            .push(
                    Router::with_path("{id}")
                    .get(photo_list)
            )
        );
    router
}
