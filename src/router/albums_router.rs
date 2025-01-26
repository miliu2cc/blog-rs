use salvo::Router;

use crate::handler::{albums_handler::{album_list, create_album, del_album}, photo_handler::album_info};

pub fn init_router() -> Router {
    let router = Router::new()
        .push(
            Router::with_path("albums")
            .get(album_list)
            .post(create_album)
            .push(
                Router::with_path("{id}")
                    .get(album_info)

            )
            .push(
                    Router::with_path("del")
                    .push(
                            Router::with_path("{id}")
                            .delete(del_album)
                    )
            )

        );
}
