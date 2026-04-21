use crate::repositories::user_repository::UserRepository;
use crate::entities::users;
use crate::dtos::auth_dto::{RegisterRequest, UserResponse};
use sea_orm::*;
use bcrypt::{hash, DEFAULT_COST};

pub struct AuthService;

impl AuthService {
    pub async fn register_user(db: &DatabaseConnection, req: RegisterRequest) -> Result<UserResponse, String> {
        let existing_user = UserRepository::find_by_username(db, &req.username)
            .await
            .map_err(|e| e.to_string());
        
        if existing_user?.is_some() {
            return Err("Username sudah terdaftar".to_string());
        }

        let hashed_password = hash(req.password, DEFAULT_COST)
            .map_err(|_| "Gagal melakukan hashing password".to_string())?;

        let new_user = users::ActiveModel {
            username: Set(req.username),
            password: Set(hashed_password),
            fullname: Set(req.fullname),
            role: Set(req.role),
            ..Default::default()
        };

        let saved_user = UserRepository::create_user(db, new_user)
            .await
            .map_err(|e| e.to_string())?;

        // Mapping dari Entity Model ke UserResponse DTO
        Ok(UserResponse {
            id: saved_user.id,
            username: saved_user.username,
            fullname: saved_user.fullname,
            role: saved_user.role,
        })
    }
}