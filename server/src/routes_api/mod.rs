use router::Router;

mod auth;

pub fn handler() -> Router {
    router! {
        login:    post "/login"    => auth::login,
        register: post "/register" => auth::register,
    }
}
