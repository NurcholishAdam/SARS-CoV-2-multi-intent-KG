// limit-sarscov2/src/governance.rs
use serde::{Serialize, Deserialize};
use crate::domain::SarsCov2Graph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceThresholds {
    pub virology_min: usize,
    pub genomics_min: usize,
    pub treatment_min: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceDecision {
    pub allowed: bool,
    pub reason: String,
}

pub fn check_merge_allowed(graph: &SarsCov2Graph, t: &EvidenceThresholds) -> GovernanceDecision {
    if graph.virology.len() < t.virology_min {
        return GovernanceDecision { allowed: false, reason: format!("Insufficient virology evidence: {} < {}", graph.virology.len(), t.virology_min) };
    }
    if graph.genomics.len() < t.genomics_min {
        return GovernanceDecision { allowed: false, reason: format!("Insufficient genomics evidence: {} < {}", graph.genomics.len(), t.genomics_min) };
    }
    if graph.treatment.len() < t.treatment_min {
        return GovernanceDecision { allowed: false, reason: format!("Insufficient treatment evidence: {} < {}", graph.treatment.len(), t.treatment_min) };
    }
    GovernanceDecision { allowed: true, reason: "Merge allowed: thresholds satisfied".into() }
}
