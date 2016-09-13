extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_marionette as mario;
extern crate tokio_service as service;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_stdlog;
#[macro_use]
extern crate log;

use slog::DrainExt;
use tokio::reactor::Core;
use service::Service;

pub fn main() {
    let log = slog::Logger::root(slog_term::streamer().stderr().build().fuse(),
                                 o!("version" => "0.1"));
    slog_stdlog::set_logger(log.clone()).unwrap();
    let mut core = Core::new().unwrap();

    let addr = "127.0.0.1:2828".parse().unwrap();

    let client = mario::client::connect(core.handle(), &addr);
    let client = core.run(client).unwrap();
}
