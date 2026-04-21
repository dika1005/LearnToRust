use crate::entities::users;
use sea_orm::*;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_username(db: &DatabaseConnection, username: &str) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(db)
            .await
    }

pub async fn create_user(db: &DatabaseConnection, data: users::ActiveModel) -> Result<users::Model, DbErr> {
        // exec_with_returning akan mengembalikan Model setelah insert
        data.insert(db).await
    }
}