pub use nickel::{
    Nickel,
    Mount,
    HttpRouter,
    Router,
    Middleware
};

pub fn routers() -> Router {
    let mut router = Router::new();
    router.get("/", middleware!{
        "admin/index"
    });
    router.get("/pages", middleware!{
        "admin/pages"
    });
    router
}
