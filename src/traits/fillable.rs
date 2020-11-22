use crate::ServerError;

pub trait Fillable<T> {
    fn fill(self, obj: &mut T) -> Result<(), ServerError>;
}
