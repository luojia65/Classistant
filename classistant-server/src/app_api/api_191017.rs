pub const VERSION: &'static str = "2019-10-17";

pub fn register_user_by_nickname(
    db: &crate::db::Database,
    nickname: &str,
    hash: &str,
) -> crate::Result<Option<u64>> {
    db.register_user_by_nickname(nickname, hash)
}
