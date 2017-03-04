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

pub struct HeaderData<'a> {
    pub title: &'a str,
}

pub fn get_main<'mw>(_req: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let header = HeaderData {
        title: "Test",
    };
    render(res, |o| templates::admin::main::index(o, &header))
}
