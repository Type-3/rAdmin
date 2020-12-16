#[cfg(feature = "tera")]
pub mod tera {
    use serde_json::Value;
    use rocket_contrib::templates::tera::Result;

    use std::collections::HashMap;

    pub fn avatar(value: Value, _: HashMap<String, Value>) -> Result<Value> {

        match value {
            Value::Null => Ok("/assets/img/default-avatar.png".into()),
            Value::String(string) => Ok(format!("/avatars/{}", string).into()),
            value => Ok(value)
        }
    }
}
