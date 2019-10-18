use serde::{Serialize, Deserialize};
use std::time::{Instant, Duration};

lazy_static::lazy_static! {
    static ref INSTANT_BEGIN: Instant = Instant::now();
}

#[derive(Serialize, Deserialize)]
pub struct IdentityInner {
    #[serde(rename = "u")]
    user_id: u64,
    #[serde(rename = "e")]
    expire: u64,
}

impl IdentityInner {
    #[inline] 
    pub fn new_uid(user_id: u64, expire_in_secs: u64) -> IdentityInner {
        let expire = Instant::now() + Duration::from_secs(expire_in_secs) - *INSTANT_BEGIN;
        let expire = expire.as_secs();
        IdentityInner { user_id, expire }
    }

    #[inline]
    pub fn from_json_str(s: &str) -> serde_json::Result<IdentityInner> {
        serde_json::from_str(s)
    }

    #[inline]
    pub fn to_json_string(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }

    #[inline]
    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn is_expired(&self) -> bool {
        let expire = *INSTANT_BEGIN + Duration::from_secs(self.expire);
        Instant::now() >= expire
    }
}
