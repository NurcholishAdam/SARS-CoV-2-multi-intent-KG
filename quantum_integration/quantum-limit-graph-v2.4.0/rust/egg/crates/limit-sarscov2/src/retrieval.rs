// limit-sarscov2/src/retrieval.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use regex::Regex;
use anyhow::Result;

use crate::nodes::{VirologyNode, GenomicsNode, TreatmentNode, ImmunologyNode, PublicHealthNode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusDoc {
    pub id: Uuid,
    pub domain: String,        // "Virology", "Genomics", "Treatment", etc.
    pub text: String,
    pub source: String,        // DOI, URL, dataset ref
}

#[derive(Debug, Clone)]
pub struct RetrievalBackend {
    pub docs: Vec<CorpusDoc>,
}

impl RetrievalBackend {
    pub fn new(docs: Vec<CorpusDoc>) -> Self { Self { docs } }

    pub fn filter_domain(&self, domain: &str) -> Vec<&CorpusDoc> {
        self.docs.iter().filter(|d| d.domain.eq_ignore_ascii_case(domain)).collect()
    }

    pub fn keyword_search(&self, domain: &str, query: &str) -> Vec<&CorpusDoc> {
        let re = Regex::new(&regex::escape(query)).unwrap();
        self.filter_domain(domain)
            .into_iter()
            .filter(|d| re.is_match(&d.text))
            .collect()
    }

    pub fn virology_from(&self, query: &str) -> Result<Vec<VirologyNode>> {
        Ok(self.keyword_search("Virology", query)
            .into_iter()
            .map(|d| VirologyNode {
                id: Uuid::new_v4(),
                topic: "Spike-ACE2 binding".into(),
                details: format!("Evidence: {} | Source: {}", summarize(&d.text), d.source),
            })
            .collect())
    }

    pub fn genomics_from(&self, variant: &str) -> Result<Vec<GenomicsNode>> {
        Ok(self.keyword_search("Genomics", variant)
            .into_iter()
            .map(|d| GenomicsNode {
                id: Uuid::new_v4(),
                variant: variant.into(),
                mutations: extract_mutations(&d.text),
            })
            .collect())
    }

    pub fn treatment_from(&self, therapy: &str) -> Result<Vec<TreatmentNode>> {
        Ok(self.keyword_search("Treatment", therapy)
            .into_iter()
            .map(|d| TreatmentNode {
                id: Uuid::new_v4(),
                therapy: therapy.into(),
                mechanism: infer_mechanism(&d.text),
            })
            .collect())
    }

    pub fn immunology_from(&self, topic: &str) -> Result<Vec<ImmunologyNode>> {
        Ok(self.keyword_search("Immunology", topic)
            .into_iter()
            .map(|d| ImmunologyNode {
                id: Uuid::new_v4(),
                topic: topic.into(),
                details: summarize(&d.text),
            })
            .collect())
    }

    pub fn public_health_from(&self, policy: &str) -> Result<Vec<PublicHealthNode>> {
        Ok(self.keyword_search("PublicHealth", policy)
            .into_iter()
            .map(|d| PublicHealthNode {
                id: Uuid::new_v4(),
                policy: policy.into(),
                effect: summarize(&d.text),
            })
            .collect())
    }
}

fn summarize(text: &str) -> String {
    let max = 240;
    text.chars().take(max).collect::<String>()
}

fn extract_mutations(text: &str) -> Vec<String> {
    // Simple stub; replace with a proper parser (e.g., regex for AA changes)
    let candidates = ["N501Y", "E484K", "D614G", "P681R"];
    candidates.iter().filter(|m| text.contains(*m)).map(|m| m.to_string()).collect()
}

fn infer_mechanism(text: &str) -> String {
    if text.contains("protease") { "Protease inhibitor".into() }
    else if text.contains("polymerase") { "Polymerase inhibitor".into() }
    else { "Mechanism: inferred from corpus".into() }
}
