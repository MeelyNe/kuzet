use std::sync::Arc;
use warp::{Filter, path};
use crate::domain::scan::service::ScanService;
use crate::infra::http::scan_controller::create_task;
use crate::infra::repository::scan::ScanRepository;

pub struct Services {
    pub scan_service: Arc<ScanService<ScanRepository>>,
}

pub struct Server {
    port: u16,
    pub services: Services,
}

pub fn with_service<T>(
    service: T,
) -> impl Filter<Extract=(T,), Error=std::convert::Infallible> + Clone
where
    T: Clone + Send,
{
    warp::any().map(move || service.clone())
}

impl Server {
    pub fn new(port: u16, services: Services) -> Self {
        Self { port, services }
    }

    pub async fn run(&self) {
        let healthz = warp::path!("healthz" / String);

        let create_task = warp::post()
            .and(path!("v1"/"scan").boxed())
            .and(path::end())
            .and(with_service(self.services.scan_service.clone()))
            .and(warp::body::json())
            .and_then(create_task);

        warp::serve(healthz.or(create_task))
            .run(([127, 0, 0, 1], self.port))
            .await;
    }
}