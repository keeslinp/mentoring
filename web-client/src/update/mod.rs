mod msg;

use std::sync::Mutex;

use stdweb::web::{EventListenerHandle, IEventTarget, window};
use stdweb::web::event::HashChangeEvent;
use url::Url;
use yew::html::Context;

use model::Model;
pub use self::msg::Msg;

lazy_static! {
    static ref HASH_CHANGE_LISTENER: Mutex<Option<EventListenerHandle>> = Mutex::new(None);
}

pub fn update(ctx: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    {
        let mut hcl = HASH_CHANGE_LISTENER.lock().unwrap();
        if hcl.is_none() {
            let mut sender = ctx.sender();
            *hcl = Some(window().add_event_listener(move |ev: HashChangeEvent| {
                match Url::parse(&ev.new_url()) {
                    Ok(url) => {
                        let frag = url.fragment().unwrap_or_default();
                        let hash = format!("#{}", frag);
                        sender.send(Msg::HashChange(hash))
                    },
                    Err(err) => error!("{}", err),
                }
            }));
        }
    }
    println!("{:?}", msg);
    match msg {
        Msg::ChangeRoute(route) => {
            js! {
                window.location.hash = @{route.path()};
            }
            if model.update_pending {
                unimplemented!()
            } else {
                model.route = route;
            }
        }
        Msg::HashChange(hash) => {
            ctx.sender().send(Msg::ChangeRoute(hash.into()));
        }
        Msg::UpdateNotify => {
            model.update_pending = true;
        }
    }
}
