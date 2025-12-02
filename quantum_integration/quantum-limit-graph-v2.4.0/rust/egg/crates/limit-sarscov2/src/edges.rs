// limit-sarscov2/src/edges.rs
// Causal and correlative edges for SARS-CoV-2 knowledge graph

use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Edge types representing different relationship semantics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EdgeType {
    Causal,           // mutation → immune escape
    Correlative,      // treatment → reduced hospitalization
    Mechanistic,      // spike protein → ACE2 binding
    Temporal,         // variant emergence → policy change
    Inhibitory,       // antibody → viral replication
}

/// Causal edge: A causes or leads to B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEdge {
    pub id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub source_domain: String,    // e.g., "Genomics"
    pub target_domain: String,    // e.g., "Immunology"
    pub relationship: String,     // "mutation → immune escape"
    pub evidence_strength: f32,   // 0.0 to 1.0
    pub source_refs: Vec<String>, // DOIs, papers
}

/// Correlative edge: A is associated with B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelativeEdge {
    pub id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub source_domain: String,
    pub target_domain: String,
    pub correlation: String,      // "treatment → reduced hospitalization"
    pub correlation_coeff: f32,   // statistical correlation
    pub source_refs: Vec<String>,
}

/// Unified graph edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: Uuid,
    pub edge_type: EdgeType,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub label: String,
    pub weight: f32,              // importance/strength
    pub metadata: EdgeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeMetadata {
    pub source_domain: String,
    pub target_domain: String,
    pub evidence_refs: Vec<String>,
    pub confidence: f32,
    pub created_at: String,
}

impl GraphEdge {
    pub fn new_causal(
        source_id: Uuid,
        target_id: Uuid,
        label: String,
        source_domain: String,
        target_domain: String,
        evidence_refs: Vec<String>,
        confidence: f32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            edge_type: EdgeType::Causal,
            source_id,
            target_id,
            label,
            weight: confidence,
            metadata: EdgeMetadata {
                source_domain,
                target_domain,
                evidence_refs,
                confidence,
                created_at: chrono::Utc::now().to_rfc3339(),
            },
        }
    }

    pub fn new_correlative(
        source_id: Uuid,
        target_id: Uuid,
        label: String,
        source_domain: String,
        target_domain: String,
        evidence_refs: Vec<String>,
        correlation: f32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            edge_type: EdgeType::Correlative,
            source_id,
            target_id,
            label,
            weight: correlation.abs(),
            metadata: EdgeMetadata {
                source_domain,
                target_domain,
                evidence_refs,
                confidence: correlation.abs(),
                created_at: chrono::Utc::now().to_rfc3339(),
            },
        }
    }

    pub fn is_cross_domain(&self) -> bool {
        self.metadata.source_domain != self.metadata.target_domain
    }
}

/// Example edge builders for common SARS-CoV-2 relationships
pub mod builders {
    use super::*;

    pub fn mutation_to_immune_escape(
        mutation_id: Uuid,
        immune_id: Uuid,
        mutation_name: &str,
        evidence: Vec<String>,
        strength: f32,
    ) -> GraphEdge {
        GraphEdge::new_causal(
            mutation_id,
            immune_id,
            format!("{} → immune escape", mutation_name),
            "Genomics".into(),
            "Immunology".into(),
            evidence,
            strength,
        )
    }

    pub fn treatment_to_outcome(
        treatment_id: Uuid,
        outcome_id: Uuid,
        treatment_name: &str,
        evidence: Vec<String>,
        correlation: f32,
    ) -> GraphEdge {
        GraphEdge::new_correlative(
            treatment_id,
            outcome_id,
            format!("{} → reduced hospitalization", treatment_name),
            "Treatment".into(),
            "PublicHealth".into(),
            evidence,
            correlation,
        )
    }

    pub fn variant_to_transmissibility(
        variant_id: Uuid,
        virology_id: Uuid,
        variant_name: &str,
        evidence: Vec<String>,
        strength: f32,
    ) -> GraphEdge {
        GraphEdge::new_causal(
            variant_id,
            virology_id,
            format!("{} → increased transmissibility", variant_name),
            "Genomics".into(),
            "Virology".into(),
            evidence,
            strength,
        )
    }

    pub fn policy_to_transmission(
        policy_id: Uuid,
        outcome_id: Uuid,
        policy_name: &str,
        evidence: Vec<String>,
        correlation: f32,
    ) -> GraphEdge {
        GraphEdge::new_correlative(
            policy_id,
            outcome_id,
            format!("{} → reduced transmission", policy_name),
            "PublicHealth".into(),
            "Virology".into(),
            evidence,
            correlation,
        )
    }
}
