extern crate sysfs_gpio;

use reqwest;
use reqwest::Client;
use sysfs_gpio::{Direction, Edge, Pin};
use std::env;
// use std::io::prelude::*;
// use std::io::stdout;

// fn interrupt(pin: u64) -> sysfs_gpio::Result<()> {
//     let input = Pin::new(pin);
//     input.with_exported(|| {
//         input.set_direction(Direction::In)?;
//         input.set_edge(Edge::BothEdges)?;
//         let mut poller = input.get_poller()?;
//         loop {
//             match poller.poll(1000)? {
//                 Some(value) => println!("{}", value),
//                 None => {
//                     let mut stdout = stdout();
//                     stdout.write_all(b".")?;
//                     stdout.flush()?;
//                 }
//             }
//         }
//     })
// }

fn make_pulse_request(client: &Client, token: &str) -> Result<(), reqwest::Error> {
    let json = r##"{
        "color":"#f47442",
        "period":0.5,
        "cycles":4,
        "power_on":true
    }"##;
    client
        .post("https://api.lifx.com/v1/lights/all/effects/pulse")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(json)
        .send()?;

    return Ok(());
}

fn run_event_loop(token: &str) {
    let client = Client::new();

    // The switch should be connected to pin ___.
    let input = Pin::new(3);
    let result = input.with_exported(|| {
        input.set_direction(Direction::In)?;
        input.set_edge(Edge::BothEdges)?;
        let mut poller = input.get_poller()?;

        println!("Beginning event loop");

        loop {
            match poller.poll(1000)? {
                Some(value) => println!("{}", value),
                None => {
                    match make_pulse_request(&client, token) {
                        Ok(()) => println!("Interrupting Complete!"),
                        Err(_) => println!("Error!"),
                    }
                }
            }
        }
    });

    match result {
        Ok(_) => println!("Nothing went wrong, nice!"),
        Err(err) => println!("{:?}", err)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./flush <token>");
    } else {
        let token = &args[1];

        // let client = Client::new();
        // make_pulse_request(&client, token).unwrap();

        run_event_loop(token);
    }
}
