extern crate hyper;
extern crate sysfs_gpio;

use hyper::{self, Method, Request, Uri, Write};
use hyper::header::CONTENT_TYPE;
use hyper::rt::{self, Future, Stream};
use hyper::Client;
use sysfs_gpio::{Direction, Edge, Pin};
use std::env;
use std::io::prelude::*;
use std::io::stdout;

fn interrupt(pin: u64) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin);
    input.with_exported(|| {
        input.set_direction(Direction::In)?;
        input.set_edge(Edge::BothEdges)?;
        let mut poller = input.get_poller()?;
        loop {
            match poller.poll(1000)? {
                Some(value) => println!("{}", value),
                None => {
                    let mut stdout = stdout();
                    stdout.write_all(b".")?;
                    stdout.flush()?;
                }
            }
        }
    })
}

fn main() {
    let client = Client::new();
    let uri: Uri = .parse().unwrap();
    let token = "OAUTH_TOKEN";
    let json = r##"{
        "color":"#f47442",
        "period":0.5,
        "cycles":4,
        "power_on":true
    }"##;
    // let mut req = Request::new(Body::from(json));
    let req = Request::builder()
        .method("POST")
        .uri("https://api.lifx.com/v1/lights/:selector/effects/pulse")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(Body::from(json))
        .unwrap();
    // *req.method_mut() = Method::POST;
    // *req.uri_mut() = uri.clone();
    // req.headers_mut().insert(
    //     CONTENT_TYPE,
    //     HeaderValue::from_static("application/json")
    // );

    client
        .request(req)
        .and_then(|resp| {
            println!("POST: {}", resp.status());
            resp.into_body().concat2()
        })

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./interrupt <pin>");
    } else {
        match args[1].parse::<u64>() {
            Ok(pin) => {
                match interrupt(pin) {
                    Ok(()) => println!("Interrupting Complete!"),
                    Err(err) => println!("Error: {}", err),
                }
            }
            Err(_) => println!("Usage: ./interrupt <pin>"),
        }
    }
}
