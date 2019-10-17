use crate::db::Database;

#[derive(Debug, Clone)]
pub struct MySQLDb {
    pub pool: mysql::Pool
}
