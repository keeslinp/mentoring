mod comment;
mod current_user;
mod project;
mod route;
mod tag;
mod user;

pub use self::comment::Comment;
pub use self::current_user::CurrentUser;
pub use self::project::Project;
pub use self::route::Route;
pub use self::tag::Tag;
pub use self::user::User;

#[derive(Debug, Default)]
pub struct Model {
    pub current_user: Option<CurrentUser>,
    pub projects: Vec<Project>,
    pub route: Route,
    pub update_pending: bool,
}

impl Model {
    pub fn index(&self) -> Route {
        if self.current_user.is_some() {
            Route::Dashboard
        } else {
            Route::Projects
        }
    }
}
