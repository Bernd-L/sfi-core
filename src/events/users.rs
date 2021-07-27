use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserSignup {
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserLogin {
    pub identifier: UserIdentifier,
    pub password: String,
    pub totp: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UserIdentifier {
    Uuid(Uuid),
    Name(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StatusNotice {
    pub status: String,
}
