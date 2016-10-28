extern crate iron;
extern crate router;
extern crate logger;

extern crate lib;


use std::io;
use std::fmt::{self, Debug, Display};
use std::error::Error;
use std::sync::Arc;

use iron::prelude::*;
use iron::status;
use router::Router;
use logger::Logger;

use lib::{MnemonicBuilder, Mnemonic};

fn main() {
    let (logger_before, logger_after) = Logger::new(None);
    let builder = Arc::new(MnemonicBuilder::new().expect("Failed to open wordlist."));

    // set up routes
    let mut router = Router::new();

    let builder1 = builder.clone();
    router.get("/passphrase",
               move |r: &mut Request| passphrase_handler(r, &builder1),
               "passphrase");

    let builder2 = builder.clone();
    router.get("/passraw",
               move |r: &mut Request| passraw_handler(r, &builder2),
               "passraw");

    let builder3 = builder.clone();
    router.get("/passphrase/:num",
               move |r: &mut Request| passphrase_sized_handler(r, &builder3),
               "passnum");

    // add logger middleware
    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn passphrase_handler(_: &mut Request, builder: &MnemonicBuilder) -> IronResult<Response> {
    let mnemonic: Result<Mnemonic, io::Error> = builder.create();

    match mnemonic {
        Ok(x) => Ok(Response::with((status::Ok, x.to_json(&builder.wordslist)))),
        Err(x) => Err(IronError::new(StringError(x.to_string()), (status::BadRequest, "Error"))),
    }
}

fn passraw_handler(_: &mut Request, builder: &MnemonicBuilder) -> IronResult<Response> {
    let mnemonic: Result<Mnemonic, io::Error> = builder.create();

    match mnemonic {
        Ok(x) => Ok(Response::with((status::Ok, x.to_words(&builder.wordslist).join(" ")))),
        Err(x) => Err(IronError::new(StringError(x.to_string()), (status::BadRequest, "Error"))),
    }
}

fn passphrase_sized_handler(req: &mut Request, builder: &MnemonicBuilder) -> IronResult<Response> {
    let num_str = req.extensions.get::<Router>().unwrap().find("num").unwrap();
    let num = num_str.parse::<usize>().unwrap_or(24);
    let mnemonic: Result<Mnemonic, io::Error> = builder.create();

    match mnemonic {
        Ok(x) => {
            Ok(Response::with((status::Ok,
                               format!("{{\"passphrase\": \"{}\"}}",
                                       x.to_words(&builder.wordslist)[0..num].join(" ")))))
        }
        Err(x) => Err(IronError::new(StringError(x.to_string()), (status::BadRequest, "Error"))),
    }
}

#[derive(Debug)]
struct StringError(String);

impl Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str {
        &*self.0
    }
}
