#![deny(warnings)]

extern crate hyper;
extern crate futures;
extern crate pretty_env_logger;
extern crate num_cpus;

use std::thread;

use hyper::header::{ContentLength, ContentType, Server};
use hyper::server::{Http, Service, Request, Response};

static PHRASE: &'static [u8] = b"Hello, world!";

#[derive(Clone, Copy)]
struct Hello;

impl Service for Hello {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = ::futures::Finished<Response, hyper::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        ::futures::finished(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_header(ContentType::plaintext())
                .with_header(Server("rocket".to_owned()))
                .with_body(PHRASE)
        )
    }

}

fn main() {
    pretty_env_logger::init().unwrap();

    for _ in 0..num_cpus::get() - 1 {
        thread::spawn(run);
    }
    run();
}

fn run() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    Http::new()
        .bind(&addr, || Ok(Hello)).unwrap()
        .run().unwrap();
}

