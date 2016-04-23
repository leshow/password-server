extern crate iron;
extern crate router;
extern crate logger;
extern crate staticfile;
extern crate mount;

extern crate lib;

use iron::prelude::*;
use iron::status;
use router::Router;
use logger::Logger;
use staticfile::Static;
use mount::Mount;

use lib::{MnemonicBuilder, Mnemonic};

use std::path::Path;

fn main() {
    let (logger_before, logger_after) = Logger::new(None);
    let builder: MnemonicBuilder = MnemonicBuilder::new().expect("Failed to open wordlist.");

    // set up routes
    let mut router = Router::new();
    router.get("/generate",
               move |r: &mut Request| generate_handler(r, &builder));

    // add logger middleware
    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
    // set up mount for static file serving
    let mut mounts = Mount::new();
    mounts.mount("/", chain)
          .mount("/public",
                 Static::new(Path::new("frontend/dist/index.html")));

    Iron::new(mounts).http("localhost:3000").unwrap();
}

fn generate_handler(_: &mut Request, builder: &MnemonicBuilder) -> IronResult<Response> {
    let mnemonic: Mnemonic = builder.create();

    Ok(Response::with((status::Ok, mnemonic.to_words(&builder.wordslist).join(" "))))
}
