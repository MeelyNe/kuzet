use crate::domain::scan::model::Scan;
use crate::domain::scan::repository::IScanRepository;
use crate::common::error;

pub struct ScanService<R>
where
    R: IScanRepository,
{
    repository: R,
}

impl<R> ScanService<R>
where
    R: IScanRepository,
{
    pub fn new(repository: R) -> Self {
        Self {
            repository,
        }
    }

    pub async fn create_scan(&self, scan: &Scan) -> error::Result<Scan> {
        self.repository.save_scan(scan).await
    }
}