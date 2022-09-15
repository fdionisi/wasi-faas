use wasi_faas_interface::{handler, Request, Response};

#[handler]
fn main(req: Request) -> Response {
    Response {
        status: 201,
        body: req.body,
        ..Default::default()
    }
}
