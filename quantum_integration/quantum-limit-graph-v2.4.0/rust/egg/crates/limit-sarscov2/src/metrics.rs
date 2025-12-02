// limit-sarscov2/src/metrics.rs
use serde::{Serialize, Deserialize};
use crate::domain::SarsCov2Graph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainCoverage {
    pub virology: usize,
    pub genomics: usize,
    pub treatment: usize,
    pub immunology: usize,
    pub public_health: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Serendipity {
    pub branching_factor: f32,     // avg children per intent/step
    pub evidence_diversity: f32,   // heuristic: domain evenness
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SARSCoV2Metrics {
    pub coverage: DomainCoverage,
    pub serendipity: Serendipity,
}

impl SARSCoV2Metrics {
    pub fn compute(graph: &SarsCov2Graph) -> Self {
        let cov = DomainCoverage {
            virology: graph.virology.len(),
            genomics: graph.genomics.len(),
            treatment: graph.treatment.len(),
            immunology: graph.immunology.len(),
            public_health: graph.public_health.len(),
        };
        let total = (cov.virology + cov.genomics + cov.treatment + cov.immunology + cov.public_health) as f32;
        let evenness = if total > 0.0 {
            let counts = [cov.virology, cov.genomics, cov.treatment, cov.immunology, cov.public_health];
            let p: Vec<f32> = counts.iter().map(|c| *c as f32 / total).collect();
            let entropy = -p.iter().map(|x| if *x > 0.0 { x * x.ln() } else { 0.0 }).sum::<f32>();
            entropy
        } else { 0.0 };
        let ser = Serendipity {
            branching_factor: Self::branching_proxy(graph),
            evidence_diversity: evenness,
        };
        Self { coverage: cov, serendipity: ser }
    }

    fn branching_proxy(graph: &SarsCov2Graph) -> f32 {
        // proxy: (domains with evidence) / 5
        let domains_nonempty = [
            !graph.virology.is_empty(),
            !graph.genomics.is_empty(),
            !graph.treatment.is_empty(),
            !graph.immunology.is_empty(),
            !graph.public_health.is_empty(),
        ].iter().filter(|b| **b).count() as f32;
        domains_nonempty / 5.0
    }
}
