mod auth;
mod dashboard;
pub mod markdown;
mod navbar;
mod projects;

use yew::html::Html;

use model::{Model, Route};
use update::Msg;

pub fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            { navbar::navbar(model) }
            { model.route.render(model) }
        </div>
    }
}

pub trait Render {
    fn render(&self, model: &Model) -> Html<Msg>;
}

impl Render for Route {
    fn render(&self, model: &Model) -> Html<Msg> {
        match model.route {
            Route::Dashboard => dashboard::render(model),
            Route::Index => unimplemented!(),
            Route::Login => auth::login(model),
            Route::NotFound(ref path) => unimplemented!(),
            Route::Projects => projects::list(model),
            Route::Project(id) => unimplemented!(),
            Route::Register => auth::register(model),
            Route::Tag(ref tag) => unimplemented!(),
            Route::User(ref user) => unimplemented!(),
        }
    }
}
