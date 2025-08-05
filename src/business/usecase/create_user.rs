use crate::business::domain::user::User;
use crate::business::services::user_service::UserService;

pub struct CreateUserUseCase;

impl CreateUserUseCase {
    pub fn execute(name: String, email: String) -> User {
        UserService::create_user(name, email)
    }
}
