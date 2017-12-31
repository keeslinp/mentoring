use yew::html::Html;

use model::{Model, Route};
use update::Msg;
use view::Render;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tag {
    pub name: String,
}

impl Tag {
    /// Creates a new Tag. name must be composed of only ASCII alphanumerics
    /// and dashes.
    pub fn new(name: String) -> Tag {
        Tag { name }
    }
}

impl Render for Tag {
    fn render(&self, _model: &Model) -> Html<Msg> {
        Route::Tag(self.name.clone()).make_link("Tag", &self.name)
    }
}
