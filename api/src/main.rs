#![allow(unused_variables)]

extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate json;
extern crate openssl_probe;

mod browser_predicate;

use crate::browser_predicate::{IsBrowser, AcceptsJSON};

use actix_web::http::{Method, StatusCode};
use actix_web::{client, middleware};
use actix_web::{
    fs, pred, server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse,
    Result,
};
use actix_web::dev::HttpResponseBuilder;
use futures::{future::ok as fut_ok, Future};
use std::env;

fn get_rss_feed() -> Box<Future<Item = String, Error = Error>> {
    Box::new(
        client::ClientRequest::get("https://blog.arranfrance.dev/index.xml")
            .finish()
            .unwrap()
            .send()
            .map_err(Error::from)
            .and_then(|resp: client::ClientResponse| {
                resp.body()
                    .from_err()
                    .and_then(|body| fut_ok(String::from_utf8(body.to_vec()).unwrap()))
            }),
    )
}

fn get_json_feed() -> Box<Future<Item = String, Error = Error>> {
    Box::new(
        client::ClientRequest::get("https://blog.arranfrance.dev/feed.json")
            .finish()
            .unwrap()
            .send()
            .map_err(Error::from)
            .and_then(|resp: client::ClientResponse| {
                resp.body()
                    .from_err()
                    .and_then(|body| fut_ok(String::from_utf8(body.to_vec()).unwrap()))
            }),
    )
}

fn json_feed(req: &HttpRequest) -> Box<Future<Item = String, Error = Error>> {
    get_json_feed()
        // .and_then(HttpResponse::Ok().build())
        // .and_then(|string: String| Ok(HttpResponse::Ok().content_type("application/json").body(string)))
        // .responder()
}

fn feed(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    get_rss_feed()
        .and_then(|string: String| Ok(HttpResponse::Ok().content_type("text/xml").body(string)))
        .responder()
}

fn show_web_app(req: &HttpRequest) -> HttpResponseBuilder {
    HttpResponse::Ok()
}

fn handle_404(req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sys = actix::System::new("feeder-api");

    println!("Binding to 8000");

    let addr = server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .middleware(
                middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"),
            )
            .resource("/", |r| {
                 // Web app if Accept text/html
                 r
                    .route()
                    .filter(pred::Get())
                    .filter(IsBrowser())
                    .f(show_web_app);
                
                // JSON response if they Accept application/json
                r
                  .route()
                  .filter(pred::Get())
                  .filter(AcceptsJSON())
                  .f(json_feed);

                // If it's a GET show the plain RSS feed
                r
                    .route()
                    .filter(pred::Get())
                    .f(feed);
                
                // Otherwise disallow
                r.route().filter(pred::Not(pred::Get())).f(
                    |req| HttpResponse::MethodNotAllowed());

            })
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(handle_404);

                // all requests that are not `GET`
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|req| HttpResponse::MethodNotAllowed());
            })
            .finish()
    })
    .bind("0.0.0.0:8000")
    .expect("Can not bind to 0.0.0.0:8000")
    .start();

    let _ = sys.run();
}
