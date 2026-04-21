use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub fullname: String,
    pub role: String,
}

use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub role: String,
}

// #[derive(Deserialize)]
// pub struct LoginRequest {
//     pub username: String,
//     pub password: String,
// }

// #[derive(Serialize)]
// pub struct LoginResponse {
//     pub token: String,
//     pub user: UserResponse,
// }

// #[derive(Serialize)]
// pub struct UserResponse {
//     pub username: String,
//     pub fullname: String,
//     pub role: String,
// }