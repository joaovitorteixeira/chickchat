use rocket::fairing::AdHoc;
use rocket_db_pools::{diesel::MysqlPool, Database};

#[derive(Database)]
#[database("diesel_mysql")]
pub struct DbConn(MysqlPool);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel MySQL Stage", |rocket| async {
        rocket.attach(DbConn::init())
    })
}
