use std::net::{IpAddr, Ipv6Addr};
use std::thread::yield_now;

use iron::prelude::*;
use logger::{Format, Logger};
use mount::Mount;
use void::Void;

use errors::{ErrorKind, Result, ResultExt};
use routes_api::handler as api_handler;
use routes_static::handler as static_handler;
use util::{getenv, getenv_parse};

pub fn run() -> Result<Void> {
    let host =
        getenv_parse("HTTP_HOST")?.unwrap_or(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)));
    let port = getenv_parse("HTTP_PORT")?.unwrap_or(8000);

    let log_format = getenv("LOG_FORMAT")?.and_then(|s| Format::new(&s));
    let (log_before, log_after) = Logger::new(log_format);

    let mut mount = Mount::new();
    mount
        .mount("/api/", api_handler())
        .mount("/", static_handler()?);

    let mut chain = Chain::new(mount);
    chain.link_before(log_before);
    chain.link_after(log_after);

    Iron::new(chain)
        .http((host, port))
        .chain_err(|| ErrorKind::CouldntStartServer)?;
    loop {
        yield_now();
    }
}
