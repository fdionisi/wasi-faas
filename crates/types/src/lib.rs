use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub trait Binary {
    fn ser(&self) -> Vec<u8>;
    fn de(raw: &[u8]) -> Self;
}

#[derive(Deserialize, Serialize)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    CONNECT,
    OPTION,
}

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            status: 200,
            body: vec![],
            headers: HashMap::default(),
        }
    }
}

impl Binary for Request {
    fn ser(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    fn de(raw: &[u8]) -> Self {
        bincode::deserialize(raw).unwrap()
    }
}

impl Binary for Response {
    fn ser(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    fn de(raw: &[u8]) -> Self {
        bincode::deserialize(raw).unwrap()
    }
}
