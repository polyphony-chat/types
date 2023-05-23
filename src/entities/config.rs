use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "sqlx")]
use sqlx::FromRow;

use crate::errors::Error;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
pub struct ConfigEntity {
    pub key: String,
    pub value: Option<Value>,
}

impl ConfigEntity {
    pub fn as_string(&self) -> Option<String> {
        let Some(v) = self.value.as_ref() else {
            return None;
        };
        Some(v.as_str().expect("value is not a string").to_string())
    }

    pub fn as_bool(&self) -> Option<bool> {
        let Some(v) = self.value.as_ref() else {
            return None;
        };
        Some(v.as_bool().expect("value is not a boolean"))
    }

    pub fn as_int(&self) -> Option<i64> {
        let Some(v) = self.value.as_ref() else {
            return None;
        };
        Some(v.as_i64().expect("value is not a number"))
    }
}

#[cfg(feature = "sqlx")]
impl ConfigEntity {
    pub async fn get_by_key(conn: &mut sqlx::MySqlConnection, key: &str) -> Result<Self, Error> {
        sqlx::query_as("SELECT * FROM config WHERE `key` = ?")
            .bind(key)
            .fetch_one(conn)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn collect(conn: &mut sqlx::MySqlConnection) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM config")
            .fetch_all(conn)
            .await
            .map_err(Error::SQLX)
    }
}
