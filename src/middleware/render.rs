use nickel::{
    MiddlewareResult,
    Response,
    Halt
};
use std::io;

pub fn render<'mw, T, F>(res: Response<'mw, T>, do_render: F)
                  ->MiddlewareResult<'mw, T>
    where F: FnOnce(&mut io::Write) -> io::Result<()>
{
    let mut stream = try!(res.start());
    match do_render(&mut stream) {
        Ok(()) => Ok(Halt(stream)),
        Err(e) => stream.bail(format!("Problem rendering template: {:?}", e))
    }
}
