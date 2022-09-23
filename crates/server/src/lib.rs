use std::{collections::HashMap, convert::Infallible, net::SocketAddr};

use hyper::{
    body::Bytes,
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Server as HyperServer,
};
use wasi_faas_runtime::Runtime;
use wasi_faas_types::{Binary, Request, Response};

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
    hyper_req: hyper::Request<Body>,
    runtime: Runtime,
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
