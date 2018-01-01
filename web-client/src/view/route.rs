use yew::html::Html;
use yew::html::onclick::Wrapper as OnClick;
use yew::virtual_dom::{VNode, VTag};

use model::Route;
use update::Msg;

impl Route {
    /// Returns HTML corresponding to an `<a>` wrapping the given content, such
    /// that if clicked, the route will be changed to this one.
    pub fn make_link<'a, II, T>(&self, inner: T, classes: II) -> Html<Msg>
        where II: IntoIterator<Item=&'a str>,
              T: Into<VNode<Msg>>,
    {
        let route = self.clone();
        let mut tag = VTag::new("a");
        for class in classes {
            tag.add_classes(class);
        }
        tag.add_attribute("href", self.path());
        tag.add_listener(Box::new(OnClick::from(move |_| {
            Msg::ChangeRoute(route.clone())
        })));
        tag.add_child(inner.into());
        tag.into()
    }
}
