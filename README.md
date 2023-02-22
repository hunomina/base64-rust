# Base 64 encoder/decoder in Rust

Ref: http://www.sunshine2k.de/articles/coding/base64/understanding_base64.html

## Helpful commands

- Build: `cargo build`
- Encode string: `target/debug/base64 Sun` should print `U3Vu`
- Decode string: `target/debug/base64 -d U3Vu` should print `Sun`
