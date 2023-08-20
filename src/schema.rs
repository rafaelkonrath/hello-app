use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: String,
}

