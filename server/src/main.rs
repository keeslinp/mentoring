extern crate bcrypt;
extern crate bodyparser;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate iron;
#[macro_use]
extern crate log;
extern crate logger;
extern crate mount;
extern crate persistent;
extern crate pretty_env_logger;
#[macro_use]
extern crate router;
#[macro_use]
extern crate serde_derive;
extern crate staticfile;
extern crate void;

mod errors;
mod models;
mod routes_api;
mod routes_errors;
mod routes_static;
mod run;
mod schema;
mod util;

use std::process::exit;

use error_chain::ChainedError;
use void::unreachable;

use run::run;

fn main() {
    dotenv::dotenv().map_err(|err| eprintln!("{}", err)).ok();
    pretty_env_logger::init().unwrap();

    match run() {
        Ok(void) => unreachable(void),
        Err(err) => {
            error!("{}", err.display_chain());
            exit(1)
        }
    }
}
