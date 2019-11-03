use std::collections::HashMap;

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

#[allow(irrefutable_let_patterns)]
impl Database {
    pub fn register_user_by_nickname(
        &self,
        nickname_hash: &[u8],
        hash_bytes: &[u8],
    ) -> crate::Result<u64> {
        if let Database::MySQL(db) = &self {
            db.register_user_by_nickname(nickname_hash, hash_bytes)
        } else {
            unreachable!()
        }
    }

    pub fn login_by_auth_id(
        &self,
        auth_id_hash: &[u8],
        hash_bytes: &[u8],
    ) -> crate::Result<u64> {
        if let Database::MySQL(db) = &self {
            db.login_by_auth_id(auth_id_hash, hash_bytes)
        } else {
            unreachable!()
        }
    }

    pub fn group_create(
        &self,
        user_id: u64,
    ) -> crate::Result<u64> {
        if let Database::MySQL(db) = &self {
            db.group_create(user_id)
        } else {
            unreachable!()
        }
    }

    pub fn group_delete(
        &self,
        group_id: u64,
        user_id: u64,
    ) -> crate::Result<()> { 
        if let Database::MySQL(db) = &self {
            db.group_delete(group_id, user_id)
        } else {
            unreachable!()
        }
    }

    pub fn group_transfer_owner(
        &self,
        group_id: u64,
        src_user_id: u64,
        dest_user_id: u64,
    ) -> crate::Result<()> {
        if let Database::MySQL(db) = &self {
            db.group_transfer_owner(group_id, src_user_id, dest_user_id)
        } else {
            unreachable!()
        }
    }
    
    pub fn data_get_batch(
        &self,
        user_id: u64,
        keys: Vec<Vec<u8>>
    ) -> crate::Result<HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>> {
        if let Database::MySQL(db) = &self {
            db.data_get_batch(user_id, keys)
        } else {
            unreachable!()
        }
    }

}
