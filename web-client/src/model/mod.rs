mod comment;
mod current_user;
mod project;
mod route;
mod user;

use std::collections::HashMap;

pub use self::comment::Comment;
pub use self::current_user::CurrentUser;
pub use self::project::Project;
pub use self::route::Route;
pub use self::user::User;

#[derive(Debug, Default)]
pub struct Model {
    pub current_user: Option<CurrentUser>,
    pub projects: HashMap<usize, Project>,
    pub route: Route,
    pub update_pending: bool,
    pub users: HashMap<usize, User>,
}

impl Model {
    pub fn find_user(&self, uid: usize) -> Option<&User> {
        self.users.get(&uid)
    }

    pub fn index(&self) -> Route {
        if self.current_user.is_some() {
            Route::Dashboard
        } else {
            Route::Projects
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaybeLoading<T> {
    NotPresent,
    Loading,
    Present(T),
}
