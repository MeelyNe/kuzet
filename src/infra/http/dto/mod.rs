use serde::{Deserialize, Serialize};

use crate::common::error::CustomError;
use crate::domain::scan::model::{Scan, ScanStatus};

#[derive(Deserialize)]
pub struct CreateTaskDto {
    pub host: String,
    pub start_port: Option<u16>,
    pub end_port: Option<u16>,
    pub udp: Option<bool>,
    pub tcp: Option<bool>,
}

#[derive(Serialize)]
pub struct CreateTaskResponse {
    pub status: ScanStatus,
}

impl TryFrom<CreateTaskDto> for Scan {
    type Error = CustomError;

    fn try_from(value: CreateTaskDto) -> Result<Self, Self::Error> {
        Ok(Scan {
            host: value.host,
            status: ScanStatus::Pending,
            scan_ports: None,
            exclude_ports: None,
            timeout: None,
            start_port: value.start_port,
            end_port: value.end_port,
            udp: value.udp,
            tcp: value.tcp,
            open_ports: None,
        })
    }
}