use debug_print::debug_println;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use lazy_static::lazy_static;
use std::{
    convert::Infallible,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    runtime::Builder,
    sync::oneshot::{self, Sender},
};

lazy_static! {
    static ref G_SENDER: Arc<Mutex<Option<Sender<()>>>> = Arc::default();
    static ref G_RESPONSE: Arc<Mutex<Option<Request<Body>>>> = Arc::default();
}

async fn handle_request(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    // TODO: Add better error handling in case of failure.
    match G_SENDER.lock().unwrap().take() {
        Some(sender) => {
            G_RESPONSE.lock().unwrap().replace(request);

            debug_println!("Killing the server");
            sender.send(()).unwrap();
        }
        None => {
            debug_println!("Kill signal found empty!");
        }
    };
    Ok(Response::new(Body::from("Request handled successfully!")))
}

async fn launch_server_and_wait_for_response(port: u16) {
    // Setup global lock.
    let (tx, rx) = oneshot::channel::<()>();

    match G_SENDER.lock() {
        Ok(mut lock) => { lock.replace(tx); },
        _ => {
            debug_println!("Error when getting the lock for kill signal"); 
            return; 
        }
    };

    // Create scaffolding for the response handler function.
    let make_service = make_service_fn(|_socket: &AddrStream| async move {
        Ok::<_, Infallible>(service_fn(move |request| handle_request(request)))
    });

    let raw_addr = ([127, 0, 0, 1], port);
    let addr = SocketAddr::from(raw_addr);

    let server = Server::bind(&addr)
        .serve(make_service)
        .with_graceful_shutdown(async {
            match rx.await {
                Ok(()) => {debug_println!("Received kill signal!");},
                _ => {debug_println!("Error when getting receiver signal");}
            };
        });

    if let Err(error) = server.await {
        debug_println!("Error when starting the server, details : {}", error);
    }

}

type ResponseClosure = fn(Option<Request<Body>>) -> ();

pub fn start_listening_for_request(port: u16, closure: ResponseClosure) {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(launch_server_and_wait_for_response(port));

    match G_RESPONSE.lock() {
        Ok(mut request) => { closure(request.take()); },
        Err(error) => {
            debug_println!("Mutex poison error when fetching request, details : {}", error);
            closure(None);
        }
    };
}

