## MICROSERVICE -- MNEMONIC GENERATOR
---

Project incorporating my [rust-mnemonic](https://github.com/leshow/rust_mnemonic) project with a simple server API, also written in Rust using [Iron](https://github.com/iron/iron).

This project is intended to be deployed internally at a company, or on a local network, although I imagine it could have some use as part of a larger tool.

Currently it uses a feature from the nightly compiler, the `?` syntax. This syntax intends to replace the `try!` macro.


Setting up server
```bash
$ git clone https://github.com/leshow/password-server.git
$ cd password-server
$ cargo build
```

Generating a mnemonic
```bash
$ cargo run     # assuming you've built first
$ curl http://localhost:3000/mnemonic
fame secret fiction loyal chef cry maze essay cousin farm barrel napkin issue predict three coil dutch any below pledge vocal crouch dynamic confirm
```

Building and deploying
```bash
$ cargo build --release
$ ./target/release/password-server
GET http://localhost:3000/mnemonic -> 200 OK (0.060287 ms)
GET http://localhost:3000/mnemonic -> 200 OK (0.061226 ms)
```

On my machine, the debug build run with `cargo run` generates mnemonics around 0.2 - 0.3ms, the release build generates in ~0.06ms, so there are significant speed gains from creating a release build.
