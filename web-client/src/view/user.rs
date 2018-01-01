use yew::html::Html;

use model::{MaybeLoading, Route, User};
use update::Msg;

impl User {
    pub fn render_name(&self) -> Html<Msg> {
        // TODO: https://github.com/rust-lang/rust/issues/25725
        Route::User(self.username.clone())
            .make_link(&self.username, None)
    }
}

impl<'a> MaybeLoading<&'a User> {
    pub fn render_name(&self) -> Option<Html<Msg>> {
        match *self {
            MaybeLoading::NotPresent => None,
            MaybeLoading::Loading => Some(html! {
                <span class=("bg-secondary", "text-white"),>
                    { "Loading..." }
                </span>
            }),
            MaybeLoading::Present(ref user) => Some(user.render_name()),
        }
    }
}
