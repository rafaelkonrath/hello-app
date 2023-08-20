use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UsersModel {
    pub username: String,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: String,
}

#[derive(Debug, FromRow, Serialize)]
#[allow(non_snake_case)]
pub struct UserNameModel {
    pub username: String,
}
