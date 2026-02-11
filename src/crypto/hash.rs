use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

pub struct PasswordHash(String);

impl From<String> for PasswordHash {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl TryFrom<&str> for PasswordHash {
    type Error = argon2::password_hash::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hashed_pwd = argon2.hash_password(s.as_bytes(), &salt)?;
        let hashed_pwd = hashed_pwd.to_string();

        Ok(Self(hashed_pwd))
    }
}

impl PasswordHash {
    pub fn value(&self) -> String {
        self.0.clone()
    }

    pub fn verify(&self, password: &String) -> Result<bool, argon2::password_hash::Error> {
        let argon2 = Argon2::default();
        let binding = self.value();
        let parsed_hash = argon2::PasswordHash::new(binding.as_str())?;
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }
}