use crate::nodes::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchDomain {
    Virology,
    Immunology,
    Genomics,
    Treatment,
    PublicHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarsCov2Graph {
    pub id: Uuid,
    pub root: VirusNode,
    pub virology: Vec<VirologyNode>,
    pub immunology: Vec<ImmunologyNode>,
    pub genomics: Vec<GenomicsNode>,
    pub treatment: Vec<TreatmentNode>,
    pub public_health: Vec<PublicHealthNode>,
}

impl SarsCov2Graph {
    pub fn new(root: VirusNode) -> Self {
        Self {
            id: Uuid::new_v4(),
            root,
            virology: vec![],
            immunology: vec![],
            genomics: vec![],
            treatment: vec![],
            public_health: vec![],
        }
    }

    pub fn add_virology(&mut self, node: VirologyNode) { self.virology.push(node); }
    pub fn add_immunology(&mut self, node: ImmunologyNode) { self.immunology.push(node); }
    pub fn add_genomics(&mut self, node: GenomicsNode) { self.genomics.push(node); }
    pub fn add_treatment(&mut self, node: TreatmentNode) { self.treatment.push(node); }
    pub fn add_public_health(&mut self, node: PublicHealthNode) { self.public_health.push(node); }
}
