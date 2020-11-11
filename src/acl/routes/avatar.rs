use std::io::Error;

use rocket::response::NamedFile;
use crate::acl::guards::AuthorizedAccount;
use rocket::{State, Config};

#[rocket::get("/<id>?")]
pub fn avatar_image(
    _auth: AuthorizedAccount,
    config: State<Config>,
    id: rocket_contrib::uuid::Uuid,
) -> Result<NamedFile, Error> {
    let data_path = config.get_string("storage_path").unwrap();
    NamedFile::open(format!("{}/avatars/{}.png", data_path, id))
}