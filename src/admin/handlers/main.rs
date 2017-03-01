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
use middleware::render;
use templates;
use config::{Config};
use super::*;

pub fn get_main<'mw>(_req: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    render(res, |o| templates::hello(o))
}
