use std::str::FromStr;

pub use http_outbound::{add_to_linker, HeadersParam, HttpError, Method, Request, Response};
use hyper::{client, http, HeaderMap, Uri};

wit_bindgen_host_wasmtime_rust::export!("../../../wit/http-outbound.wit");

pub use crate::allowd_hosts::AllowedHttpHosts;

pub const ALLOW_ALL_HOSTS: &str = "unsafe://allow-all";

struct Headers<'a>(HeadersParam<'a>);

struct RequestEnvelope<'a>(Uri, Request<'a>);

pub struct HttpOutbound {
    allowed_hosts: AllowedHttpHosts,
}

impl HttpOutbound {
    pub fn new(allowed_hosts: AllowedHttpHosts) -> Self {
        Self { allowed_hosts }
    }

    fn is_allowed(&self, url: &str) -> Result<Option<Uri>, HttpError> {
        let url = Uri::from_str(url).map_err(|_| HttpError::InvalidUrl)?;
        if self.allowed_hosts.allow(&url) {
            return Ok(Some(url));
        } else {
            Ok(None)
        }
    }
}

impl http_outbound::HttpOutbound for HttpOutbound {
    fn request(&mut self, req: Request) -> Result<Response, HttpError> {
        let url = self
            .is_allowed(&req.uri)?
            .ok_or(HttpError::DestinationNotAllowed)?;

        let c = client::Client::new();

        dbg!(&url);
        let r =
            futures::executor::block_on(c.request(RequestEnvelope(url, req).try_into()?)).unwrap();

        Ok(Response {
            status: r.status().as_u16(),
            headers: Some(
                r.headers()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap().into()))
                    .collect(),
            ),
            body: Some(vec![]),
        })
    }
}

impl From<Method> for http::Method {
    fn from(m: Method) -> Self {
        match m {
            Method::Get => http::Method::GET,
            Method::Post => http::Method::POST,
            Method::Put => http::Method::PUT,
            Method::Delete => http::Method::DELETE,
            Method::Patch => http::Method::PATCH,
            Method::Head => http::Method::HEAD,
            Method::Options => http::Method::OPTIONS,
        }
    }
}

impl<'a> TryFrom<Headers<'a>> for hyper::HeaderMap {
    type Error = HttpError;

    fn try_from(h: Headers<'a>) -> Result<Self, Self::Error> {
        let mut m = HeaderMap::new();
        for (k, v) in h.0 {
            m.insert(
                http::header::HeaderName::from_str(&k).map_err(|_| HttpError::RequestError)?,
                http::header::HeaderValue::from_str(&v).map_err(|_| HttpError::RequestError)?,
            );
        }
        Ok(m)
    }
}

impl<'a> TryFrom<RequestEnvelope<'a>> for hyper::Request<hyper::Body> {
    type Error = HttpError;

    fn try_from(RequestEnvelope(url, req): RequestEnvelope<'a>) -> Result<Self, Self::Error> {
        let body = req
            .body
            .map(|b| hyper::Body::from(b.to_owned()))
            .unwrap_or_else(|| hyper::Body::empty());

        let mut builder = hyper::Request::builder();

        match (builder.headers_mut(), req.headers.is_empty()) {
            (Some(h), false) => {
                h.extend(hyper::HeaderMap::try_from(Headers(req.headers))?);
            }
            _ => (),
        }

        builder
            .uri(url)
            .method(hyper::Method::from(req.method))
            .body(body)
            .map_err(|_| HttpError::RuntimeError)
    }
}
