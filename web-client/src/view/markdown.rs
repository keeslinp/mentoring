//! TODO: This should become its own crate.

use pulldown_cmark::{Alignment, Event, OPTION_ENABLE_TABLES, Parser, Tag};
use yew::html::Html;
use yew::virtual_dom::{VNode, VTag, VText};

/// Renders a string of Markdown to HTML with the default options (footnotes
/// disabled, tables enabled).
pub fn render_markdown<M>(src: &str) -> Html<M> {
    let mut elems = vec![];
    let mut spine = vec![];

    macro_rules! add_child {
        ($child:expr) => {{
            let l = spine.len();
            assert_ne!(l, 0);
            spine[l-1].add_child($child);
        }}
    }

    for ev in Parser::new_ext(src, OPTION_ENABLE_TABLES) {
        match ev {
            Event::Start(tag) => {
                spine.push(make_tag(tag));
            }
            Event::End(tag) => {
                // TODO Verify stack end.
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap();
                if let Tag::CodeBlock(_) = tag {
                    let mut pre = VTag::new("pre");
                    pre.add_child(top.into());
                    top = pre;
                } else if let Tag::Table(aligns) = tag {
                    for r in top.childs.iter_mut() {
                        if let &mut VNode::VTag { ref mut vtag, .. } = r {
                            for (i, c) in vtag.childs.iter_mut().enumerate() {
                                if let &mut VNode::VTag { ref mut vtag, .. } = c {
                                    match aligns[i] {
                                        Alignment::None => {},
                                        Alignment::Left => vtag.add_classes("text-left"),
                                        Alignment::Center => vtag.add_classes("text-center"),
                                        Alignment::Right => vtag.add_classes("text-right"),
                                    }
                                }
                            }
                        }
                    }
                // TODO: https://github.com/DenisKolodin/yew/issues/67
                /*
                } else if let Tag::TableHead = tag {
                    for c in top.childs.iter_mut() {
                        if let &mut VNode::VTag { ref mut vtag, .. } = c {
                            vtag.tag = "th".into();
                            vtag.add_attribute("scope", "col");
                        }
                    }
                */
                }
                if l == 1 {
                    elems.push(top);
                } else {
                    spine[l-2].add_child(top.into());
                }
            }
            Event::Text(text) => add_child!(VText::new(text).into()),
            Event::SoftBreak => add_child!(VText::new("\n").into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),
            _ => println!("Unknown event: {:#?}", ev),
        }
    }

    if elems.len() == 1 {
        elems.pop().unwrap()
    } else {
        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

fn make_tag<M>(t: Tag) -> VTag<M> {
    match t {
        Tag::Paragraph => VTag::new("p"),
        Tag::Rule => VTag::new("hr"),
        Tag::Header(n) => {
            assert!(n > 0);
            assert!(n < 7);
            VTag::new(format!("h{}", n))
        }
        Tag::BlockQuote => {
            let mut el = VTag::new("blockquote");
            el.add_classes("blockquote");
            el
        }
        Tag::CodeBlock(lang) => {
            let mut el = VTag::new("code");
            if lang != "" {
                unimplemented!()
            }
            el
        }
        Tag::List(None) => VTag::new("ul"),
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start);
            el
        }
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => {
            let mut el = VTag::new("table");
            el.add_classes("table");
            el
        }
        Tag::TableHead => VTag::new("tr"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => {
            let mut el = VTag::new("span");
            el.add_classes("font-italic");
            el
        }
        Tag::Strong => {
            let mut el = VTag::new("span");
            el.add_classes("font-weight-bold");
            el
        }
        Tag::Code => VTag::new("code"),
        Tag::Link(href, title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href);
            if title != "" {
                el.add_attribute("title", title);
            }
            el
        }
        Tag::Image(src, title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src);
            if title != "" {
                el.add_attribute("title", title);
            }
            el
        }
        _ => unimplemented!("tag {:?}", t)
    }
}
