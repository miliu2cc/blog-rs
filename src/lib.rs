use sea_orm::{Database, DatabaseConnection, DbConn, DbErr};

pub async fn connect_db() -> DatabaseConnection {
    let db = Database::connect("mysql://miliu:123456@localhost/blog").await.expect("faild");
    db
    
}
