#![allow(dead_code)]
// crates
use rand::{OsRng, Rng};
// lib
use mnemonic::Mnemonic;
// std
use std::fs::File;
use std::path::Path;
use std::io::{Read, Error};

static LENGTH: usize = 32;

pub struct MnemonicBuilder<'a> {
    pub wordslist: Vec<String>,
    seed: &'a str,
    bit_length: usize,
}

impl<'a> MnemonicBuilder<'a> {
    //
    pub fn new<'b>() -> Result<MnemonicBuilder<'a>, Error> {
        let str_seed: &str = "seed";
        let path = Path::new("src/wordslist/english.txt");
        let mut file = try!(File::open(&path));
        let mut string_from_file = String::new();

        try!(file.read_to_string(&mut string_from_file));
        let words: Vec<String> = string_from_file.split_whitespace()
                                                 .map(|s| s.to_owned())
                                                 .collect();

        Ok(MnemonicBuilder {
            seed: str_seed,
            wordslist: words,
            bit_length: LENGTH,
        })
    }

    pub fn with_seed(mut self, new_seed: &'a str) -> MnemonicBuilder<'a> {
        // MnemonicBuilder { seed: new_seed, .. self }
        self.seed = new_seed;
        self
    }

    pub fn with_words(mut self, new_wordslist: Vec<String>) -> MnemonicBuilder<'a> {
        // MnemonicBuilder { wordslist: new_wordslist, .. self }
        self.wordslist = new_wordslist;
        self
    }

    pub fn create(&self) -> Mnemonic {
        let mut rng = OsRng::new().unwrap();
        let random_chars: String = rng.gen_ascii_chars().take(self.bit_length).collect();

        Mnemonic::new(random_chars)
    }
}
