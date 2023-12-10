use hyper::{Body, Request, Response, Server, StatusCode};
use routerify::{Middleware, RequestInfo, Router, RouterService};
use std::fs;
use std::net::SocketAddr;
mod controllers;
mod middlewares;

async fn home_handler(_: Request<Body>) -> Result<Response<Body>, routerify_json_response::Error> {
  let index_page = fs::read_to_string("views/index.html")
    .expect("Should have been able to read views/index.html");

  Ok(Response::new(Body::from(index_page)))
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
  eprintln!("{}", err);
  Response::builder()
    .status(StatusCode::INTERNAL_SERVER_ERROR)
    .body(Body::from(format!("Something went wrong: {}", err)))
    .unwrap()
}

fn router() -> Router<Body, routerify_json_response::Error> {
  Router::builder()
    .middleware(Middleware::pre(middlewares::logger::logger))
    .get("/", home_handler)
    .post(
        "/api/send-magic-packet",
        controllers::magic::magic_packet_handler,
    )
    .err_handler_with_info(error_handler)
    .build()
    .unwrap()
}

#[tokio::main]
async fn main() {
  let router = router();
  let service = RouterService::new(router).unwrap();
  let addr = SocketAddr::from(([0, 0, 0, 0], 5644));
  let server = Server::bind(&addr).serve(service);

  println!("App is running on: {}", addr);
  if let Err(err) = server.await {
    eprintln!("Server error: {}", err);
  }
}
