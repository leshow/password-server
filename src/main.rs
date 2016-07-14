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

    // set up routes
    let mut router = Router::new();
    router.get("/mnemonic",
               move |r: &mut Request| generate_handler(r, &builder));
    router.get("/mnemonic/:num",
               move |r: &mut Request| generate_length_handler(r, &builder));

    // add logger middleware
    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn generate_handler(_: &mut Request, builder: &MnemonicBuilder) -> IronResult<Response> {
    let mnemonic: Mnemonic = builder.create().unwrap();

    Ok(Response::with((status::Ok, mnemonic.to_words(&builder.wordslist).join(" "))))
}

fn generate_length_handler(req: &mut Request, builder: &MnemonicBuilder) -> IronResult<Response> {
    // let ref num: u8 = req.extensions.get::<Router>().unwrap().find("num").unwrap_or(12);
    let num = 8usize;
    let mnemonic: Mnemonic = builder.create().unwrap();

    Ok(Response::with((status::Ok,
                       mnemonic.to_words(&builder.wordslist)
        .iter()
        .take(num)
        .fold(String::new(), |acc, &word| format!("{} {}", acc, word)))))
}
