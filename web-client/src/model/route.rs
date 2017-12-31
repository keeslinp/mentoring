use nom::IResult;
use stdweb::web::window;
use yew::html::{Html, Href};
use yew::virtual_dom::VNode;

use update::Msg;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Route {
    /// The current user's dashboard.
    Dashboard,

    /// The index page, which will redirect after the page loads.
    Index,

    /// The login page.
    Login,

    /// The 404 page.
    NotFound(String),

    /// A single project.
    Project(usize),

    /// A list of all projects.
    Projects,

    /// The register page.
    Register,

    /// A tag's page.
    Tag(String),

    /// A user's profile.
    User(String),
}

impl Route {
    /// Returns the current route.
    pub fn current() -> Route {
        match window().location() {
            Some(l) => l.hash().into(),
            None => Route::Index,
        }
    }

    /// Returns HTML corresponding to an `<a>` wrapping the given content, such
    /// that if clicked, the route will be changed to this one.
    pub fn make_link<T>(&self, class: &'static str, inner: T) -> Html<Msg>
        where T: Into<VNode<Msg>>,
    {
        let route = self.clone();
        html! {
            <a class=class, href=self,
               onclick=move |_| Msg::ChangeRoute(route.clone()),>
                { inner.into() }
            </a>
        }
    }

    /// Returns the hash associated with a route.
    pub fn path(&self) -> String {
        match *self {
            Route::Dashboard => "#me/dashboard".into(),
            Route::Index => "#".into(),
            Route::Login => "#login".into(),
            Route::NotFound(ref path) => format!("{}", path),
            Route::Project(id) => format!("#projects/{}", id),
            Route::Projects => "#projects".into(),
            Route::Register => "#register".into(),
            Route::Tag(ref tag) => format!("#tag/{}", tag),
            Route::User(ref user) => format!("#users/{}", user),
        }
    }
}

impl Default for Route {
    fn default() -> Route { Route::current() }
}

impl<'a> From<&'a str> for Route {
    fn from(s: &str) -> Route {
        match parse_route(s) {
            IResult::Done("", route) => route,
            _ => Route::NotFound(s.to_string()),
        }
    }
}

impl From<String> for Route {
    fn from(s: String) -> Route {
        match parse_route(&s) {
            IResult::Done("", route) => route,
            _ => Route::NotFound(s),
        }
    }
}

impl<'a> Into<Href> for &'a Route {
    fn into(self) -> Href {
        self.path().into()
    }
}

named!(parse_route(&str) -> Route, complete!(alt_complete!(
    parse_dashboard | parse_login | parse_profile | parse_projects |
    parse_register | parse_tag | parse_root)));
named!(parse_dashboard(&str) -> Route,
    map!(tag!("#me/dashboard"), |_| Route::Dashboard));
named!(parse_login(&str) -> Route, map!(tag!("#login"), |_| Route::Login));
named!(parse_profile(&str) -> Route, do_parse!(
    tag!("#users/") >>
    username: parse_name >>
    ( Route::User(username) )));
named!(parse_projects(&str) -> Route,
    map!(tag!("#projects"), |_| Route::Projects));
named!(parse_register(&str) -> Route,
    map!(tag!("#register"), |_| Route::Register));
named!(parse_root(&str) -> Route,
    map!(opt!(complete!(tag!("#"))), |_| Route::Index));
named!(parse_tag(&str) -> Route, do_parse!(
    tag!("#tag/") >>
    tag: parse_name >>
    ( Route::Tag(tag) )));

named!(parse_name(&str) -> String,
    map!(take_while1_s!(is_name_char), |s| s.to_string()));

fn is_name_char(c: char) -> bool {
    let n = c as u32;
    (0x30 <= n && n <= 0x39) || (0x41 <= n && n <= 0x5a) || (0x61 <= n && n <= 0x7a) || c == '-'
}
