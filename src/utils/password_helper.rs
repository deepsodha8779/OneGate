use argon2::{Config, Result};
use rand::Rng;

pub fn hash_password(password: &str) -> Result<String> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();
    argon2::hash_encoded(&password.as_bytes(), &salt, &config)
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool> {
    argon2::verify_encoded(&hash, &password.as_bytes())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn password_hashing() {
        let hash = hash_password(&"password").unwrap();
        let is_valid = verify_password(&hash, &"password").unwrap_or(false);
        assert!(is_valid);
        assert_ne!(&hash, &"password");
    }
}
