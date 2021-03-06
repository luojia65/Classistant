use blake2::{Blake2b, Digest};

// pub fn hash(uid: u64, pwd: &str, buf: &mut [u8]){
//     let mut hasher = Blake2b::new();
//     hasher.input(pwd.as_bytes());
//     hasher.input(uid.to_le_bytes());
//     buf.clone_from_slice(&hasher.result());
// }

pub fn auth_id_hash(auth_id: &str, buf: &mut [u8]) {
    let mut hasher = Blake2b::new();
    hasher.input(auth_id.as_bytes());
    buf.clone_from_slice(&hasher.result());
}

