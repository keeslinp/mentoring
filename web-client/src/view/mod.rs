mod auth;
mod dashboard;
pub mod markdown;
mod navbar;
mod projects;
mod route;
mod user;

use yew::html::Html;

use model::{Model, Route};
use update::Msg;

pub fn view(model: &Model) -> Html<Msg> {
    let main = match model.route {
        Route::Dashboard => dashboard::render(model),
        Route::Index => html! {
            <h1>{ "Loading..." }</h1>
        },
        Route::Login => auth::login(model),
        Route::NotFound(ref path) => panic!("TODO 404"),
        Route::Projects => model.render_project_list(),
        Route::Project(id) => if let Some(project) = model.projects.get(&id) {
            project.render_page(model)
        } else {
            panic!("TODO 404")
        },
        Route::Register => auth::register(model),
        Route::Tag(ref tag) => unimplemented!(),
        Route::User(ref user) => unimplemented!(),
    };

    html! {
        <div>
            { navbar::navbar(model) }
            { main }
        </div>
    }
}
