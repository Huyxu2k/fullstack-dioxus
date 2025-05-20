use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MysqlConfig{
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    #[serde(default ="default_conn")]
    pub max_connections: u32
}
fn default_conn()->u32{
    5
}