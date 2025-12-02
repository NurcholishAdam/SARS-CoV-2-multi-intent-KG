// limit-sarscov2/src/serendipity_trace.rs
// Serendipity traces to visualize agent exploration of multiple hypotheses

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Type of hypothesis being explored
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HypothesisType {
    Transmissibility,      // "mutation X increases transmissibility"
    VaccineEfficacy,       // "mutation X affects vaccine efficacy"
    ImmuneEscape,          // "mutation X enables immune escape"
    TreatmentResponse,     // "variant X responds to treatment Y"
    PublicHealthImpact,    // "policy X reduces transmission"
}

/// Single step in exploration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationStep {
    pub id: Uuid,
    pub step_number: usize,
    pub hypothesis: HypothesisType,
    pub query: String,
    pub domains_explored: Vec<String>,
    pub evidence_found: usize,
    pub confidence: f32,
    pub timestamp: String,
}

/// Complete serendipity trace for a research session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerendipityTrace {
    pub id: Uuid,
    pub session_id: String,
    pub question: String,
    pub steps: Vec<ExplorationStep>,
    pub hypotheses_explored: HashMap<HypothesisType, usize>,  // count per type
    pub total_evidence: usize,
    pub cross_domain_jumps: usize,
    pub created_at: String,
}

impl SerendipityTrace {
    pub fn new(session_id: String, question: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id,
            question,
            steps: vec![],
            hypotheses_explored: HashMap::new(),
            total_evidence: 0,
            cross_domain_jumps: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn add_step(&mut self, step: ExplorationStep) {
        // Track hypothesis type
        *self.hypotheses_explored.entry(step.hypothesis.clone()).or_insert(0) += 1;
        
        // Track evidence
        self.total_evidence += step.evidence_found;
        
        // Detect cross-domain jumps
        if self.steps.len() > 0 {
            let prev_domains = &self.steps.last().unwrap().domains_explored;
            let curr_domains = &step.domains_explored;
            if prev_domains != curr_domains {
                self.cross_domain_jumps += 1;
            }
        }
        
        self.steps.push(step);
    }

    pub fn branching_factor(&self) -> f32 {
        if self.steps.is_empty() {
            return 0.0;
        }
        self.hypotheses_explored.len() as f32 / self.steps.len() as f32
    }

    pub fn diversity_score(&self) -> f32 {
        // Shannon entropy of hypothesis distribution
        let total = self.steps.len() as f32;
        if total == 0.0 {
            return 0.0;
        }
        
        let mut entropy = 0.0;
        for count in self.hypotheses_explored.values() {
            let p = *count as f32 / total;
            if p > 0.0 {
                entropy -= p * p.ln();
            }
        }
        entropy
    }

    pub fn exploration_depth(&self) -> usize {
        self.steps.len()
    }

    pub fn avg_confidence(&self) -> f32 {
        if self.steps.is_empty() {
            return 0.0;
        }
        self.steps.iter().map(|s| s.confidence).sum::<f32>() / self.steps.len() as f32
    }

    pub fn summary(&self) -> SerendipitySummary {
        SerendipitySummary {
            trace_id: self.id,
            question: self.question.clone(),
            total_steps: self.steps.len(),
            unique_hypotheses: self.hypotheses_explored.len(),
            branching_factor: self.branching_factor(),
            diversity_score: self.diversity_score(),
            cross_domain_jumps: self.cross_domain_jumps,
            total_evidence: self.total_evidence,
            avg_confidence: self.avg_confidence(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerendipitySummary {
    pub trace_id: Uuid,
    pub question: String,
    pub total_steps: usize,
    pub unique_hypotheses: usize,
    pub branching_factor: f32,
    pub diversity_score: f32,
    pub cross_domain_jumps: usize,
    pub total_evidence: usize,
    pub avg_confidence: f32,
}

/// Builder for creating exploration steps
pub struct StepBuilder {
    step_number: usize,
    hypothesis: HypothesisType,
    query: String,
    domains: Vec<String>,
    evidence: usize,
    confidence: f32,
}

impl StepBuilder {
    pub fn new(step_number: usize, hypothesis: HypothesisType, query: String) -> Self {
        Self {
            step_number,
            hypothesis,
            query,
            domains: vec![],
            evidence: 0,
            confidence: 0.0,
        }
    }

    pub fn domains(mut self, domains: Vec<String>) -> Self {
        self.domains = domains;
        self
    }

    pub fn evidence(mut self, count: usize) -> Self {
        self.evidence = count;
        self
    }

    pub fn confidence(mut self, conf: f32) -> Self {
        self.confidence = conf;
        self
    }

    pub fn build(self) -> ExplorationStep {
        ExplorationStep {
            id: Uuid::new_v4(),
            step_number: self.step_number,
            hypothesis: self.hypothesis,
            query: self.query,
            domains_explored: self.domains,
            evidence_found: self.evidence,
            confidence: self.confidence,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Example trace scenarios
pub mod examples {
    use super::*;

    pub fn omicron_exploration_trace() -> SerendipityTrace {
        let mut trace = SerendipityTrace::new(
            "session-001".into(),
            "How does Omicron BA.5 affect vaccine efficacy and transmissibility?".into(),
        );

        // Step 1: Explore transmissibility hypothesis
        let step1 = StepBuilder::new(
            1,
            HypothesisType::Transmissibility,
            "Omicron BA.5 transmissibility mutations".into(),
        )
        .domains(vec!["Genomics".into(), "Virology".into()])
        .evidence(12)
        .confidence(0.85)
        .build();
        trace.add_step(step1);

        // Step 2: Explore vaccine efficacy hypothesis
        let step2 = StepBuilder::new(
            2,
            HypothesisType::VaccineEfficacy,
            "BA.5 spike mutations vaccine escape".into(),
        )
        .domains(vec!["Immunology".into(), "Genomics".into()])
        .evidence(8)
        .confidence(0.72)
        .build();
        trace.add_step(step2);

        // Step 3: Explore immune escape mechanism
        let step3 = StepBuilder::new(
            3,
            HypothesisType::ImmuneEscape,
            "BA.5 antibody neutralization resistance".into(),
        )
        .domains(vec!["Immunology".into()])
        .evidence(15)
        .confidence(0.88)
        .build();
        trace.add_step(step3);

        // Step 4: Public health implications
        let step4 = StepBuilder::new(
            4,
            HypothesisType::PublicHealthImpact,
            "BA.5 breakthrough infections policy response".into(),
        )
        .domains(vec!["PublicHealth".into(), "Immunology".into()])
        .evidence(6)
        .confidence(0.65)
        .build();
        trace.add_step(step4);

        trace
    }

    pub fn paxlovid_treatment_trace() -> SerendipityTrace {
        let mut trace = SerendipityTrace::new(
            "session-002".into(),
            "What is Paxlovid's effectiveness against different variants?".into(),
        );

        let step1 = StepBuilder::new(
            1,
            HypothesisType::TreatmentResponse,
            "Paxlovid mechanism protease inhibition".into(),
        )
        .domains(vec!["Treatment".into(), "Virology".into()])
        .evidence(10)
        .confidence(0.90)
        .build();
        trace.add_step(step1);

        let step2 = StepBuilder::new(
            2,
            HypothesisType::TreatmentResponse,
            "Paxlovid efficacy Delta variant".into(),
        )
        .domains(vec!["Treatment".into(), "Genomics".into()])
        .evidence(7)
        .confidence(0.82)
        .build();
        trace.add_step(step2);

        let step3 = StepBuilder::new(
            3,
            HypothesisType::TreatmentResponse,
            "Paxlovid efficacy Omicron variants".into(),
        )
        .domains(vec!["Treatment".into(), "Genomics".into()])
        .evidence(9)
        .confidence(0.78)
        .build();
        trace.add_step(step3);

        trace
    }
}
