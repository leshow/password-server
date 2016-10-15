## MICROSERVICE -- MNEMONIC GENERATOR
---

Project incorporating my [rust-mnemonic](https://github.com/leshow/rust_mnemonic) project with a simple server API, also written in Rust using [Iron](https://github.com/iron/iron).

This project is intended to be deployed internally at a company, or on a local network, although I imagine it could have some use as part of a larger tool.


Setting up server
```bash
$ git clone https://github.com/leshow/password-server.git
$ cd password-server
$ cargo build
```

Generating a mnemonic
```bash
$ cargo run     # assuming you've built first
$ curl http://localhost:3000/passphrase
fame secret fiction loyal chef cry maze essay cousin farm barrel napkin issue predict three coil dutch any below pledge vocal crouch dynamic confirm
$ curl http://localhost:3000/passphrase/6
hen mechanic snow shoot vital december
```

Building and deploying
```bash
$ cargo build --release
$ ./target/release/password-server
GET http://localhost:3000/passphrase -> 200 OK (0.05844 ms)
GET http://localhost:3000/passphrase -> 200 OK (0.055507 ms)
```

On my machine, the debug build run with `cargo run` generates mnemonics ~0.2ms, the release build generates in ~0.05, so there are significant speed gains from creating a release build.

I was also able to get a decent speed up by using the `nom` crate to parse the the hash into the wordslist. The BIP39 implementation I referenced in python took 11 bits at a time, converted to an integer and used that as the index in the wordslist. Using `nom` I was able to very simply make a parser out of their combinator macros that consumes 11 bits of data at a time.
