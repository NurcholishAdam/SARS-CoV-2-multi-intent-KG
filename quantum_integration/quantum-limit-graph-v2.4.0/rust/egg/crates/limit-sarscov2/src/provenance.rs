use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceNote {
    pub id: Uuid,
    pub source: String,        // paper DOI, dataset, lab report
    pub operation: String,     // add/merge/split
    pub rationale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceTag {
    pub id: Uuid,
    pub label: String,         // "evidence-complete", "unsafe-merge-blocked"
    pub passed: bool,
    pub details: Option<String>,
}
