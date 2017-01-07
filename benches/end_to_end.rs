#![feature(test)]
#![deny(warnings)]

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate pretty_env_logger;

extern crate test;

use futures::{Future, Stream};
use tokio_core::reactor::Core;

use hyper::client;
use hyper::header::{ContentLength, ContentType};
use hyper::Method;
use hyper::server::{self, Service};


#[bench]
fn get_one_at_a_time(b: &mut test::Bencher) {
    let _ = pretty_env_logger::init();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = hyper::Server::http(&"127.0.0.1:0".parse().unwrap(), &handle).unwrap()
        .handle(|| Ok(Hello), &handle).unwrap();

    let mut client = hyper::Client::new(&handle);

    let url: hyper::Url = format!("http://{}/get", addr).parse().unwrap();

    b.bytes = 160 * 2 + PHRASE.len() as u64;
    b.iter(move || {
        let work = client.get(url.clone()).and_then(|res| {
            res.body().for_each(|_chunk| {
                Ok(())
            })
        });

        core.run(work).unwrap();
    });
}

#[bench]
fn post_one_at_a_time(b: &mut test::Bencher) {
    let _ = pretty_env_logger::init();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = hyper::Server::http(&"127.0.0.1:0".parse().unwrap(), &handle).unwrap()
        .handle(|| Ok(Hello), &handle).unwrap();

    let mut client = hyper::Client::new(&handle);

    let url: hyper::Url = format!("http://{}/get", addr).parse().unwrap();

    let post = "foo bar baz quux";
    b.bytes = 180 * 2 + post.len() as u64 + PHRASE.len() as u64;
    b.iter(move || {
        let mut req = client::Request::new(Method::Post, url.clone());
        req.headers_mut().set(ContentLength(post.len() as u64));
        req.set_body(post);

        let work = client.get(url.clone()).and_then(|res| {
            res.body().for_each(|_chunk| {
                Ok(())
            })
        });

        core.run(work).unwrap();
    });
}

static PHRASE: &'static [u8] = include_bytes!("../CHANGELOG.md"); //b"Hello, World!";

#[derive(Clone, Copy)]
struct Hello;

impl Service for Hello {
    type Request = server::Request;
    type Response = server::Response;
    type Error = hyper::Error;
    type Future = ::futures::Finished<Self::Response, hyper::Error>;
    fn call(&mut self, _req: Self::Request) -> Self::Future {
        ::futures::finished(
            server::Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_header(ContentType::plaintext())
                .with_body(PHRASE)
        )
    }

}
