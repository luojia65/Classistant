#[derive(Debug, Clone)]
pub struct MySQLDb {
    pub pool: mysql::Pool
}

impl MySQLDb {
    pub fn register_user_by_nickname(
        &self,
        nickname_hash: &[u8],
        hash_bytes: &[u8],
    ) -> crate::Result<u64> {
        let mut conn = self.pool.get_conn()?;
        let mut stmt = conn.prepare("CALL PUserRegisterByNickname(?)")?;
        let mut ans_iter = stmt.execute((nickname_hash,))?;
        let ans = if let Some(ans) = ans_iter.next() { ans } 
        else { return Err(crate::Error::EmptyResponse) }?;
        let return_id: u64 = if let Some(ans) = ans.get("return_id") { ans }
        else { return Err(crate::Error::FieldNotFound) };
        if return_id != 0 {
            if return_id == 1 {
                return Err(crate::Error::UserAlreadyExists)
            } 
            return Err(crate::Error::InvalidReturnId)
        }
        let user_id: u64 = if let Some(ans) = ans.get("user_id") { ans }
        else { return Err(crate::Error::FieldNotFound) };
        let mut conn = self.pool.get_conn()?;
        let mut stmt = conn.prepare("CALL PUserRegisterFillHash(?, ?)")?;
        stmt.execute((user_id, hash_bytes))?;
        Ok(user_id)
    }

    pub fn login_by_auth_id(
        &self,
        auth_id_hash: &[u8],
        hash_bytes: &[u8],
    ) -> crate::Result<u64> {
        let mut conn = self.pool.get_conn()?;
        let mut stmt = conn.prepare("CALL PUserLoginByAuthId(?, ?)")?;
        let mut ans_iter = stmt.execute((auth_id_hash, hash_bytes))?;
        let ans = if let Some(ans) = ans_iter.next() { ans } 
        else { return Err(crate::Error::EmptyResponse) }?;
        let return_id: u64 = if let Some(ans) = ans.get("return_id") { ans }
        else { return Err(crate::Error::FieldNotFound) };
        if return_id != 0 {
            if return_id == 3 {
                return Err(crate::Error::UserNotExists)
            } else if return_id == 1 {
                return Err(crate::Error::WrongPassword)
            }
            return Err(crate::Error::InvalidReturnId)
        }
        let user_id: u64 = if let Some(ans) = ans.get("user_id") { ans }
        else { return Err(crate::Error::FieldNotFound) };
        Ok(user_id)
    }
}
