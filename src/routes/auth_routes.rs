use actix_web::{web, HttpResponse, Responder};
use crate::app_state::AppState;
use crate::dtos::auth_dto::{RegisterRequest, UserResponse};
use crate::dtos::api_response::ApiResponse;
use crate::services::auth_service::AuthService;

pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> impl Responder {
    match AuthService::register_user(&state.db, body.into_inner()).await {
        Ok(user_data) => {
            let res = ApiResponse {
                status: "success".to_string(),
                message: "User created successfully".to_string(),
                data: Some(user_data),
            };
            HttpResponse::Created().json(res)
        },
        Err(err_msg) => {
            let res: ApiResponse<()> = ApiResponse {
                status: "error".to_string(),
                message: err_msg,
                data: None,
            };
            HttpResponse::BadRequest().json(res)
        }
    }
}