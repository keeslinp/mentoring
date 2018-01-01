use model::CurrentUser;

#[derive(Debug)]
pub struct User {
    pub username: String,
}

impl From<CurrentUser> for User {
    fn from(cu: CurrentUser) -> User {
        User {
            username: cu.username,
        }
    }
}
