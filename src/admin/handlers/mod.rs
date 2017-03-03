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

pub mod main;
pub mod pages;

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
