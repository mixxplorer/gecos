# gecos

This is a rust library to generate and parse [gecos](https://man.freebsd.org/cgi/man.cgi?query=passwd&sektion=5).

We started developing this library to be used in conjunction with [libnss](https://crates.io/crates/libnss).
For example, this library is used in the [guest-users nss package](https://rechenknecht.net/mixxplorer/guest-users/-/tree/main/nss?ref_type=heads).

## Install

Simply install via `cargo`:

```bash
cargo add gecos
```

## Usage

For a full reference, please check out the [`Gecos`] struct.

```rust
use std::convert::TryFrom;
use gecos::{Gecos, GecosSanitizedString};

// read gecos string from passwd etc.
let raw_gecos_string = "Some Person,Room,Work phone,Home phone,Other 1,Other 2";

let mut gecos = Gecos::from_gecos_string(raw_gecos_string).unwrap();

// access fields like
//         var   field     option   for comp
assert_eq!(gecos.full_name.as_ref().unwrap().to_string(), "Some Person");

// and you even can convert it back to a raw gecos string
assert_eq!(gecos.to_gecos_string(), raw_gecos_string);

// modifying fields work like this
gecos.full_name = Some("Another name".to_string().try_into().unwrap());
// or more explicitly
gecos.room = Some(GecosSanitizedString::new("St. 9".to_string()).unwrap());

assert_eq!(gecos.full_name.as_ref().unwrap().to_string(), "Another name");
assert_eq!(gecos.room.as_ref().unwrap().to_string(), "St. 9");
```
