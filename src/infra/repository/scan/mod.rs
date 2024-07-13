use async_trait::async_trait;
use sqlx::PgPool;

use crate::common::error;
use crate::domain::scan::model::{Scan, ScanStatus};
use crate::domain::scan::repository::IScanRepository;
use crate::infra::database::DbPool;

pub struct ScanRepository {
    db_pool: PgPool,
}

#[async_trait]
impl IScanRepository for ScanRepository {
    async fn save_scan(&self, scan: &Scan) -> error::Result<Scan> {
        // TODO: make without converting
        let scan_ports: Option<Vec<i32>> = scan.scan_ports.as_ref().map(|v| v.iter().map(|&x| x as i32).collect());
        let exclude_ports: Option<Vec<i32>> = scan.exclude_ports.as_ref().map(|v| v.iter().map(|&x| x as i32).collect());
        let open_ports: Option<Vec<i32>> = scan.open_ports.as_ref().map(|v| v.iter().map(|&x| x as i32).collect());

        let record = sqlx::query!(
        r#"
        INSERT INTO scans (host, status, scan_ports, exclude_ports, timeout, start_port, end_port, udp, tcp, open_ports)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, host, status as "status: ScanStatus", scan_ports, exclude_ports, timeout, start_port, end_port, udp, tcp, open_ports
                "#,
        scan.host,
        scan.status as ScanStatus,
        scan_ports.as_deref(),
        exclude_ports.as_deref(),
        scan.timeout.map(|v| v as i64),
        scan.start_port.map(|v| v as i32),
        scan.end_port.map(|v| v as i32),
        scan.udp,
        scan.tcp,
        open_ports.as_deref()
        )
            .fetch_one(&self.db_pool)
            .await?;

        Ok(Scan {
            host: record.host,
            status: record.status,
            scan_ports: record.scan_ports.map(|v| v.iter().map(|&x| x as u16).collect()),
            exclude_ports: record.exclude_ports.map(|v| v.iter().map(|&x| x as u16).collect()),
            timeout: record.timeout.map(|v| v as u64),
            start_port: record.start_port.map(|v| v as u16),
            end_port: record.end_port.map(|v| v as u16),
            udp: record.udp,
            tcp: record.tcp,
            open_ports: record.open_ports.map(|v| v.iter().map(|&x| x as u16).collect()),
        })
    }
}

impl ScanRepository {
    pub fn new(db_pool: DbPool) -> Self {
        ScanRepository { db_pool }
    }
}