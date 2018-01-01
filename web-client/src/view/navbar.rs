use yew::html::Html;

use model::{Model, Route};
use update::Msg;

pub fn navbar(model: &Model) -> Html<Msg> {
    let mut links = vec![];
    links.push(navbar_item(model, Route::Projects, "Projects"));
    if model.current_user.is_some() {
        links.push(navbar_item(model, Route::Dashboard, "Dashboard"));
    }

    html! {
        <nav class=("navbar", "navbar-expand-sm", "navbar-light", "bg-light"),>
            // TODO: https://github.com/rust-lang/rust/issues/25725
            { model.index().make_link("mentoring.acm.umn.edu", Some("navbar-brand")) }
            <div class="navbar-collapse", id="navbarNav",>
                <ul class="navbar-nav",>
                    { for links.into_iter() }
                </ul>
            </div>
            { navbar_user(model) }
        </nav>
    }
}

fn navbar_item(model: &Model, route: Route, name: &str) -> Html<Msg> {
    // TODO: https://github.com/rust-lang/rust/issues/25725
    let a = route.make_link(name, Some("nav-link"));
    if model.route == route {
        html! {
            <li class=("nav-item", "active"),>{ a }</li>
        }
    } else {
        html! {
            <li class="nav-item",>{ a }</li>
        }
    }
}

fn navbar_user(model: &Model) -> Html<Msg> {
    if let Some(ref current_user) = model.current_user {
        // TODO Make this a dropdown.
        let route = Route::User(current_user.username.to_string());
        navbar_item(model, route, &current_user.username)
    } else {
        let login = navbar_item(model, Route::Login, "Log In");
        let register = navbar_item(model, Route::Register, "Register");
        html! {
            <ul class=("navbar-nav", "float-right"),>
                { login }
                { register }
            </ul>
        }
    }
}
