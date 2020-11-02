use diesel::{PgConnection, SaveChangesDsl};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::acl::models::Account;
use crate::types::PasswordType;
use crate::ServerError;

pub struct Auth;

impl Auth {
    pub fn get_password_hash(conf: &rocket::Config) -> PasswordType {
        if let Some(rocket::config::Value::Table(area)) = conf.extras.get("databases") {
            if let Some(pw_hash) = area.get("password_type") {
                return match pw_hash.as_str() {
                    Some("bcrypt") => PasswordType::Bcrypt,
                    Some("argon2") => PasswordType::Argon2,
                    _ => PasswordType::Argon2,
                };
            }
        }
        PasswordType::Argon2
    }

    pub fn verify(acc: &Account, pass: &str) -> Result<bool, ServerError> {
        let salt = &acc.password_salt;
        let hash = Auth::hash_password(pass, salt, acc.password_type)?;
        Ok(hash == acc.password_hash)
    }

    pub fn perform_login(
        acc: &mut Account,
        pass: &str,
        conn: &PgConnection,
    ) -> Result<bool, ServerError> {
        if Auth::verify(&acc, pass)? {
            acc.auth_token = Some(Auth::generate_token());
            acc.save_changes::<Account>(conn)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn logout(acc: &mut Account, conn: &PgConnection) -> Result<(), ServerError> {
        acc.auth_token = None;
        acc.save_changes::<Account>(conn)?;
        Ok(())
    }

    pub fn generate_token() -> String {
        thread_rng().sample_iter(&Alphanumeric).take(32).collect()
    }

    pub fn generate_salt() -> Vec<u8> {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(|item| item as u8)
            .collect()
    }

    pub fn hash_password(
        pass: &str,
        salt: &[u8],
        ty: PasswordType,
    ) -> Result<Vec<u8>, ServerError> {
        match ty {
            PasswordType::Argon2 => {
                Ok(argon2::hash_raw(pass.as_bytes(), salt, &Default::default())
                    .map_err(|_| ServerError::HashError)?)
            }
            PasswordType::Bcrypt => {
                let hash = bcrypt::hash_with_salt(pass, bcrypt::DEFAULT_COST, salt)
                    .map_err(|_| ServerError::HashError)?;
                Ok(hash.to_string().as_bytes().to_vec())
            }
        }
    }

    /// Hash a fake password with a random salt just for the purposes of
    /// waisting time.
    pub fn hash_nonsense(hash: Option<PasswordType>) -> Result<(), ServerError> {
        let fake_pw = "blad 98sd asdj343 ";
        let salt = Auth::generate_salt();
        let ty = hash.unwrap_or_else(Default::default);
        Auth::hash_password(fake_pw, &salt, ty)?;
        Ok(())
    }
}
