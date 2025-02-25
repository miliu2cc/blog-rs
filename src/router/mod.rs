use salvo::Router;
use sea_orm::DatabaseConnection;

mod article_router;
mod post_router;
mod albums_router;
mod admin_router;

pub fn init_router() -> Router {
    let router :Router = Router::new()
        .push(article_router::init_router())
        .push(post_router::init_router())
        .push(albums_router::init_router())
        .push(admin_router::init_router());
    router
    
}
