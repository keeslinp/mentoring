use bodyparser;
use iron::prelude::*;
use iron::status;

#[derive(Clone, Debug, Deserialize)]
struct LoginBody {
    username: String,
    password: String,
}

pub fn login(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Struct<LoginBody>>();
    info!("{:?}", body);
    Ok(Response::with(status::NotImplemented))
}
