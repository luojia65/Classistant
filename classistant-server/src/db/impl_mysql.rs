#[derive(Debug, Clone)]
pub struct MySQLDb {
    pub pool: mysql::Pool
}

impl MySQLDb {
    pub fn register_user_by_nickname(
        &self,
        nickname: &str,
        hash_base64: &str,
    ) -> crate::Result<Option<u64>> {
        let mut conn = self.pool.get_conn()?;
        let mut stmt = conn.prepare("CALL PUserRegisterByNickname(?, ?)")?;
        let mut ans_iter = stmt.execute((nickname,))?;
        let ans = if let Some(ans) = ans_iter.next() { ans } 
        else { return Err(crate::Error::None) }?;
        let return_id: u64 = if let Some(ans) = ans.get("return_id") { ans }
        else { return Err(crate::Error::None) };
        if return_id != 0 {
            if return_id == 1 {
                return Ok(None)
            } 
            return Err(crate::Error::InvalidReturnId)
        }
        let user_id: u64 = if let Some(ans) = ans.get("user_id") { ans }
        else { return Err(crate::Error::None) };
        let hash_bytes = base64::decode(hash_base64)?;
        let mut conn = self.pool.get_conn()?;
        let mut stmt = conn.prepare("CALL PUserRegisterFillHash(?, ?)")?;
        stmt.execute((user_id, hash_bytes))?;
        Ok(Some(user_id))
    }
}
