extern crate tokio;
extern crate tokio_timer;
extern crate eui48;

extern crate client;

use std::net::Ipv4Addr;
use tokio::prelude::*;

fn main() {
    let future = client::Client::new(
        Some(Ipv4Addr::new(192,168,0,12)),
    )
        .unwrap()
        .map_err(|error| println!("{}", error))
        .map(|result| match result {
            Some((message, _addr)) => println!("{}", message),
            None => println!("None"),
        });
    tokio::run(future);

//    let future = future::ok::<i32, ()>(match socket.start_send((discover, server_addr)) {
//        Ok(AsyncSink::Ready) => {
//            match socket.poll_complete() {
//                Ok(Async::Ready(_)) => 1,
//                Ok(Async::NotReady) => 2,
//                Err(_) => 3,
//            }
//        },
//        Ok(AsyncSink::NotReady(_)) => 4,
//        Err(_) => 5,
//    });
}