#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate toml;

#[macro_use] extern crate tera;
#[macro_use] extern crate lazy_static;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
pub use templates::*;

use nickel::{Nickel, Mountable, StaticFilesHandler};

mod admin;
pub mod middleware;
pub mod config;

/// Build all routers rule
pub fn routers(server: &mut Nickel<config::Config>) {
    server.mount("/admin/", admin::routers());
    server.mount("/", StaticFilesHandler::new("assets/"));
}
