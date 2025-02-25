use salvo::{cors::Cors, http::Method, prelude::*};
use sea_orm::{Database, DatabaseConnection};


mod router;
mod entity;
mod handler;
mod auth;



#[tokio::main]
async  fn main() {
    tracing_subscriber::fmt().init();

    let cors = Cors::new()
        //.allow_origin("http://localhost:3000")
        .allow_origin(vec!["http://localhost:3000", "http://localhost:3000/login"])
        .allow_methods(vec![Method::GET, Method::POST,Method::DELETE])
        .into_handler();

    let router = Router::new().push(router::init_router());
    let server = Service::new(router).hoop(cors);
    let accepter = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(accepter).serve(server).await;
}
