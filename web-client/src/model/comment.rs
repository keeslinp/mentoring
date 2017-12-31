use model::User;

#[derive(Debug)]
pub struct Comment {
    pub author: User,
    pub contents: String,
}
