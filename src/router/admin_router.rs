use salvo::Router;

use crate::{
    auth::jwt::jwt_middleware,
    handler::{
        article_handler::{create_artilce, del_article, update_top_status, update_publish_status, update_delete_status, update_article},
        user_handler::{login, registin},
        post_handler::{create_post, del_post},
        albums_handler::{create_album, del_album},
        photo_handler::{create_photo, del_photo}
    }
};

pub fn init_router() -> Router {
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
                Router::with_path("articles")
                .push(
                    Router::with_path("create")
                    .post(create_artilce)
                )
                .push(
                    Router::with_path("update/{id}")
                    .post(update_article)
                )
                .push(
                    Router::with_path("top/{id}/{status}")
                    .post(update_top_status)
                )
                .push(
                    Router::with_path("publish/{id}/{status}")
                    .post(update_publish_status)
                )
                .push(
                    Router::with_path("delete/{id}/{status}")
                    .post(update_delete_status)
                )
                .push(
                    Router::with_path("del/{id}")
                    .delete(del_article)
                )
            )
            .push(
                Router::with_path("posts")
                .post(create_post)
                .push(
                    Router::with_path("del/{id}")
                    .delete(del_post)
                )
            )
            .push(
                Router::with_path("albums")
                .post(create_album)
                .push(
                    Router::with_path("del/{id}")
                    .delete(del_album)
                )
            )
            .push(
                Router::with_path("photos")
                .post(create_photo)
                .push(
                    Router::with_path("del/{id}")
                    .delete(del_photo)
                )
            )
        );
    router
}
