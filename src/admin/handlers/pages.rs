use super::*;

pub fn get_main<'mw>(_: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let mut header = HeaderData::new();
    header.title = "Pages";
    header.action = "pages";
    render(res, |o| templates::admin::main::index(o, &header))
}
