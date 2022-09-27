use wasi_faas_sdk::{
    handler,
    http::{request, send, Request, Response},
};

#[handler]
fn main(_: Request) -> Response {
    send(
        request::Builder::new()
            .method("GET")
            .uri("http://httpbin.org/get")
            .body(vec![].into())
            .unwrap(),
    )
    .unwrap()
}
