use diesel::PgConnection;

use crate::ServerError;

pub trait Submitable {
    /// Submit the form.
    /// **Note** This assumes the form has already been validated.
    fn submit(self, _: &PgConnection) -> Result<(), ServerError>;
}
