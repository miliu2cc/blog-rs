use salvo::Router;
use sea_orm::DatabaseConnection;

mod article_router;
mod post_router;

pub fn init_router() -> Router {
    let router :Router = Router::new()
        .push(article_router::init_router())
        .push(post_router::init_router());
    router
    
}
