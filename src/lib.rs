#[macro_use] extern crate nickel;

use nickel::{Nickel, Mountable, StaticFilesHandler};

mod admin;

/// Build all routers rule
pub fn routers(server: &mut Nickel<()>) {
    server.mount("/admin/", admin::routers());

    server.mount("/", StaticFilesHandler::new("assets/"));
}
