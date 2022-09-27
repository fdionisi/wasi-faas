use hyper::Uri;

/// An HTTP host allow-list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AllowedHttpHosts {
    /// All HTTP hosts are allowed (the "insecure:allow-all" value was present in the list)
    AllowAll,
    /// Only the specified hosts are allowed.
    AllowSpecific(Vec<AllowedHttpHost>),
}

// An HTTP host allow-list entry.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AllowedHttpHost {
    domain: String,
    port: Option<u16>,
}

impl Default for AllowedHttpHosts {
    fn default() -> Self {
        Self::AllowSpecific(vec![])
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
        Self {
            domain: name.into(),
            port,
        }
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
        (url.scheme()
            .map(|schema| schema == "http" || schema == "https")
            .unwrap_or(false))
            && self.domain == url.host().unwrap_or_default()
            && self.port == url.port_u16()
    }
}
