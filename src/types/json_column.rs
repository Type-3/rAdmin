use diesel::sql_types::Jsonb;
use diesel::pg::Pg;
use serde::{Serialize, de::DeserializeOwned, Serializer, Deserializer, Deserialize};
use diesel::deserialize::FromSql;
use diesel::serialize::{ToSql, Output};

use std::fmt;

#[derive(AsExpression)]
#[sql_type = "Jsonb"]
pub struct JsonColumn<T: Serialize>(T);

impl <T: Serialize + for<'de> Deserialize<'de>> AsRef<T> for JsonColumn<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl <T: Serialize + DeserializeOwned> AsMut<T> for JsonColumn<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl <T: Serialize + for<'de> Deserialize<'de>>ToSql<Jsonb, Pg> for JsonColumn<T> {
    fn to_sql<W>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result
    where
        W: std::io::Write,
    {
        let value = serde_json::to_value(&self.0).unwrap();
         ToSql::<Jsonb, Pg>::to_sql::<W>(&value, out)
    }
}

impl <T: Serialize + for<'de> Deserialize<'de>>FromSql<Jsonb, Pg> for JsonColumn<T> {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        let v = FromSql::<Jsonb, Pg>::from_sql(bytes)?;
        Ok(JsonColumn(serde_json::from_value(v).unwrap()))
    }
}

impl <T: Serialize + for<'de> Deserialize<'de>>fmt::Debug for JsonColumn<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.0).unwrap())?;
        Ok(())
    }
}

impl <T: Serialize + for<'de> Deserialize<'de>>Serialize for JsonColumn<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        self.0.serialize(serializer)
    }
}

impl <'de, T: Serialize + Deserialize<'de>> Deserialize<'de> for JsonColumn<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        Ok(JsonColumn(T::deserialize(deserializer)?))
    }
}

impl <T: Clone + Serialize + for <'de> Deserialize<'de>> Clone for JsonColumn<T> {
    fn clone(&self) -> Self {
        JsonColumn(self.0.clone())
    }
}

impl <T: Serialize + for<'de> Deserialize<'de> + PartialEq>PartialEq for JsonColumn<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}