use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize)]
pub(super) struct StoredExchange {
    pub(super) row: StoredRow,
    pub(super) status: u8,
    pub(super) req_headers: Vec<(String, Vec<u8>)>,
    pub(super) req_body: Vec<u8>,
    pub(super) res_headers: Option<Vec<(String, Vec<u8>)>>,
    pub(super) res_body: Option<Vec<u8>>,
}

#[derive(Archive, Serialize, Deserialize)]
pub(super) struct StoredRow {
    pub(super) id: u128,
    pub(super) time: (u64, u32),
    pub(super) addr_type: u8,
    pub(super) dirn: u8,
    pub(super) method: String,
    pub(super) host: String,
    pub(super) uri: String,
    pub(super) status: Option<u16>,
    pub(super) length: u64,
    pub(super) tls: bool,
    pub(super) ip: String,
    pub(super) port: u16,
    pub(super) mime: String,
    pub(super) extension: Option<String>,
    pub(super) title: Option<String>,
    pub(super) cookies: Vec<Vec<u8>>,
    pub(super) start_response_timer: Option<(u64, u32)>,
    pub(super) end_response_timer: Option<(u64, u32)>,
    pub(super) websocket_id: Option<u64>,
    pub(super) notes: String,
}
