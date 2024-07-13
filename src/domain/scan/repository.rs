use async_trait::async_trait;
use crate::domain::scan::model::Scan;
use crate::common::error;

#[async_trait]
pub trait IScanRepository {
    async fn save_scan(&self, scan: &Scan) -> error::Result<Scan>;
}