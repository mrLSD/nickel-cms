pub use nickel::{HttpRouter, Router};
pub use config::{Config};
pub mod handlers;

pub fn routers() -> Router<Config> {
    let mut router = Router::new();
    router.get("/", handlers::main::get_main);

    router.get("/pages", handlers::pages::get_main);
    router.get("/pages/create", handlers::pages::get_create);
    router.post("/pages/create", handlers::pages::post_create);

    router
}
