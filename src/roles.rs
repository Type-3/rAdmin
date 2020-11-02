use radmin_macros::Role;

pub trait RoleDef {
    const NAME: &'static str;
}

#[derive(Role)]
#[name("admin")]
pub struct AdminRole;

#[derive(Role)]
#[name("user")]
pub struct UserRole;
