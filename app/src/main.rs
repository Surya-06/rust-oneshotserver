extern crate osw_lib;

use hyper::{Request, Body};

fn handler(request: Option<Request<Body>>) {
    println!("This is in the handler function");

    println!("Exiting now");

    return;
}

fn main() {
    println!("Hello, world!");

    println!("calling basic function");
    osw_lib::basic_dummy();

    /* MAIN TEST */

    let port: u16 = 3000;

    osw_lib::start_listening_for_request(port, handler);

    /* MAIN TEST */
}
