use diesel::PgConnection;

use crate::ServerError;

pub trait Fillable<T> {
    fn fill(self, obj: &mut T, conn: &PgConnection) -> Result<(), ServerError>;
}
