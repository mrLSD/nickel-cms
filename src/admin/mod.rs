pub use nickel::{
    Nickel,
    Mount,
    HttpRouter,
    Router,
    Middleware,
    MiddlewareResult,
    Request,
    Response
};
pub use config::{Config};
use tera::{Context};
use middleware::TEMPLATES;
pub mod handlers;

pub fn routers() -> Router<Config> {
    let mut router = Router::new();
    router.get("/", handlers::main::get_main);

    router.get("/pages", handlers::pages::get_main);
    router.get("/pages/create", handlers::pages::get_create);
    router.post("/pages/create", handlers::pages::post_create);

    router.get("/tera", middleware! {
        TEMPLATES.render("hello.html", Context::new()).unwrap()
    });
    router
}
