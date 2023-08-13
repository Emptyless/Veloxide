use crate::domain::User;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UserData {
    pub user_id: Uuid,
    pub user_email: String,
    // TODO: Add user roles / permissions here.
}

impl From<User> for UserData {
    fn from(user: User) -> Self {
        UserData {
            user_id: user.id,
            user_email: user.email,
        }
    }
}
