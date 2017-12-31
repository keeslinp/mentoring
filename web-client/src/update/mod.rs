mod msg;

use yew::html::Context;

use model::Model;
pub use self::msg::Msg;

pub fn update(_ctx: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    println!("{:#?}", msg);
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
        Msg::UpdateNotify => {
            model.update_pending = true;
        }
    }
}
