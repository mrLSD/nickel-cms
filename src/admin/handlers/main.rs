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
    pub keyword: &'a str,
    pub description: &'a str,
    pub action: &'a str,
}

impl<'a> HeaderData<'a> {
    pub fn new() -> HeaderData<'a> {
        HeaderData {
            title: "",
            keyword: "",
            description: "",
            action: "",
        }
    }
    pub fn is_active(&self, action: &str) -> bool {
        if self.action == action { true } else { false }
    }
}

pub fn get_main<'mw>(_: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let mut header = HeaderData::new();
    header.title = "main";
    header.action = "main";
    render(res, |o| templates::admin::main::index(o, &header))
}
