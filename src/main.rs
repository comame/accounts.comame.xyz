use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::{convert::Infallible, env, net::SocketAddr};

mod auth;
mod crypto;
mod data;
mod db;
mod http;

async fn service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(match http::uri::trim(req.uri().path()) {
        Some(path) => http::redirect::moved_permanently(path.as_str()),
        None => http::routes::routes(req),
    })
}

#[tokio::main]
async fn main() {
    let mysql_user = env::var("MYSQL_USER").unwrap();
    let mysql_password = env::var("MYSQL_PASSWORD").unwrap();
    let mysql_db = env::var("MYSQL_DATABASE").unwrap();
    db::mysql::init(
        format!(
            "mysql://{}:{}@mysql.comame.dev/{}",
            mysql_user, mysql_password, mysql_db
        )
        .as_str(),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(service)) });
    let serve = Server::bind(&addr).serve(make_service);
    let result = tokio::spawn(serve);
    println!("Server is listening on {}", addr);
    if let Err(err) = result.await {
        eprintln!("Server error: {}", err);
    };
}
