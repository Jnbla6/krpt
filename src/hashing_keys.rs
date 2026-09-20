use sha2::{Digest, Sha256};

pub fn hashingstring(pass: String) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(pass.trim().as_bytes());
    let key: [u8; 32] = hasher.finalize().into();
    key
}

pub fn hashingvec(pass: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&pass);
    let key: [u8; 32] = hasher.finalize().into();
    key
}