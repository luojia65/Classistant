pub const VERSION: &'static str = "2019-10-17";

pub fn register_user_by_nickname(
    db: &crate::db::Database,
    nickname: &str,
    hash_base64: &str,
) -> crate::Result<u64> {
    let mut buf = vec![0u8; 64];
    crate::auth_hash::auth_id_hash(nickname, &mut buf);
    let hash_bytes = base64::decode(hash_base64)?;
    db.register_user_by_nickname(&buf, &hash_bytes)
}

pub fn login_by_auth_id(
    db: &crate::db::Database,
    auth_id_str: &str,
    hash_base64: &str,
) -> crate::Result<u64> {
    let mut buf = vec![0u8; 64];
    crate::auth_hash::auth_id_hash(auth_id_str, &mut buf);
    let hash_bytes = base64::decode(hash_base64)?;
    db.login_by_auth_id(&buf, &hash_bytes)
}
