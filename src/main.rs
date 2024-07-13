use std::sync::Arc;
use crate::infra::database::{check_db_connection, get_db_connection};
use sqlx::migrate::Migrator;
use crate::domain::scan::service::ScanService;
use crate::infra::http::server::Services;
use crate::infra::repository::scan::ScanRepository;

mod config;
mod infra;
mod domain;
mod common;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::config::Config::new();

    let db_pool = get_db_connection()
        .await
        .expect("Failed to connect to database");

    check_db_connection(&db_pool)
        .await
        .expect("Failed to connect to database");

    MIGRATOR
        .run(&db_pool)
        .await
        .expect("Unable to run migrations");

    let scan_repo = ScanRepository::new(db_pool.clone());
    let scan_service = Arc::new(ScanService::new(scan_repo));

    let server = infra::http::server::Server::new(cfg.server.port, Services { scan_service });

    server.run().await;
    Ok(())
}

// let addrs = vec![IpAddr::from([127, 0, 0, 1])];
//
//     let range = PortRange {
//         start: 1,
//         end: 65535,
//     };
//
//     let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Random);
//
//     let _scanner = Scanner::new(
//         &addrs,
//         10,
//         Duration::from_millis(100),
//         1,
//         false,
//         strategy,
//         false,
//         vec![],
//         true,
//     );
//
//     let resp = _scanner.run().await;
//
//     println!("{:?}", resp);