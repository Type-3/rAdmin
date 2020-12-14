use std::io::Error;

use crate::acl::guards::AuthorizedAccount;
use rocket::response::NamedFile;
use rocket::{Config, State};

#[rocket::get("/<id>?", rank=6)]
pub fn avatar_image(
    _auth: AuthorizedAccount,
    config: State<Config>,
    id: rocket_contrib::uuid::Uuid,
) -> Result<NamedFile, Error> {
    let data_path = config.get_string("storage_path").unwrap();
    NamedFile::open(format!("{}/avatars/{}.png", data_path, id))
}
