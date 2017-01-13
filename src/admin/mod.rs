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

fn handler<'mw>(req: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let config = req.server_data();
    res.send(format!("Server port: {}", config.server.port))
}

pub fn routers() -> Router<Config> {
    let mut router = Router::new();
    router.get("/", middleware!{
        "admin/index"
    });
    router.get("/pages", middleware!{
        "admin/pages"
    });
    router.get("/config", handler);
    router
}
