use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::{errors::Error, Result};

pub fn hash_password(password: &str) -> Result<String>{
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password: Vec<u8> = password.bytes().collect();
    let password_hash = argon2.hash_password(&password, &salt).map_err(|_|{Error::Hash})?.to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool>{
    let hash = PasswordHash::new(hash).map_err(|error|{Error::Hash})?;
    let password: Vec<u8> = password.bytes().collect();
    Argon2::default().verify_password(&password, &hash).map_err(|error|{Error::BadPassword})?;

    Ok(true)
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn can_hash_password(){
        let result = hash_password("test");

        assert!(result.is_ok())
    }

    #[test]
    fn can_verify_good_password(){
        let hash = "$argon2id$v=19$m=19456,t=2,p=1$gcLRU4ew7Z2CZyzbxhAkcQ$C3eVFJaFxB6wdXMy5MZJhEFq3cTH/nP1RnizqP7Y2F8";
        let result = verify_password("test", &hash);
        
        assert!(result.is_ok())
    }

    #[test]
    fn can_verify_bad_password(){
        let hash = "$argon2id$v=19$m=19456,t=2,p=1$gcLRU4ew7Z2CZyzbxhAkcQ$C3eVFJaFxB6wdXMy5MZJhEFq3cTH/nP1RnizqP7Y2F8";
        let result = verify_password("bad", &hash);
        
        assert!(result.is_err())
    }

    #[test]
    fn can_hash_password_and_verify(){
        let password = "goodpassword";
        let hash = match hash_password(password){
            Ok(hash) => hash,
            Err(_) => {
                assert!(false);
                return;
            }
        };

        let result = verify_password(password, &hash);
        assert!(result.is_ok())
    }
}
