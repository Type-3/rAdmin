use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[sql_type = "SmallInt"]
pub enum PasswordType {
    Argon2,
    Bcrypt,
}

impl<DB: Backend> ToSql<SmallInt, DB> for PasswordType
where
    i16: ToSql<SmallInt, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: io::Write,
    {
        let v = match *self {
            PasswordType::Argon2 => 1,
            PasswordType::Bcrypt => 2,
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<SmallInt, DB> for PasswordType
where
    i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let v = i16::from_sql(bytes)?;
        Ok(match v {
            1 => PasswordType::Argon2,
            2 => PasswordType::Bcrypt,
            num => return Err(format!("Invalid password type: {}", num).into()),
        })
    }
}

impl Default for PasswordType {
    fn default() -> PasswordType {
        PasswordType::Argon2
    }
}

impl fake::Dummy<fake::Faker> for PasswordType {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &fake::Faker, _rng: &mut R) -> Self {
        Self::default()
    }
}
