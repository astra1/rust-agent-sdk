#[macro_use]
extern crate serde_derive;
extern crate bytes;
extern crate hyper;
extern crate lru;
extern crate mio_extras;
extern crate serde;
extern crate url;
extern crate ws;

mod csdsclient;
mod external_services;
mod sdk;
mod structs;
mod transformer;
mod transport;
mod utils;

/// Error returned by most functions.
///
/// check error handling crate
///
/// For performance reasons, boxing is avoided in any hot path. For example, in
/// `parse`, a custom error `enum` is defined. This is because the error is hit
/// and handled during normal execution when a partial frame is received on a
/// socket. `std::error::Error` is implemented for `parse::Error` which allows
/// it to be converted to `Box<dyn std::error::Error>`.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for mini-redis operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, Error>;
