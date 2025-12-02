// limit-sarscov2/src/main.rs
use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

use limit_sarscov2::{api, domain::SarsCov2Graph, nodes::VirusNode};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let root = VirusNode { id: uuid::Uuid::new_v4(), name: "SARS-CoV-2".into(), genome_kb: 30.0 };
    let graph = limit_sarscov2::domain::SarsCov2Graph::new(root);

    let state = api::AppState {
        graphs: std::sync::Arc::new(std::sync::Mutex::new(vec![graph])),
        provenance: std::sync::Arc::new(std::sync::Mutex::new(vec![])),
        rd_curves: std::sync::Arc::new(std::sync::Mutex::new(vec![])),
    };

    let app: Router = api::router(state);
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    tracing::info!("Starting API on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}
