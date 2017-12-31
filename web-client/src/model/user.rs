use yew::html::Html;

use model::{CurrentUser, Model, Route};
use update::Msg;
use view::Render;

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

impl Render for User {
    fn render(&self, model: &Model) -> Html<Msg> {
        Route::User(self.username.clone())
            .make_link("User", &self.username)
    }
}
