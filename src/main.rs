extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", handler);
    router.get("/generate", generate_handler);

    Iron::new(router).http("localhost:3000").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn generate_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok)))
}
