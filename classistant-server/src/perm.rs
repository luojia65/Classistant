use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
struct Perm {
    groups: Vec<u64>,
    users: Vec<u64>,
}

pub fn check_perm_user(perm: &str, user_id: u64) -> crate::Result<bool> {
    let a: Perm = from_str(perm)?;
    Ok(a.groups.contains(&user_id))
}


pub fn check_perm_group(perm: &str, groups: Vec<u64>) -> crate::Result<bool> {
    let a: Perm = from_str(perm)?;
    for i in groups {
        if a.groups.contains(&i) {
            return Ok(true)
        }
    }
    Ok(false)
}
