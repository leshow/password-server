use mnemonic::Mnemonic;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::Read;

pub struct MnemonicBuilder <'a> {
    seed: &'a str,
    wordslist: Vec<String>
}

impl <'a> MnemonicBuilder <'a> {
    fn new<'b>() -> MnemonicBuilder<'a> {
        let str_seed: &str = "seed";
        let path = Path::new("src/wordslist/english.txt");
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
            Ok(file) => file
        };
        let mut string_from_file = String::new();

        match file.read_to_string(&mut string_from_file) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                       Error::description(&why)),
            Ok(_) => println!("read to string_from_file"),
        };
        let words: Vec<String> = string_from_file.split_whitespace().map(|s| s.to_owned()).collect();

        MnemonicBuilder { seed: str_seed, wordslist: words }
    }
}
