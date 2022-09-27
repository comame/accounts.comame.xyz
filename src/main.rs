use std::{convert::Infallible, net::SocketAddr};
use hyper::{Request, Body, Response, Server};
use hyper::service::{make_service_fn, service_fn};

mod http;
mod handler;
mod crypto;

async fn service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut path = String::from(req.uri().path());
    let trimed = http::uri::trim(&mut path);

    Ok(if trimed {
        http::routes::redirect(&path)
    } else {
        http::routes::routes(req)
    })
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([ 127, 0, 0, 1 ], 8080));

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(service))
    });

    let serve = Server::bind(&addr).serve(make_service);

    let result = tokio::spawn(serve);

    println!("Server is listening on {}", addr);

    if let Err(err) = result.await {
        eprintln!("Server error: {}", err);
    };
}
