use yew::html::Html;

use model::{Comment, Model, User};
use update::Msg;
use view::Render;
use view::markdown::render_markdown;

#[derive(Debug)]
pub struct Project {
    pub comments: Vec<Comment>,
    pub description: String,
    pub name: String,
    pub mentor: Option<User>,
    pub mentee: Option<User>,
}

impl Render for Project {
    fn render(&self, model: &Model) -> Html<Msg> {
        let mentor = if let Some(ref mentor) = self.mentor {
            html! {
                <span>
                    { "Mentored by " }
                    { mentor.render(model) }
                </span>
            }
        } else {
            html! {
                <span>{ "No mentor" }</span>
            }
        };
        let mentee = if let Some(ref mentee) = self.mentee {
            unimplemented!() // TODO Wording
        } else {
            html! {
                <span>{ "No mentee" }</span>
            }
        };

        html! {
            <div class="card",>
                <div class="card-body",>
                    <h5 class="card-title",>
                        { &self.name }
                    </h5>
                    <h6 class=("card-subtitle", "mb-2", "text-muted"),>
                        { self.comments.len() }
                        { " comments, " }
                        { mentor }
                        { ", " }
                        { mentee }
                    </h6>
                    <div class="card-text",>
                        { render_markdown(&self.description) }
                    </div>
                </div>
            </div>
        }
    }
}
