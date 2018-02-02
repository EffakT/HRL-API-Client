extern crate futures;
extern crate hyper;
extern crate tokio_core;

use hyper::{Client, Request};
use tokio_core::reactor::Core;
use std::ffi::CStr;


pub struct HttpResponse {
}

impl HttpResponse {
    fn new() -> HttpResponse {
        HttpResponse {
        }
    }
}
#[no_mangle]
pub unsafe extern fn http_post(url_ptr : *const i8, json : *const i8) {
    let url = CStr::from_ptr(url_ptr).to_str().unwrap().to_owned();
    let json = CStr::from_ptr(json).to_str().unwrap().to_owned();
    let url = url.parse().unwrap();
    HttpResponse::new();
    let do_it = move || {
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());
        let mut req = Request::new(hyper::Method::Post, url);
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(json.len() as u64));
        req.set_body(json);
        let work = client.request(req);
        core.run(work).ok();
    };
    std::thread::spawn(move || { do_it() });
}
