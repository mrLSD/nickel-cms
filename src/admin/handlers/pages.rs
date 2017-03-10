use super::*;

pub fn get_main<'mw>(_: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let mut header = HeaderData::new();
    header.title = "Pages";
    header.action = "pages";
    render(res, |o| templates::admin::pages::index(o, &header))
}

pub fn get_create<'mw>(_: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let mut header = HeaderData::new();
    header.title = "Create page";
    header.action = "pages";
    render(res, |o| templates::admin::pages::create(o, &header))
}

pub fn post_create<'mw>(req: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    //let form_body = try_with!(res, req.form_body());
    let mut header = HeaderData::new();
    header.title = "Create page";
    header.action = "pages";

    let form_data = try_with!(res, req.form_body());

    println!("{:?}\n", form_data);
    println!("{:?}\n", form_data.get("test").unwrap_or("test"));
    println!("{:?}\n", form_data.get("Contents[section][]").unwrap_or("pages"));
    println!("{:?}\n", form_data.get("pages[title]").unwrap_or("pages"));

    render(res, |o| templates::admin::pages::create(o, &header))
}
