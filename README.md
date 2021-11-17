uuid
---------

[![Latest Version](https://img.shields.io/crates/v/uuid.svg)](https://crates.io/crates/uuid)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.46.0+-yellow.svg)
[![Continuous integration](https://github.com/uuid-rs/uuid/actions/workflows/ci.yml/badge.svg)](https://github.com/uuid-rs/uuid/actions/workflows/ci.yml)

---

Generate and parse UUIDs.

Provides support for Universally Unique Identifiers (UUIDs). A UUID is a
unique 128-bit number, stored as 16 octets. UUIDs are used to  assign
unique identifiers to entities without requiring a central allocating
authority.

They are particularly useful in distributed systems, though they can be used in
disparate areas, such as databases and network protocols.  Typically a UUID
is displayed in a readable string form as a sequence of hexadecimal digits,
separated into groups by hyphens.

The uniqueness property is not strictly guaranteed, however for all
practical purposes, it can be assumed that an unintentional collision would
be extremely unlikely.

## Getting started

To get started with generating random UUIDs, add this to your `Cargo.toml`:

```toml
[dependencies.uuid]
version = "1"
features = ["v4", "fast-rng"]
```

and then call `Uuid::new_v4` in your code:

```rust
use uuid::Uuid;

let my_uuid = Uuid::new_v4();
```

You can also parse UUIDs without needing any crate features:

```rust
use uuid::{Uuid, Version};

let my_uuid = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8")?;

assert_eq!(Some(Version::Random), my_uuid.get_version());
```

You can parse UUIDs at compile time instead of at runtime.

If you add the `macro-diagnostics` feature then you can see much better 
error messages.

```rust
#[macro_use]
extern crate uuid;

let my_uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8")?;

assert_eq!(Some(Version::Random), my_uuid.get_version());
```

## Dependencies

By default, this crate depends on nothing but `std` and cannot generate
[`Uuid`]s. You need to enable the following Cargo features to enable
various pieces of functionality:

* `v1` - adds the `Uuid::new_v1` function and the ability to create a V1
  using an implementation of `uuid::v1::ClockSequence` (usually
`uuid::v1::Context`) and a timestamp from `time::timespec`.
* `v3` - adds the `Uuid::new_v3` function and the ability to create a V3
  UUID based on the MD5 hash of some data.
* `v4` - adds the `Uuid::new_v4` function and the ability to randomly
  generate a `Uuid`.
* `v5` - adds the `Uuid::new_v5` function and the ability to create a V5
  UUID based on the SHA1 hash of some data.
* `macro-diagnostics` - enhances the diagnostics of `uuid!` macro.
* `serde` - adds the ability to serialize and deserialize a `Uuid` using the
  `serde` crate.
* `arbitrary` - adds an `Arbitrary` trait implementation to `Uuid`.
* `fast-rng` - when combined with `v4` uses a faster algorithm for generating
  random UUIDs. This feature requires more dependencies to compile, but is just
  as suitable for UUIDs as the default algorithm.

You need to enable one of the following Cargo features together with the
`v4` feature if you're targeting `wasm32-unknown-unknown` target:

* `js` - enables support for randomness on
  `wasm32-unknown-unknown` via [`getrandom`]

Alternatively, you can provide a custom `getrandom` implementation yourself
via [`getrandom::register_custom_getrandom`](https://docs.rs/getrandom/0.2.2/getrandom/macro.register_custom_getrandom.html).

### Unstable features

Some features are unstable. They may be incomplete or depend on other unstable libraries.
These include:

* `zerocopy-unstable` - adds support for zero-copy deserialization using the `zerocopy` library.

Unstable features may break between minor releases.

To allow unstable features, you'll need to enable the Cargo feature as normal, but also pass an additional
flag through your environment to opt-in to unstable `uuid` features:

```
RUSTFLAGS="--cfg uuid_unstable"
```

## Minimum Supported Rust Version (MSRV)

The minimum supported Rust version for `uuid` is documented in
CI. It may be bumped in minor releases as necessary.

## References

* [Wikipedia: Universally Unique Identifier](     http://en.wikipedia.org/wiki/Universally_unique_identifier)
* [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](     http://tools.ietf.org/html/rfc4122)

[`wasm-bindgen`]: https://github.com/rustwasm/wasm-bindgen

[`Uuid`]: https://docs.rs/uuid/1.0.0-alpha.1/uuid/struct.Uuid.html

---
# License

Licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.


[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fuuid-rs%2Fuuid.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fuuid-rs%2Fuuid?ref=badge_large)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.