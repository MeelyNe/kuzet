use std::sync::Arc;
use warp::{reject, Rejection, Reply};
use crate::domain::scan::model::Scan;
use crate::domain::scan::service::ScanService;
use crate::infra::http::dto::{CreateTaskDto, CreateTaskResponse};
use crate::infra::repository::scan::ScanRepository;

pub async fn create_task(scan_service: Arc<ScanService<ScanRepository>>, dto: CreateTaskDto) -> Result<impl Reply, Rejection> {
    let scan = Scan::try_from(dto).map_err(reject::custom)?;

    match scan_service.create_scan(&scan).await {
        Ok(task) => Ok(warp::reply::json(&CreateTaskResponse {
            status: task.status,
        })),
        Err(e) => {
            Err(reject::custom(e))
        }
    }
}