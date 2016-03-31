extern crate iron;
extern crate router;
extern crate logger;

extern crate lib;

use iron::prelude::*;
use iron::status;
use router::Router;
use logger::Logger;

use lib::{MnemonicBuilder, Mnemonic};

fn main() {
    let (logger_before, logger_after) = Logger::new(None);

    let mut router = Router::new();
    router.get("/", handler);
    router.get("/generate", generate_handler);

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn generate_handler(_: &mut Request) -> IronResult<Response> {
    let mnemonic: Mnemonic = MnemonicBuilder::new().create();

    Ok(Response::with((status::Ok, mnemonic.binary_hash)))
}
