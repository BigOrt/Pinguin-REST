use colored::Colorize;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{ Body, Request, Response, Server, Method, StatusCode };
use hyper::service::{ make_service_fn, service_fn };

async fn pinguin_run(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Ok(Response::new("Hello from pinguin".into()))

    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            // here 
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Ok(response)
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

    println!("{}", "> pinguin starting ...!".bright_green());
    println!("{}{}", "> Listening on http://".bright_yellow(), addr.to_string().bright_red());

    // run server in loop
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

}
