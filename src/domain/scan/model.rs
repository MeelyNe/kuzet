use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, sqlx::Type, Clone, Copy, Deserialize, Serialize)]
#[sqlx(type_name = "ScanStatus", rename_all = "lowercase")]
pub enum ScanStatus {
    Pending,
    Running,
    Finished,
    Error,
}

/// Scan
/// model of business object, which means scan ports of a host
#[derive(Debug, FromRow)]
pub struct Scan {
    pub host: String,
    pub status: ScanStatus,

    // optional fields
    pub scan_ports: Option<Vec<u16>>,
    pub exclude_ports: Option<Vec<u16>>,
    pub timeout: Option<u64>,
    pub start_port: Option<u16>,
    pub end_port: Option<u16>,
    pub udp: Option<bool>,
    pub tcp: Option<bool>,

    // scan result
    pub open_ports: Option<Vec<u16>>,
}