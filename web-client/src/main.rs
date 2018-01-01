extern crate chrono;
extern crate chrono_humanize;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate pulldown_cmark;
extern crate regex;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
extern crate url;
#[macro_use]
extern crate yew;

mod model;
mod update;
mod view;

use std::collections::HashMap;

use chrono::{Duration, Utc};
use yew::html::program;

use model::{Model, Project, User};
use update::update;
use view::view;

fn main() {
    stdweb::initialize();

    let mut model = Model::default();
    model.projects = HashMap::new();
    model.projects.insert(0, Project {
        comments: Vec::new(),
        created: Utc::now() - Duration::weeks(1),
        creator_id: 0,
        developer_id: None,
        name: "Mentoring Site".to_string(),
        long_description: "This is the long description.

Basically, make a site to keep track of side projects and offer mentoring.".to_string(),
        mentor_id: None,
        short_description: "This site!".to_string(),
        tags: vec![
            "postgresql".to_string(),
            "webdev".to_string(),
        ],
    });
    model.users = HashMap::new();
    model.users.insert(0, User {
        username: "foobar".to_string(),
    });
    program(model, update, view);
}
