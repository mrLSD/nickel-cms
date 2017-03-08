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
    let form_data = try_with!(res, req.form_body());
    //let form_body = try_with!(res, req.form_body());
    let mut header = HeaderData::new();
    header.title = "Create page";
    header.action = "pages";



    //println!("{:?}\n", form_data);
    println!("GET: {:#?}\n", form_data.get("test"));
//    println!("{:?}\n", form_data.get("Contents[section][]").unwrap_or("pages"));
//    println!("{:?}\n", form_data.get("pages[title]").unwrap_or("pages"));

    render(res, |o| templates::admin::pages::create(o, &header))
}

pub fn get_pages<'mw>(_: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let mut header = HeaderData::new();
    header.title = "Create page";
    header.action = "pages";
    render(res, |o| templates::admin::pages::form(o, &header))
}

pub fn post_pages<'mw>(req: &mut Request<Config>, res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let mut header = HeaderData::new();
    header.title = "Create page";
    header.action = "pages";

    let form_data = try_with!(res, req.form_body());
    println!("Title: {:#?}", form_data.get("title"));
    println!("Age: {:#?}", form_data.get("age"));
    println!("Done A: {:#?}", form_data.get("done.a"));
    println!("Done B: {:#?}", form_data.get("done.b"));
    println!("Done C: {:#?}", form_data.all("done.c[]"));
    println!("Done E: {:#?}", form_data.get("done.e"));
    println!("Papers Titlte: {:#?}", form_data.all("papers[].title"));
    println!("Papers Pages: {:#?}", form_data.all("papers[].pages"));

    render(res, |o| templates::admin::pages::form(o, &header))
}