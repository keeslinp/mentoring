use chrono::Local;
use chrono_humanize::Humanize;
use yew::html::Html;

use model::{Model, Project, Route};
use update::Msg;
use view::markdown::render_markdown;

impl Model {
    pub fn render_project_card(&self, id: usize) -> Html<Msg> {
        let project = &self.projects[&id];
        // TODO: https://github.com/rust-lang/rust/issues/25725
        let title = Route::Project(id)
            .make_link(&project.name, None);

        let tags = project.tags.iter().map(|tag| {
            // TODO: https://github.com/rust-lang/rust/issues/25725
            Route::Tag(tag.clone())
                .make_link(format!("#{}", tag), Some("card-link"))
        });

        html! {
            <div class="card",>
                <div class="card-body",>
                    <h5 class="card-title",>
                        { title }
                    </h5>
                    <h6 class=("card-subtitle", "mb-2", "text-muted"),>
                        { project.comments.len() }
                        { " comments" }
                        <br/>
                        { project.render_creator(self) }
                        { ", " }
                        { project.render_mentor(self) }
                        { ", " }
                        { project.render_developer(self) }
                    </h6>
                    <div class="card-text",>
                        { render_markdown(&project.short_description) }
                    </div>
                    { for tags }
                </div>
            </div>
        }
    }

    pub fn render_project_list(&self) -> Html<Msg> {
        let project_cards = self.projects.keys()
            .map(|&id| self.render_project_card(id));
        html! {
            <main class="container",>
                { for project_cards }
            </main>
        }
    }
}

impl Project {
    pub fn render_creator(&self, model: &Model) -> Html<Msg> {
        if let Some(ref creator) = self.creator(model) {
            html! {
                <span>
                    { "Created " }
                    { self.created.with_timezone(&Local).humanize() }
                    { " by " }
                    { creator.render_name() }
                    { "." }
                </span>
            }
        } else {
            warn!("Couldn't find creator ({})", self.creator_id);
            html! {
                { "Creator not found." }
            }
        }
    }

    pub fn render_developer(&self, model: &Model) -> Html<Msg> {
        if let Some(name) = self.developer(model).render_name() {
            html! {
                <span>
                    { "Developed by " }
                    { name }
                    { "." }
                </span>
            }
        } else {
            html! {
                <span>{ "No developer." }</span>
            }
        }
    }

    pub fn render_mentor(&self, model: &Model) -> Html<Msg> {
        if let Some(name) = self.mentor(model).render_name() {
            html! {
                <span>
                    { "Mentored by " }
                    { name }
                    { "." }
                </span>
            }
        } else {
            html! {
                <span>{ "No mentor." }</span>
            }
        }
    }

    pub fn render_page(&self, model: &Model) -> Html<Msg> {
        html! {
            <main class="container",>
                <div class="jumbotron",>
                    <h1>{ &self.name }</h1>
                    <p class="text-muted",>
                        { self.render_creator(model) }
                        { " " }
                        { self.render_mentor(model) }
                        { " " }
                        { self.render_developer(model) }
                    </p>
                    { render_markdown(&self.long_description) }
                </div>
            </main>
        }
    }
}
