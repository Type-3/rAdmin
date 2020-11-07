use std::io::Error;

use rocket::response::NamedFile;
use crate::acl::guards::AuthorizedAccount;

#[rocket::get("/<id>?")]
pub fn avatar_image(
    _auth: AuthorizedAccount,
    id: rocket_contrib::uuid::Uuid,
) -> Result<NamedFile, Error> {
    NamedFile::open(format!("data/avatars/{}.png", id))
}