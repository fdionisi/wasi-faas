#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod allowd_hosts {
    use hyper::Uri;
    /// An HTTP host allow-list.
    pub enum AllowedHttpHosts {
        /// All HTTP hosts are allowed (the "insecure:allow-all" value was present in the list)
        AllowAll,
        /// Only the specified hosts are allowed.
        AllowSpecific(Vec<AllowedHttpHost>),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AllowedHttpHosts {
        #[inline]
        fn clone(&self) -> AllowedHttpHosts {
            match self {
                AllowedHttpHosts::AllowAll => AllowedHttpHosts::AllowAll,
                AllowedHttpHosts::AllowSpecific(__self_0) => {
                    AllowedHttpHosts::AllowSpecific(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AllowedHttpHosts {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AllowedHttpHosts::AllowAll => {
                    ::core::fmt::Formatter::write_str(f, "AllowAll")
                }
                AllowedHttpHosts::AllowSpecific(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AllowSpecific",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for AllowedHttpHosts {}
    #[automatically_derived]
    impl ::core::cmp::Eq for AllowedHttpHosts {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Vec<AllowedHttpHost>>;
        }
    }
    impl ::core::marker::StructuralPartialEq for AllowedHttpHosts {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AllowedHttpHosts {
        #[inline]
        fn eq(&self, other: &AllowedHttpHosts) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        AllowedHttpHosts::AllowSpecific(__self_0),
                        AllowedHttpHosts::AllowSpecific(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    _ => true,
                }
        }
    }
    pub struct AllowedHttpHost {
        domain: String,
        port: Option<u16>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AllowedHttpHost {
        #[inline]
        fn clone(&self) -> AllowedHttpHost {
            AllowedHttpHost {
                domain: ::core::clone::Clone::clone(&self.domain),
                port: ::core::clone::Clone::clone(&self.port),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AllowedHttpHost {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "AllowedHttpHost",
                "domain",
                &&self.domain,
                "port",
                &&self.port,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for AllowedHttpHost {
        #[inline]
        fn default() -> AllowedHttpHost {
            AllowedHttpHost {
                domain: ::core::default::Default::default(),
                port: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::marker::StructuralEq for AllowedHttpHost {}
    #[automatically_derived]
    impl ::core::cmp::Eq for AllowedHttpHost {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Option<u16>>;
        }
    }
    impl ::core::marker::StructuralPartialEq for AllowedHttpHost {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AllowedHttpHost {
        #[inline]
        fn eq(&self, other: &AllowedHttpHost) -> bool {
            self.domain == other.domain && self.port == other.port
        }
    }
    impl Default for AllowedHttpHosts {
        fn default() -> Self {
            Self::AllowSpecific(::alloc::vec::Vec::new())
        }
    }
    impl AllowedHttpHosts {
        /// Tests whether the given URL is allowed according to the allow-list.
        pub fn allow(&self, url: &Uri) -> bool {
            match self {
                Self::AllowAll => true,
                Self::AllowSpecific(hosts) => hosts.iter().any(|h| h.allow(url)),
            }
        }
    }
    impl AllowedHttpHost {
        /// Creates a new allow-list entry.
        pub fn new(name: impl Into<String>, port: Option<u16>) -> Self {
            Self { domain: name.into(), port }
        }
        /// An allow-list entry that specifies a host and allows the default port.
        pub fn host(name: impl Into<String>) -> Self {
            Self {
                domain: name.into(),
                port: None,
            }
        }
        /// An allow-list entry that specifies a host and port.
        pub fn host_and_port(name: impl Into<String>, port: u16) -> Self {
            Self {
                domain: name.into(),
                port: Some(port),
            }
        }
        fn allow(&self, url: &Uri) -> bool {
            (url
                .scheme()
                .map(|schema| schema == "http" || schema == "https")
                .unwrap_or(false)) && self.domain == url.host().unwrap_or_default()
                && self.port == url.port_u16()
        }
    }
}
mod client {
    use std::str::FromStr;
    pub use http_outbound::{
        add_to_linker, HeadersParam, HttpError, Method, Request, Response,
    };
    use hyper::{client, http, HeaderMap, Uri};
    #[allow(clippy::all)]
    pub mod http_outbound {
        #[allow(unused_imports)]
        use wit_bindgen_host_wasmtime_rust::{wasmtime, anyhow};
        pub type BodyParam<'a> = &'a [u8];
        pub type BodyResult = Vec<u8>;
        pub type HeadersParam<'a> = Vec<(&'a str, &'a str)>;
        pub type HeadersResult = Vec<(String, String)>;
        #[repr(u8)]
        pub enum HttpError {
            Success,
            DestinationNotAllowed,
            InvalidUrl,
            RequestError,
            RuntimeError,
            TooManyRequests,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for HttpError {
            #[inline]
            fn clone(&self) -> HttpError {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for HttpError {}
        impl ::core::marker::StructuralPartialEq for HttpError {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for HttpError {
            #[inline]
            fn eq(&self, other: &HttpError) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        impl ::core::marker::StructuralEq for HttpError {}
        #[automatically_derived]
        impl ::core::cmp::Eq for HttpError {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl core::fmt::Debug for HttpError {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    HttpError::Success => f.debug_tuple("HttpError::Success").finish(),
                    HttpError::DestinationNotAllowed => {
                        f.debug_tuple("HttpError::DestinationNotAllowed").finish()
                    }
                    HttpError::InvalidUrl => {
                        f.debug_tuple("HttpError::InvalidUrl").finish()
                    }
                    HttpError::RequestError => {
                        f.debug_tuple("HttpError::RequestError").finish()
                    }
                    HttpError::RuntimeError => {
                        f.debug_tuple("HttpError::RuntimeError").finish()
                    }
                    HttpError::TooManyRequests => {
                        f.debug_tuple("HttpError::TooManyRequests").finish()
                    }
                }
            }
        }
        pub type HttpStatus = u16;
        #[repr(u8)]
        pub enum Method {
            Get,
            Post,
            Put,
            Delete,
            Patch,
            Head,
            Options,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Method {
            #[inline]
            fn clone(&self) -> Method {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Method {}
        impl ::core::marker::StructuralPartialEq for Method {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Method {
            #[inline]
            fn eq(&self, other: &Method) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        impl ::core::marker::StructuralEq for Method {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Method {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl core::fmt::Debug for Method {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    Method::Get => f.debug_tuple("Method::Get").finish(),
                    Method::Post => f.debug_tuple("Method::Post").finish(),
                    Method::Put => f.debug_tuple("Method::Put").finish(),
                    Method::Delete => f.debug_tuple("Method::Delete").finish(),
                    Method::Patch => f.debug_tuple("Method::Patch").finish(),
                    Method::Head => f.debug_tuple("Method::Head").finish(),
                    Method::Options => f.debug_tuple("Method::Options").finish(),
                }
            }
        }
        pub type Params<'a> = Vec<(&'a str, &'a str)>;
        pub type Uri<'a> = &'a str;
        pub struct Request<'a> {
            pub method: Method,
            pub uri: Uri<'a>,
            pub headers: HeadersParam<'a>,
            pub params: Params<'a>,
            pub body: Option<BodyParam<'a>>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for Request<'a> {
            #[inline]
            fn clone(&self) -> Request<'a> {
                Request {
                    method: ::core::clone::Clone::clone(&self.method),
                    uri: ::core::clone::Clone::clone(&self.uri),
                    headers: ::core::clone::Clone::clone(&self.headers),
                    params: ::core::clone::Clone::clone(&self.params),
                    body: ::core::clone::Clone::clone(&self.body),
                }
            }
        }
        impl<'a> core::fmt::Debug for Request<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("Request")
                    .field("method", &self.method)
                    .field("uri", &self.uri)
                    .field("headers", &self.headers)
                    .field("params", &self.params)
                    .field("body", &self.body)
                    .finish()
            }
        }
        pub struct Response {
            pub status: HttpStatus,
            pub headers: Option<HeadersResult>,
            pub body: Option<BodyResult>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Response {
            #[inline]
            fn clone(&self) -> Response {
                Response {
                    status: ::core::clone::Clone::clone(&self.status),
                    headers: ::core::clone::Clone::clone(&self.headers),
                    body: ::core::clone::Clone::clone(&self.body),
                }
            }
        }
        impl core::fmt::Debug for Response {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("Response")
                    .field("status", &self.status)
                    .field("headers", &self.headers)
                    .field("body", &self.body)
                    .finish()
            }
        }
        pub trait HttpOutbound: Sized {
            fn request(&mut self, req: Request<'_>) -> Result<Response, HttpError>;
        }
        pub fn add_to_linker<T, U>(
            linker: &mut wasmtime::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<()>
        where
            U: HttpOutbound,
        {
            use wit_bindgen_host_wasmtime_rust::rt::get_memory;
            use wit_bindgen_host_wasmtime_rust::rt::get_func;
            linker
                .func_wrap(
                    "http-outbound",
                    "request: func(req: record { method: enum { get, post, put, delete, patch, head, options }, uri: string, headers: list<tuple<string, string>>, params: list<tuple<string, string>>, body: option<list<u8>> }) -> result<record { status: u16, headers: option<list<tuple<string, string>>>, body: option<list<u8>> }, enum { success, destination-not-allowed, invalid-url, request-error, runtime-error, too-many-requests }>",
                    move |
                        mut caller: wasmtime::Caller<'_, T>,
                        arg0: i32,
                        arg1: i32,
                        arg2: i32,
                        arg3: i32,
                        arg4: i32,
                        arg5: i32,
                        arg6: i32,
                        arg7: i32,
                        arg8: i32,
                        arg9: i32,
                        arg10: i32|
                    {
                        let func = get_func(&mut caller, "cabi_realloc")?;
                        let func_cabi_realloc = func
                            .typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
                        let memory = &get_memory(&mut caller, "memory")?;
                        let (mem, data) = memory.data_and_store_mut(&mut caller);
                        let mut _bc = wit_bindgen_host_wasmtime_rust::BorrowChecker::new(
                            mem,
                        );
                        let host = get(data);
                        let ptr0 = arg1;
                        let len0 = arg2;
                        let len7 = arg4;
                        let base7 = arg3;
                        let mut result7 = Vec::with_capacity(len7 as usize);
                        for i in 0..len7 {
                            let base = base7 + i * 16;
                            result7
                                .push({
                                    let load1 = _bc.load::<i32>(base + 0)?;
                                    let load2 = _bc.load::<i32>(base + 4)?;
                                    let ptr3 = load1;
                                    let len3 = load2;
                                    let load4 = _bc.load::<i32>(base + 8)?;
                                    let load5 = _bc.load::<i32>(base + 12)?;
                                    let ptr6 = load4;
                                    let len6 = load5;
                                    (_bc.slice_str(ptr3, len3)?, _bc.slice_str(ptr6, len6)?)
                                });
                        }
                        let len14 = arg6;
                        let base14 = arg5;
                        let mut result14 = Vec::with_capacity(len14 as usize);
                        for i in 0..len14 {
                            let base = base14 + i * 16;
                            result14
                                .push({
                                    let load8 = _bc.load::<i32>(base + 0)?;
                                    let load9 = _bc.load::<i32>(base + 4)?;
                                    let ptr10 = load8;
                                    let len10 = load9;
                                    let load11 = _bc.load::<i32>(base + 8)?;
                                    let load12 = _bc.load::<i32>(base + 12)?;
                                    let ptr13 = load11;
                                    let len13 = load12;
                                    (_bc.slice_str(ptr10, len10)?, _bc.slice_str(ptr13, len13)?)
                                });
                        }
                        let param0 = Request {
                            method: match arg0 {
                                0 => Method::Get,
                                1 => Method::Post,
                                2 => Method::Put,
                                3 => Method::Delete,
                                4 => Method::Patch,
                                5 => Method::Head,
                                6 => Method::Options,
                                _ => return Err(invalid_variant("Method")),
                            },
                            uri: _bc.slice_str(ptr0, len0)?,
                            headers: result7,
                            params: result14,
                            body: match arg7 {
                                0 => None,
                                1 => {
                                    Some({
                                        let ptr15 = arg8;
                                        let len15 = arg9;
                                        _bc.slice(ptr15, len15)?
                                    })
                                }
                                _ => return Err(invalid_variant("option")),
                            },
                        };
                        let result16 = host.request(param0);
                        match result16 {
                            Ok(e) => {
                                let caller_memory = memory.data_mut(&mut caller);
                                caller_memory
                                    .store(
                                        arg10 + 0,
                                        wit_bindgen_host_wasmtime_rust::rt::as_i32(0i32) as u8,
                                    )?;
                                let Response {
                                    status: status17,
                                    headers: headers17,
                                    body: body17,
                                } = e;
                                caller_memory
                                    .store(
                                        arg10 + 4,
                                        wit_bindgen_host_wasmtime_rust::rt::as_i32(
                                            wit_bindgen_host_wasmtime_rust::rt::as_i32(status17),
                                        ) as u16,
                                    )?;
                                match headers17 {
                                    Some(e) => {
                                        let caller_memory = memory.data_mut(&mut caller);
                                        caller_memory
                                            .store(
                                                arg10 + 8,
                                                wit_bindgen_host_wasmtime_rust::rt::as_i32(1i32) as u8,
                                            )?;
                                        let vec21 = e;
                                        let len21 = vec21.len() as i32;
                                        let result21 = func_cabi_realloc
                                            .call(&mut caller, (0, 0, 4, len21 * 16))?;
                                        for (i, e) in vec21.into_iter().enumerate() {
                                            let base = result21 + (i as i32) * 16;
                                            {
                                                let (t18_0, t18_1) = e;
                                                let vec19 = t18_0;
                                                let ptr19 = func_cabi_realloc
                                                    .call(&mut caller, (0, 0, 1, vec19.len() as i32))?;
                                                let caller_memory = memory.data_mut(&mut caller);
                                                caller_memory.store_many(ptr19, vec19.as_bytes())?;
                                                caller_memory
                                                    .store(
                                                        base + 4,
                                                        wit_bindgen_host_wasmtime_rust::rt::as_i32(
                                                            vec19.len() as i32,
                                                        ),
                                                    )?;
                                                caller_memory
                                                    .store(
                                                        base + 0,
                                                        wit_bindgen_host_wasmtime_rust::rt::as_i32(ptr19),
                                                    )?;
                                                let vec20 = t18_1;
                                                let ptr20 = func_cabi_realloc
                                                    .call(&mut caller, (0, 0, 1, vec20.len() as i32))?;
                                                let caller_memory = memory.data_mut(&mut caller);
                                                caller_memory.store_many(ptr20, vec20.as_bytes())?;
                                                caller_memory
                                                    .store(
                                                        base + 12,
                                                        wit_bindgen_host_wasmtime_rust::rt::as_i32(
                                                            vec20.len() as i32,
                                                        ),
                                                    )?;
                                                caller_memory
                                                    .store(
                                                        base + 8,
                                                        wit_bindgen_host_wasmtime_rust::rt::as_i32(ptr20),
                                                    )?;
                                            }
                                        }
                                        let caller_memory = memory.data_mut(&mut caller);
                                        caller_memory
                                            .store(
                                                arg10 + 16,
                                                wit_bindgen_host_wasmtime_rust::rt::as_i32(len21),
                                            )?;
                                        caller_memory
                                            .store(
                                                arg10 + 12,
                                                wit_bindgen_host_wasmtime_rust::rt::as_i32(result21),
                                            )?;
                                    }
                                    None => {
                                        caller_memory
                                            .store(
                                                arg10 + 8,
                                                wit_bindgen_host_wasmtime_rust::rt::as_i32(0i32) as u8,
                                            )?;
                                    }
                                };
                                match body17 {
                                    Some(e) => {
                                        let caller_memory = memory.data_mut(&mut caller);
                                        caller_memory
                                            .store(
                                                arg10 + 20,
                                                wit_bindgen_host_wasmtime_rust::rt::as_i32(1i32) as u8,
                                            )?;
                                        let vec22 = e;
                                        let ptr22 = func_cabi_realloc
                                            .call(&mut caller, (0, 0, 1, (vec22.len() as i32) * 1))?;
                                        let caller_memory = memory.data_mut(&mut caller);
                                        caller_memory.store_many(ptr22, &vec22)?;
                                        caller_memory
                                            .store(
                                                arg10 + 28,
                                                wit_bindgen_host_wasmtime_rust::rt::as_i32(
                                                    vec22.len() as i32,
                                                ),
                                            )?;
                                        caller_memory
                                            .store(
                                                arg10 + 24,
                                                wit_bindgen_host_wasmtime_rust::rt::as_i32(ptr22),
                                            )?;
                                    }
                                    None => {
                                        let caller_memory = memory.data_mut(&mut caller);
                                        caller_memory
                                            .store(
                                                arg10 + 20,
                                                wit_bindgen_host_wasmtime_rust::rt::as_i32(0i32) as u8,
                                            )?;
                                    }
                                };
                            }
                            Err(e) => {
                                let caller_memory = memory.data_mut(&mut caller);
                                caller_memory
                                    .store(
                                        arg10 + 0,
                                        wit_bindgen_host_wasmtime_rust::rt::as_i32(1i32) as u8,
                                    )?;
                                caller_memory
                                    .store(
                                        arg10 + 4,
                                        wit_bindgen_host_wasmtime_rust::rt::as_i32(e as i32) as u8,
                                    )?;
                            }
                        };
                        Ok(())
                    },
                )?;
            Ok(())
        }
        use wit_bindgen_host_wasmtime_rust::rt::RawMem;
        use wit_bindgen_host_wasmtime_rust::rt::invalid_variant;
    }
    const _: &str = "use * from http-types\n\n// Send an HTTP request and return a response or a potential error.\nrequest: func(req: request) -> result<response, http-error>\n";
    pub use crate::allowd_hosts::AllowedHttpHosts;
    pub const ALLOW_ALL_HOSTS: &str = "unsafe://allow-all";
    struct Headers<'a>(HeadersParam<'a>);
    struct RequestEnvelope<'a>(Uri, Request<'a>);
    /// A very simple implementation for outbound HTTP requests.
    pub struct HttpOutbound {
        allowed_hosts: AllowedHttpHosts,
    }
    impl HttpOutbound {
        pub fn new(allowed_hosts: AllowedHttpHosts) -> Self {
            Self { allowed_hosts }
        }
        /// Check if guest module is allowed to send request to URL, based on the list of
        /// allowed hosts defined by the runtime. If the list of allowed hosts contains
        /// `insecure:allow-all`, then all hosts are allowed.
        /// If `None` is passed, the guest module is not allowed to send the request.
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
            match &url {
                tmp => {
                    {
                        ::std::io::_eprint(
                            ::core::fmt::Arguments::new_v1_formatted(
                                &["[", ":", "] ", " = ", "\n"],
                                &match (
                                    &"crates/extensions/http-outbound/src/client.rs",
                                    &50u32,
                                    &"&url",
                                    &&tmp,
                                ) {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                            ::core::fmt::ArgumentV1::new_display(args.2),
                                            ::core::fmt::ArgumentV1::new_debug(args.3),
                                        ]
                                    }
                                },
                                &[
                                    ::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::core::fmt::rt::v1::Argument {
                                        position: 1usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::core::fmt::rt::v1::Argument {
                                        position: 2usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::core::fmt::rt::v1::Argument {
                                        position: 3usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 4u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ),
                        );
                    };
                    tmp
                }
            };
            let r = futures::executor::block_on(
                    c.request(RequestEnvelope(url, req).try_into()?),
                )
                .unwrap();
            Ok(Response {
                status: r.status().as_u16(),
                headers: Some(
                    r
                        .headers()
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v.to_str().unwrap().into()))
                        .collect(),
                ),
                body: Some(::alloc::vec::Vec::new()),
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
                    http::header::HeaderName::from_str(&k)
                        .map_err(|_| HttpError::RequestError)?,
                    http::header::HeaderValue::from_str(&v)
                        .map_err(|_| HttpError::RequestError)?,
                );
            }
            Ok(m)
        }
    }
    impl<'a> TryFrom<RequestEnvelope<'a>> for hyper::Request<hyper::Body> {
        type Error = HttpError;
        fn try_from(
            RequestEnvelope(url, req): RequestEnvelope<'a>,
        ) -> Result<Self, Self::Error> {
            let body = req
                .body
                .map(|b| hyper::Body::from(b.to_owned()))
                .unwrap_or_else(|| hyper::Body::empty());
            let mut builder = hyper::Request::builder();
            match (builder.headers_mut(), req.headers.is_empty()) {
                (Some(h), false) => {
                    h.extend(hyper::HeaderMap::try_from(Headers(req.headers))?);
                }
                _ => {}
            }
            builder
                .uri(url)
                .method(hyper::Method::from(req.method))
                .body(body)
                .map_err(|_| HttpError::RuntimeError)
        }
    }
}
pub use crate::client::*;
