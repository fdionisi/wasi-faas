use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    body::Bytes,
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Server as HyperServer,
};
use wasi_faas_runtime::Runtime;
use wasi_faas_types::HttpInboundResponse;

pub struct Server(Runtime);

pub struct ServerBuilder(Option<Runtime>);

impl Server {
    pub fn builder() -> ServerBuilder {
        ServerBuilder(None)
    }

    pub async fn serve<A>(&self, addr: A) -> Result<(), Box<dyn std::error::Error>>
    where
        A: Into<SocketAddr>,
    {
        let runtime = self.0.clone();
        let make_service = make_service_fn(move |_conn: &AddrStream| {
            let runtime = runtime.clone();

            // TODO(@fdionisi): grab the address of the incoming connection.
            // let _addr = conn.remote_addr();

            let service = service_fn(move |req| invoke(req, runtime.clone()));

            async move { Ok::<_, Infallible>(service) }
        });

        let server = HyperServer::bind(&addr.into()).serve(make_service);

        Ok(server.await?)
    }
}

impl ServerBuilder {
    pub fn build(&mut self) -> Server {
        Server(self.0.take().unwrap())
    }

    pub fn with_runtime(&mut self, runtime: Runtime) -> &mut Self {
        self.0.replace(runtime);
        self
    }
}

async fn invoke(
    req: hyper::Request<Body>,
    runtime: Runtime,
) -> Result<hyper::Response<Body>, Infallible> {
    let path = req.uri().to_string();
    let method = req.method().to_string();

    let headers: Vec<(String, String)> = req
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), format!("{}", v.to_str().unwrap())))
        .collect();

    let body = hyper::body::to_bytes(req.into_body())
        .await
        .unwrap()
        .to_vec();

    let res = runtime.exec(vec![method, path], headers, body).await;

    let res = HttpInboundResponse::de(&res);

    Ok(hyper::Response::builder()
        .status(res.status)
        .body(Body::from(Bytes::from(res.body)))
        .unwrap())
}
