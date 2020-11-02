use validator::ValidationErrors;

pub trait Validatable: Sized {
    fn validate(self) -> Result<Self, ValidationErrors>;
}
