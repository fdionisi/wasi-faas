use std::{collections::HashMap, convert::Infallible, net::SocketAddr};

use hyper::{
    body::Bytes,
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Server,
};
use wasi_faas_types::{Binary, Request, Response};

use crate::runtime::Runtime;

async fn handle(
    runtime: Runtime,
    hyper_req: hyper::Request<Body>,
) -> Result<hyper::Response<Body>, Infallible> {
    let path = hyper_req.uri().to_string();
    let method = hyper_req.method().to_string();
    let body = hyper::body::to_bytes(hyper_req.into_body())
        .await
        .unwrap()
        .to_vec();
    let request = Request {
        path,
        method,
        body,
        headers: HashMap::default(),
    };

    let response = Response::de(&runtime.exec(request.ser()).await);

    let hyper_res = hyper::Response::builder()
        .status(response.status)
        .body(Body::from(Bytes::from(response.body)))
        .unwrap();

    Ok(hyper_res)
}

pub async fn serve<A>(addr: A, runtime: Runtime) -> Result<(), Box<dyn std::error::Error>>
where
    A: Into<SocketAddr>,
{
    let make_service = make_service_fn(move |_conn: &AddrStream| {
        let runtime = runtime.clone();

        // TODO(@fdionisi): grab the address of the incoming connection.
        // let _addr = conn.remote_addr();

        let service = service_fn(move |req| handle(runtime.clone(), req));

        async move { Ok::<_, Infallible>(service) }
    });

    let server = Server::bind(&addr.into()).serve(make_service);

    Ok(server.await?)
}
