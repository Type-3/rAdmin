mod accounts;
pub use self::accounts::AccountRequest;

mod role;
pub use self::role::RoleRequest;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;
use validator::ValidationErrors;
use validator_derive::Validate;

#[derive(Debug, TypedBuilder, PartialEq, Clone, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[builder(setter(into))]
    #[validate(length(min = 4))]
    pub username: String,
    #[builder(setter(into))]
    #[validate(length(min = 4, message = "Password to small"))]
    #[validate(length(max = 25, message = "Password to long"))]
    pub password: String,
}

impl LoginRequest {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        Validate::validate(self)?;
        Ok(())
    }
}

#[derive(Debug, TypedBuilder, PartialEq, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into))]
    #[validate(length(min = 4))]
    pub username: String,
    #[builder(setter(into))]
    #[validate(length(min = 4, message = "Password to small"))]
    #[validate(length(max = 25, message = "Password to long"))]
    pub password: String,
    #[builder(setter(into))]
    //#[validate(must_match("password"))]
    #[validate(length(min = 4, message = "Password Confirm to short"))]
    #[validate(length(max = 25, message = "Password to long"))]
    pub password_config: String,
}

impl RegisterRequest {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        Validate::validate(self)?;
        Ok(())
    }
}
