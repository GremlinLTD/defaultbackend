use std::{convert::Infallible, env};
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response_text = env::var("RESP_TEXT").unwrap_or_else(|_| {
            let val = "{\"error\": \"not found\"}".to_string();
            val
        });

    let builder = Response::builder()
        .header(hyper::header::CONTENT_TYPE, hyper::header::HeaderValue::from_static("application/json"))
        .status(StatusCode::OK)
        .body(Body::from(response_text));
    Ok(builder.unwrap())

}

#[tokio::main]
async fn main() {
    let ip = env::var("IP").unwrap_or_else(|_| {
        let val = "0.0.0.0".to_string();
        val
    });

    let port = env::var("PORT").unwrap_or_else(|_| {
        let val = "8080".to_string();
        val
    });

    let addr = format!("{}:{}", ip, port).parse().expect("Unable to parse socket address");
    println!("Listening on http://{}", addr);

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}