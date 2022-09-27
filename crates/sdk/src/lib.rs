pub mod error;
pub mod result;

pub use wasi_faas_macros::*;

pub use bytes;

pub mod http {

    wit_bindgen_guest_rust::import!("../../wit/http-outbound.wit");

    use bytes::Bytes;
    pub use wasi_faas_types::HttpInboundResponse;

    pub use http::header::{HeaderName, HeaderValue};

    pub use self::http_outbound::HttpError;
    use self::http_outbound::{Request as HttpOutboundRequest, Response as HttpOutboundResponse};

    type Result<T> = std::result::Result<T, HttpError>;

    pub mod request {
        pub use http::request::{Builder, Parts};
        pub type Request = http::Request<bytes::Bytes>;
    }

    pub mod response {
        pub use http::response::{Builder, Parts};
        pub type Response = http::Response<bytes::Bytes>;
    }

    pub use request::Request;
    pub use response::Response;

    pub fn send(req: Request) -> Result<Response> {
        let (req, body) = req.into_parts();

        let method = req.method.try_into()?;

        let uri = req.uri.to_string();

        let params = vec![];

        let headers = &req
            .headers
            .iter()
            .map(try_header_to_strs)
            .collect::<Result<Vec<_>>>()?;

        let body = body.to_vec();

        let out_req = HttpOutboundRequest {
            method,
            uri: &uri,
            params: &params,
            headers,
            body: Some(body.as_ref()),
        };

        let HttpOutboundResponse {
            status,
            headers,
            body,
        } = self::http_outbound::request(out_req)?;

        let resp_builder = response::Builder::new().status(status);
        let resp_builder = headers
            .into_iter()
            .flatten()
            .fold(resp_builder, |b, (k, v)| b.header(k, v));

        resp_builder
            .body(body.map(Into::into).unwrap_or(Bytes::default()))
            .map_err(|_| HttpError::RuntimeError)
    }

    fn try_header_to_strs<'k, 'v>(
        header: (&'k HeaderName, &'v HeaderValue),
    ) -> Result<(&'k str, &'v str)> {
        Ok((
            header.0.as_str(),
            header.1.to_str().map_err(|_| HttpError::InvalidUrl)?,
        ))
    }

    impl TryFrom<http::Method> for self::http_outbound::Method {
        type Error = HttpError;

        fn try_from(method: http::Method) -> Result<Self> {
            use self::http_outbound::Method::*;
            use http::Method;
            Ok(match method {
                Method::GET => Get,
                Method::POST => Post,
                Method::PUT => Put,
                Method::DELETE => Delete,
                Method::PATCH => Patch,
                Method::HEAD => Head,
                Method::OPTIONS => Options,
                _ => return Err(self::http_outbound::HttpError::RequestError),
            })
        }
    }

    impl std::fmt::Display for HttpError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl std::error::Error for HttpError {}
}
