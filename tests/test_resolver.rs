extern crate futures;
extern crate tokio_timer as timer;
extern crate tokio_middleware as middleware;
extern crate tokio_service as service;
extern crate service_fn;

use futures::{Future, Poll, Async};
use service_fn::service_fn;
use timer::Timer;
use middleware::Timeout;
use service::Service;
use std::io;
use std::time::Duration;
use middleware::Resolver;

#[test]
fn test_request_succeeds() {
    let m = Resolver::new();
    let response = m.call((String::from("www.google.com"), 80)).wait();
    response.unwrap();

}


#[test]
fn test_request_fails() {
    let m = Resolver::new();
    let response = m.call((String::from("dfadf.com"), 80)).wait();
    assert!(response.is_err());
}

