use actix_web::{post, web, HttpResponse, Responder};
use crate::business::usecase::create_user::CreateUserUseCase;

#[post("/users")]
pub async fn create_user(form: web::Json<(String, String)>) -> impl Responder {
    let (name, email) = form.into_inner();
    let user = CreateUserUseCase::execute(name, email);
    HttpResponse::Ok().json(user)
}
