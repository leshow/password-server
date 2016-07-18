#![feature(plugin)]

#![plugin(clippy)]

extern crate crypto;
extern crate rustc_serialize;
extern crate rand;

pub use mnemonicbuilder::MnemonicBuilder;
pub use mnemonic::Mnemonic;

mod mnemonicbuilder;
mod mnemonic;
