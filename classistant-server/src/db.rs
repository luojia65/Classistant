use std::io;

mod impl_mysql;

pub(crate) fn connect_mysql(params: Vec<&str>) -> Database {
    let mut builder = mysql::OptsBuilder::new();
    builder.ip_or_hostname(Some(params[0]));
    builder.tcp_port(params[1].parse().expect("parse port"));
    builder.user(Some(params[2]));
    builder.pass(Some(params[3]));
    builder.db_name(Some(params[4]));
    let pool = mysql::Pool::new(builder)
        .expect("connect to mysql server");
    Database::MySQL(impl_mysql::MySQLDb { pool })
}

#[derive(Clone)]
pub enum Database {
    MySQL(impl_mysql::MySQLDb),
}

impl Database {
    pub fn register_user_by_nick(
        nickname: String
    ) -> io::Result<()> {
        Ok(())
    }
}