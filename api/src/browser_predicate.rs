#![allow(non_snake_case)]

use std::marker::PhantomData;

use actix_web::pred::*;
use actix_web::Request;

pub struct BrowserPredicate<S>(PhantomData<S>);

pub fn IsBrowser<S: 'static>() -> BrowserPredicate<S> {
    BrowserPredicate(PhantomData)
}

impl<S: 'static> Predicate<S> for BrowserPredicate<S> {
    fn check(&self, req: &Request, _: &S) -> bool {
        let info = req.headers();
        match info.get("Accept") {
            Some(value) => value.to_str().unwrap_or("").contains("text/html"),
            None => false,
        }
    }
}

pub struct JSONPredicate<S>(PhantomData<S>);

pub fn AcceptsJSON<S: 'static>() -> JSONPredicate<S> {
    JSONPredicate(PhantomData)
}

impl<S: 'static> Predicate<S> for JSONPredicate<S> {
    fn check(&self, req: &Request, _: &S) -> bool {
        let info = req.headers();
        match info.get("Accept") {
            Some(value) => value.to_str().unwrap_or("").contains("application/json"),
            None => false,
        }
    }
}
