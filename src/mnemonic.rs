use crypto::pbkdf2::pbkdf2;
use crypto::sha2::{Sha256, Sha512};
use crypto::hmac::Hmac;
use crypto::digest::Digest;

use rustc_serialize::hex::FromHex;

use nom::IResult;

static PBKDF2_ROUNDS: u32 = 2048;
static PBKDF2_KEY_LEN: usize = 64;

pub struct Mnemonic {
    pub mnemonic: Vec<u8>,
}

impl Mnemonic {
    pub fn new(chars: &str) -> Mnemonic {
        let h = Mnemonic::gen_sha256(&chars).from_hex().unwrap();
        let length = chars.len() / 32;

        Mnemonic { mnemonic: [chars.as_ref(), &h[..length]].concat() }
    }

    pub fn to_seed(&self, mnemonic: &str, seed_value: &str) -> Vec<u8> {
        let mut mac = Hmac::new(Sha512::new(), mnemonic.as_bytes());

        let mut result = vec![0u8; PBKDF2_KEY_LEN];
        let salt = format!("mnemonic{}", seed_value);

        pbkdf2(&mut mac, salt.as_bytes(), PBKDF2_ROUNDS, &mut result);

        result
    }

    pub fn to_words(&self, wordslist: &[String]) -> Vec<String> {
        // Some explanation is necessary.. This uses nom's macros to create a function that
        // creates a parser specifically for grabbing bits 11 at a time, and putting them in a
        // u16.
        named!(bit_vec<Vec<u16> >, bits!(many0!(take_bits!(u16, 11))));

        let mut mnem_words = Vec::new();
        if let IResult::Done(_, bit_sequence) = bit_vec(self.mnemonic.as_slice()) {
            for idx in bit_sequence.iter() {
                mnem_words.push(wordslist[*idx as usize].clone());
            }
        }

        mnem_words
    }

    fn gen_sha256(hashme: &str) -> String {
        let mut sh = Sha256::new();
        sh.input_str(hashme);

        sh.result_str()
    }
}
