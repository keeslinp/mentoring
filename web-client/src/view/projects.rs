use yew::html::Html;

use model::Model;
use update::Msg;
use view::Render;

pub fn list(model: &Model) -> Html<Msg> {
    let project_cards = model.projects.iter()
        .map(|p| p.render(model));
    html! {
        <main class="container",>
            { for project_cards }
        </main>
    }
}
