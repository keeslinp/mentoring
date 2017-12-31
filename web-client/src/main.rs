extern crate pulldown_cmark;
#[macro_use]
extern crate nom;
extern crate regex;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

mod model;
mod update;
mod view;

use yew::html::program;

use model::Model;
use update::update;
use view::view;

fn main() {
    stdweb::initialize();
    let mut model = Model::default();
    model.projects = vec![
        model::Project {
            comments: Vec::new(),
            description: "\
1.  List item one.

    List item one continued with a second paragraph followed by an
    Indented block.

        $ ls *.sh
        $ mv *.sh ~/tmp

    List item continued with a third paragraph.

2.  List item two continued with an open block.

    This paragraph is part of the preceding list item.

    1. This list is nested and does not require explicit item continuation.

       This paragraph is part of the preceding list item.

    2. List item b.

    This paragraph belongs to item two of the outer list.

# one
## two
### three

> a man
> a plan
> a canal
> panama

*Footnotes* and **Tables**

asdf [^qwerty]

[^qwerty]: zxcvbn

| Tables        | Are           | Cool  |
| ------------- |:-------------:| -----:|
| col 3 is      | right-aligned | $1600 |
| col 2 is      | centered      |   $12 |
| zebra stripes | are neat      |    $1 |

---

at least I can `code` inline?

[imgur](imgur.com)

![cat](https://i.imgur.com/nz8Sh.gif)

#### end".to_string(),
            name: "foo some bars!".to_string(),
            mentor: None,
            mentee: None,
        }
    ];
    program(model, update, view);
}
