extern crate bytes;
// The `futures` crate contains the future & stream implementations as well
// as combinators to manipulate the async values.
extern crate futures;
// The `tokio_core` crate contains the async IO runtime.
extern crate tokio_core as tokio;
// The `tokio_proto` crate contains the abstractions and building blocks for
// quickly implementing a protocol client or server.
extern crate tokio_proto as proto;
// The `Service` trait
extern crate tokio_service;

#[macro_use]
extern crate log;

pub mod low_level_transport;
pub use low_level_transport::new_marionette_transport;
// Contains the client part - connecting and calling a remote service.
pub mod client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
