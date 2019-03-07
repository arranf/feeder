#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

#[macro_use]
extern crate log;

extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate json;
extern crate openssl_probe;

use actix_web::{App, AsyncResponder, error, Error, fs,
                HttpMessage, HttpRequest, HttpResponse, pred, Result, server};
use actix_web::{client, middleware};
use actix_web::http::{Method, StatusCode};
use futures::{Future, future::ok as fut_ok};
use std::{env, io, str};

fn get_external_feed() -> Box<Future<Item=String, Error=Error>> {
    Box::new(
        client::ClientRequest::get("https://blog.arranfrance.dev/index.xml")
            .finish().unwrap()
            .send()
            .map_err(Error::from)
            .and_then(
                |resp: client::ClientResponse| 
                resp.body()
                    .from_err()
                    .and_then(|body| {
                        fut_ok(String::from_utf8(body.to_vec()).unwrap())
                    })
            ),
    )
}

fn feed(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    get_external_feed().and_then(|string: String| {
                info!("Sending back response in running");
                Ok(HttpResponse::Ok()
                    .content_type("text/xml")
                    .body(string))
            }).responder()
}


/// 404 handler
fn p404(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?
        .set_status_code(StatusCode::NOT_FOUND))
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();


    let sys = actix::System::new("feeder-api");

    println!("Binding to 8000");
    
    let addr = server::new(
        || App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .middleware(middleware::DefaultHeaders::new()
                    .header("Access-Control-Allow-Origin", "*"))
            .resource("/", |r| r.method(Method::GET).a(feed))
            .resource("/error", |r| r.f(|req| {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"), StatusCode::INTERNAL_SERVER_ERROR)
            }))
            // default
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(p404);

                // all requests that are not `GET`
                r.route().filter(pred::Not(pred::Get())).f(
                    |req| HttpResponse::MethodNotAllowed());
            }))
        .bind("0.0.0.0:8000").expect("Can not bind to 0.0.0.0:8000")
        .shutdown_timeout(0)    // <- Set shutdown timeout to 0 seconds (default 60s)
        .start();

    let _ = sys.run();
}

