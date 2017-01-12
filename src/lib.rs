#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate toml;

use nickel::{Nickel, Mountable, StaticFilesHandler};

mod admin;
pub mod config;

/// Build all routers rule
pub fn routers(server: &mut Nickel<config::Config>) {
    //server.mount("/admin/", admin::routers());
    server.mount("/", StaticFilesHandler::new("assets/"));
}
