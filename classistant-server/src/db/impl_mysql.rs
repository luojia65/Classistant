use std::collections::HashMap;

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

    pub fn group_create(
        &self,
        user_id: u64,
    ) -> crate::Result<u64> {
        let mut conn = self.pool.get_conn()?;
        let mut stmt = conn.prepare("CALL PGroupCreate(?)")?;
        let mut ans_iter = stmt.execute((user_id,))?;
        let ans = if let Some(ans) = ans_iter.next() { ans } 
        else { return Err(crate::Error::EmptyResponse) }?;
        let group_id: u64 = if let Some(ans) = ans.get("group_id") { ans }
        else { return Err(crate::Error::FieldNotFound) };
        Ok(group_id)
    }

    pub fn group_delete(
        &self,
        group_id: u64,
        user_id: u64,
    ) -> crate::Result<()> {
        let mut conn = self.pool.get_conn()?;
        let mut stmt = conn.prepare("CALL PGroupDelete(?, ?)")?;
        let mut ans_iter = stmt.execute((group_id, user_id))?;
        let ans = if let Some(ans) = ans_iter.next() { ans } 
        else { return Err(crate::Error::EmptyResponse) }?;
        let return_id: u64 = if let Some(ans) = ans.get("return_id") { ans }
        else { return Err(crate::Error::FieldNotFound) };
        if return_id != 0 {
            if return_id == 2 {
                return Err(crate::Error::GroupNotExists)
            } else if return_id == 1 {
                return Err(crate::Error::PermissionDenied)
            }
            return Err(crate::Error::InvalidReturnId)
        }
        Ok(())
    }
    
    pub fn group_transfer_owner(
        &self,
        group_id: u64,
        src_user_id: u64,
        dest_user_id: u64,
    ) -> crate::Result<()> {
        let mut conn = self.pool.get_conn()?;
        let mut stmt = conn.prepare("CALL PGroupTransferOwner(?, ?)")?;
        let mut ans_iter = stmt.execute((group_id, src_user_id, dest_user_id))?;
        let ans = if let Some(ans) = ans_iter.next() { ans } 
        else { return Err(crate::Error::EmptyResponse) }?;
        let return_id: u64 = if let Some(ans) = ans.get("return_id") { ans }
        else { return Err(crate::Error::FieldNotFound) };
        if return_id != 0 {
            if return_id == 2 {
                return Err(crate::Error::OperatorUserNotInGroup)
            } else if return_id == 1 {
                return Err(crate::Error::PermissionDenied)
            } else if return_id == 3 {
                return Err(crate::Error::DestUserNotInGroup)
            }
            return Err(crate::Error::InvalidReturnId)
        }
        Ok(())
    }

    pub fn data_get_batch(
        &self,
        user_id: u64,
        keys: Vec<String>,
    ) -> crate::Result<HashMap<String, (Vec<u8>, Vec<u8>)>> {
        let mut conn = self.pool.get_conn()?;
        let mut ret = HashMap::new();
        let mut stmt = conn.prepare("CALL PUserDataGet(?, ?)")?;
        for key in keys {
            let key_bytes: [u8; 16] = md5::compute(key.clone()).into();
            let mut ans_iter = stmt.execute((user_id, &key_bytes))?;
            let ans = if let Some(ans) = ans_iter.next() { ans } 
            else { continue }?;
            let data: Vec<u8> = if let Some(ans) = ans.get("data") { ans }
            else { return Err(crate::Error::FieldNotFound) };
            let encryption: Vec<u8> = if let Some(ans) = ans.get("encryption") { ans }
            else { return Err(crate::Error::FieldNotFound) };
            ret.insert(key, (data, encryption));
        }
        Ok(ret)
    }

    pub fn data_modify_batch(
        &self,
        user_id: u64,
        entries: HashMap<String, (Vec<u8>, Vec<u8>)>,
    ) -> crate::Result<Vec<String>> {
        let mut conn = self.pool.get_conn()?;
        let mut ret = Vec::new();
        let mut stmt = conn.prepare("CALL PUserDataInsert(?, ?, ?, ?)")?;
        for (key, (new_value, new_encrption)) in entries {
            let key_bytes: [u8; 16] = md5::compute(key.clone()).into();
            stmt.execute((user_id, &key_bytes, new_value, new_encrption))?;
            ret.push(key);
        }
        Ok(ret)
    }
}
