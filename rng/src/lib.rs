//! Implementation details for the `uuid` crate.
//!
//! This crate is not meant to be used directly. It
//! allows `wasm32-unknown-unknown` users who aren't
//! in a JS-enabled runtime to configure a source of
//! randomness via `getrandom`:
//!
//! ```toml
//! [dependencies.uuid]
//! features = ["v4", "rng-getrandom"]
//! ```

#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "getrandom")]
    pub use getrandom;

    #[cfg(feature = "rand")]
    pub use rand;
}
