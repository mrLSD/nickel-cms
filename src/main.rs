#[macro_use] extern crate nickel;
extern crate nickel_cms;

use nickel::{Nickel, Options};

fn main() {
    let config = nickel_cms::config::load_config();
    let mut server = Nickel::with_data(config);
    server.options = Options::default()
                     .output_on_listen(false)
                     .thread_count(Some(100));

    // Middlewars
    nickel_cms::routers(&mut server);

    server.listen("127.0.0.1:3000").unwrap();
}
