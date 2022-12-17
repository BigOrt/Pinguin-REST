use colored::Colorize;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{ Body, Request, Response, Server, Method, StatusCode };
use hyper::service::{ make_service_fn, service_fn };
use futures::TryStreamExt as _;

async fn pinguin_run(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Ok(Response::new("Hello from pinguin".into()))

    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body(); 
        },
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req
                .into_body().map_ok(|chunk| { chunk.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
            });

            *response.body_mut() = Body::wrap_stream(mapping);
        },
        (&Method::POST, "/echo/reverse") => {

            let full_body = hyper::body::to_bytes(req.into_body()).await?;
            let reversed = full_body.iter()
                .rev()
                .cloned()
                .collect::<Vec<u8>>();

            *response.body_mut() = reversed.into();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Ok(response)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    // bind to addr below.
    let addr = SocketAddr::from(([0,0,0,0], 9000));

    // service for connection
    let make_svc = make_service_fn(|_conn| async {
        // convert fn to service
        Ok::<_, Infallible>(service_fn(pinguin_run))
    });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("{}", "> pinguin starting ...!".bright_green());
    println!("{}{}", "> Listening on http://".bright_yellow(), addr.to_string().bright_red());

    // run server in loop
    if let Err(e) = graceful.await {
        eprintln!("Server error: {}", e);
    }

}
