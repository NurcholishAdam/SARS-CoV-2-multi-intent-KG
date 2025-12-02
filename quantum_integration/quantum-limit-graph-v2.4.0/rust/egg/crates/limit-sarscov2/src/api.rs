// limit-sarscov2/src/api.rs
use axum::{
    routing::{get, post},
    extract::{Path, State},
    Json, Router,
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::{domain::SarsCov2Graph, metrics::SARSCoV2Metrics, provenance::ProvenanceNote, rd::RDCurve, governance::{EvidenceThresholds, check_merge_allowed}};

#[derive(Clone)]
pub struct AppState {
    pub graphs: Arc<Mutex<Vec<SarsCov2Graph>>>,
    pub provenance: Arc<Mutex<Vec<ProvenanceNote>>>,
    pub rd_curves: Arc<Mutex<Vec<(Uuid, RDCurve)>>>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/graph/:id", get(get_graph))
        .route("/provenance/:id", get(get_provenance))
        .route("/traces/:id", get(get_traces))        // placeholder: returns provenance as “traces”
        .route("/metrics/:id", get(get_metrics))
        .route("/rd/:id", get(get_rd))
        .route("/governance/check/:id", post(post_governance_check))
        .with_state(state)
}

async fn get_graph(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<Option<SarsCov2Graph>> {
    let graphs = state.graphs.lock().unwrap();
    Json(graphs.iter().find(|g| g.id == id).cloned())
}

async fn get_provenance(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<Vec<ProvenanceNote>> {
    let prov = state.provenance.lock().unwrap();
    Json(prov.iter().filter(|p| p.source.contains(&id.to_string())).cloned().collect())
}

async fn get_traces(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<Vec<ProvenanceNote>> {
    // For now, reuse provenance as “serendipity traces”
    get_provenance(State(state), Path(id)).await
}

async fn get_metrics(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<Option<SARSCoV2Metrics>> {
    let graphs = state.graphs.lock().unwrap();
    let g = graphs.iter().find(|g| g.id == id).cloned();
    Json(g.map(|graph| SARSCoV2Metrics::compute(&graph)))
}

async fn get_rd(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<Option<RDCurve>> {
    let curves = state.rd_curves.lock().unwrap();
    Json(curves.iter().find(|(gid, _)| *gid == id).map(|(_, c)| c.clone()))
}

#[derive(serde::Deserialize)]
struct ThresholdsPayload {
    virology_min: usize,
    genomics_min: usize,
    treatment_min: usize,
}

async fn post_governance_check(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ThresholdsPayload>,
) -> Json<Option<crate::governance::GovernanceDecision>> {
    let graphs = state.graphs.lock().unwrap();
    let g = graphs.iter().find(|g| g.id == id).cloned();
    Json(g.map(|graph| {
        let t = EvidenceThresholds {
            virology_min: payload.virology_min,
            genomics_min: payload.genomics_min,
            treatment_min: payload.treatment_min,
        };
        check_merge_allowed(&graph, &t)
    }))
}
