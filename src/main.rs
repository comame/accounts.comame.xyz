use crate::auth::password::set_password;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

mod auth;
mod crypto;
mod data;
mod db;
mod http;
mod time;

fn create_admin_user() {
    let user_id = env::var("ADMIN_USER").unwrap();
    let password = env::var("ADMIN_PASSWORD").unwrap();

    let user = data::user::User { id: user_id };
    let create_user = db::user::insert_user(&user);
    if let Err(err) = create_user {
        println!("{}", err);
        println!("Skipped creating admin user.");
        return;
    }
    set_password(&user.id, &password);
    println!("Admin user created.");
}

async fn service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(match http::uri::trim(req.uri().path()) {
        Some(path) => http::redirect::moved_permanently(path.as_str()),
        None => http::routes::routes(req).await,
    })
}

#[tokio::main]
async fn main() {
    let mysql_user = env::var("MYSQL_USER").unwrap();
    let mysql_password = env::var("MYSQL_PASSWORD").unwrap();
    let mysql_db = env::var("MYSQL_DATABASE").unwrap();
    let mysql_host = env::var("MYSQL_HOST").unwrap();
    db::mysql::init(&format!(
        "mysql://{}:{}@{}/{}",
        mysql_user, mysql_password, mysql_host, mysql_db
    ));

    create_admin_user();

    let redis_host = env::var("REDIS_HOST").unwrap();
    db::redis::init(&format!("redis://{}", redis_host));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(service)) });
    let serve = Server::bind(&addr).serve(make_service);
    let result = tokio::spawn(serve);
    println!("Server is listening on {}", addr);
    if let Err(err) = result.await {
        eprintln!("Server error: {}", err);
    };
}
