use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirusNode {
    pub id: Uuid,
    pub name: String,         // "SARS-CoV-2"
    pub genome_kb: f32,       // ~30.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirologyNode {
    pub id: Uuid,
    pub topic: String,        // "Spike-ACE2 binding"
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunologyNode {
    pub id: Uuid,
    pub topic: String,        // "Antibody neutralization", "T-cell response"
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomicsNode {
    pub id: Uuid,
    pub variant: String,      // "Alpha", "Delta", "Omicron"
    pub mutations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatmentNode {
    pub id: Uuid,
    pub therapy: String,      // "Paxlovid", "Remdesivir", "mAbs"
    pub mechanism: String,    // "Protease inhibitor", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicHealthNode {
    pub id: Uuid,
    pub policy: String,       // "Mask mandate", "Ventilation"
    pub effect: String,       // "Reduced transmission", etc.
}
