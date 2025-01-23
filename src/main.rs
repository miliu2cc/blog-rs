use salvo::prelude::*;
use sea_orm::{Database, DatabaseConnection};


mod router;
mod entity;
mod handler;



#[tokio::main]
async  fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().push(router::init_router());
    let accepter = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(accepter).serve(router).await;
}
