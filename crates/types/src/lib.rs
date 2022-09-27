use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HttpInboundResponse {
    pub status: u16,
    pub body: Vec<u8>,
    pub headers: Vec<(String, String)>,
}

impl Default for HttpInboundResponse {
    fn default() -> Self {
        Self {
            status: 200,
            body: vec![],
            headers: vec![],
        }
    }
}

impl HttpInboundResponse {
    pub fn ser(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn de(raw: &[u8]) -> Self {
        bincode::deserialize(raw).unwrap()
    }
}
