use hyper::{Body, Request};
use routerify::prelude::*;

pub async fn logger(req: Request<Body>) -> Result<Request<Body>, routerify_json_response::Error> {
  println!(
    "{} {} {}",
    req.remote_addr(),
    req.method(),
    req.uri().path()
  );
  Ok(req)
}
