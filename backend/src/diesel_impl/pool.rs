use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use tokio::task;

use crate::config::MysqlConfig;

pub type Pool<T>= r2d2::Pool<ConnectionManager<T>>;
pub type MysqlPool= Pool<diesel::mysql::MysqlConnection>;


// mysql
pub type DbConn= MysqlPool;

pub fn db_pool(config:MysqlConfig)->DbConn{
    let database_url=format!("mysql://{}:{}@{}:{}/{}",config.user,config.password,config.host,config.port,config.database);
    println!("Database: {}",database_url);

    let manager= ConnectionManager::<diesel::mysql::MysqlConnection>::new(database_url);
    Pool::builder()
    .max_size(config.max_connections)
    .build(manager)
    .expect("Failed to create pool")
}


pub async fn run<F,R>(f: F)-> Result<R, tokio::task::JoinError>
where F: FnOnce() -> R + Send + 'static,
      R: Send + 'static
{
    task::spawn_blocking(f).await
}

pub type AsyncPoolError= tokio::task::JoinError;