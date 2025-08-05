use actix_web::{get, post, web, HttpResponse, Responder};
use crate::business::usecase::create_user::CreateUserUseCase;
use crate::business::services::user_service::UserService; // Tambahkan jika perlu

#[get("/users")]
pub async fn get_users() -> impl Responder {
    let users = UserService::list_users();
    HttpResponse::Ok().json(users)
}

#[post("/users")]
pub async fn create_user(form: web::Json<(String, String)>) -> impl Responder {
    let (name, email) = form.into_inner();
    let user = CreateUserUseCase::execute(name, email);
    HttpResponse::Ok().json(user)
}
