#![deny(warnings)]
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "0.2", features = ["macros"] }`
#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(dispatcher))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn dispatcher(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // println!("{:#?}", _req);
    // println!("{:#?}", _req.uri());
    let uri = _req.uri();
    let query = uri.query();
    // println!("{:#?}", query);
    let params_str = query.unwrap();
    let params: Vec<&str> = params_str.split("&").collect();
    println!("{:#?}", params);

    Ok(Response::new("Hello, World".into()))
}

// async fn handle_request(url: &str) -> Result<(), reqwest::Error> {
//     let res = reqwest::get(url).await?;

//     println!("Status: {}", res.status());

//     let body = res.text().await?;

//     println!("Body:\n\n{:#?}", body);

//     return (body, OK());
// }