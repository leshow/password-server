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
    let builder: MnemonicBuilder = MnemonicBuilder::new().expect("Failed to open wordlist.");

    let mut router = Router::new();
    router.get("/", handler);
    router.get("/generate", move |r: &mut Request| generate_handler(r, &builder));

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn generate_handler(_: &mut Request, builder: &MnemonicBuilder) -> IronResult<Response> {
    let mnemonic: Mnemonic = builder.create();

    Ok(Response::with((status::Ok, mnemonic.to_words(&builder.wordslist).join(" "))))
}
