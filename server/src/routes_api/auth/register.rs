use iron::prelude::*;
use iron::status;

pub fn register(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::NotImplemented))
}
