// limit-sarscov2/src/multi_intent_graph.rs
// Multi-intent knowledge graph with nodes, edges, and hypothesis paths

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

use crate::{
    domain::{SarsCov2Graph, ResearchDomain},
    nodes::*,
    edges::{GraphEdge, EdgeType},
    serendipity_trace::{SerendipityTrace, HypothesisType},
    rd::RDCurve,
};

/// Intent-specific node in the multi-intent graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentNode {
    pub id: Uuid,
    pub intent: String,           // "transmissibility", "vaccine_efficacy", etc.
    pub domain: ResearchDomain,
    pub content: NodeContent,
    pub metadata: NodeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeContent {
    Biology(VirologyNode),
    Immunology(ImmunologyNode),
    Variant(GenomicsNode),
    Treatment(TreatmentNode),
    PublicHealth(PublicHealthNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub evidence_count: usize,
    pub confidence: f32,
    pub sources: Vec<String>,
    pub created_at: String,
}

/// Hypothesis exploration path through the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypothesisPath {
    pub id: Uuid,
    pub hypothesis_type: HypothesisType,
    pub description: String,
    pub node_sequence: Vec<Uuid>,     // ordered node IDs
    pub edge_sequence: Vec<Uuid>,     // ordered edge IDs
    pub total_confidence: f32,
    pub evidence_coverage: f32,
}

/// Multi-intent knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiIntentGraph {
    pub id: Uuid,
    pub base_graph: SarsCov2Graph,
    pub intent_nodes: HashMap<Uuid, IntentNode>,
    pub edges: HashMap<Uuid, GraphEdge>,
    pub hypothesis_paths: Vec<HypothesisPath>,
    pub serendipity_traces: Vec<SerendipityTrace>,
    pub rd_curves: HashMap<String, RDCurve>,  // keyed by intent
    pub metadata: GraphMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMetadata {
    pub created_at: String,
    pub last_updated: String,
    pub total_nodes: usize,
    pub total_edges: usize,
    pub domains_covered: HashSet<String>,
}

impl MultiIntentGraph {
    pub fn new(base_graph: SarsCov2Graph) -> Self {
        Self {
            id: Uuid::new_v4(),
            base_graph,
            intent_nodes: HashMap::new(),
            edges: HashMap::new(),
            hypothesis_paths: vec![],
            serendipity_traces: vec![],
            rd_curves: HashMap::new(),
            metadata: GraphMetadata {
                created_at: chrono::Utc::now().to_rfc3339(),
                last_updated: chrono::Utc::now().to_rfc3339(),
                total_nodes: 0,
                total_edges: 0,
                domains_covered: HashSet::new(),
            },
        }
    }

    /// Add an intent node
    pub fn add_node(&mut self, node: IntentNode) {
        self.metadata.domains_covered.insert(format!("{:?}", node.domain));
        self.intent_nodes.insert(node.id, node);
        self.metadata.total_nodes = self.intent_nodes.len();
        self.update_timestamp();
    }

    /// Add an edge between nodes
    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.insert(edge.id, edge);
        self.metadata.total_edges = self.edges.len();
        self.update_timestamp();
    }

    /// Add a hypothesis path
    pub fn add_hypothesis_path(&mut self, path: HypothesisPath) {
        self.hypothesis_paths.push(path);
        self.update_timestamp();
    }

    /// Add a serendipity trace
    pub fn add_trace(&mut self, trace: SerendipityTrace) {
        self.serendipity_traces.push(trace);
        self.update_timestamp();
    }

    /// Add rate-distortion curve for an intent
    pub fn add_rd_curve(&mut self, intent: String, curve: RDCurve) {
        self.rd_curves.insert(intent, curve);
        self.update_timestamp();
    }

    /// Get all edges of a specific type
    pub fn edges_by_type(&self, edge_type: EdgeType) -> Vec<&GraphEdge> {
        self.edges.values()
            .filter(|e| e.edge_type == edge_type)
            .collect()
    }

    /// Get all nodes in a specific domain
    pub fn nodes_by_domain(&self, domain: ResearchDomain) -> Vec<&IntentNode> {
        self.intent_nodes.values()
            .filter(|n| std::mem::discriminant(&n.domain) == std::mem::discriminant(&domain))
            .collect()
    }

    /// Get cross-domain edges
    pub fn cross_domain_edges(&self) -> Vec<&GraphEdge> {
        self.edges.values()
            .filter(|e| e.is_cross_domain())
            .collect()
    }

    /// Find paths between two nodes
    pub fn find_paths(&self, start_id: Uuid, end_id: Uuid, max_depth: usize) -> Vec<Vec<Uuid>> {
        let mut paths = vec![];
        let mut current_path = vec![start_id];
        let mut visited = HashSet::new();
        
        self.dfs_paths(start_id, end_id, &mut current_path, &mut visited, &mut paths, max_depth);
        paths
    }

    fn dfs_paths(
        &self,
        current: Uuid,
        target: Uuid,
        path: &mut Vec<Uuid>,
        visited: &mut HashSet<Uuid>,
        paths: &mut Vec<Vec<Uuid>>,
        max_depth: usize,
    ) {
        if path.len() > max_depth {
            return;
        }

        if current == target {
            paths.push(path.clone());
            return;
        }

        visited.insert(current);

        // Find outgoing edges
        for edge in self.edges.values() {
            if edge.source_id == current && !visited.contains(&edge.target_id) {
                path.push(edge.target_id);
                self.dfs_paths(edge.target_id, target, path, visited, paths, max_depth);
                path.pop();
            }
        }

        visited.remove(&current);
    }

    /// Calculate graph statistics
    pub fn statistics(&self) -> GraphStatistics {
        let causal_edges = self.edges_by_type(EdgeType::Causal).len();
        let correlative_edges = self.edges_by_type(EdgeType::Correlative).len();
        let cross_domain = self.cross_domain_edges().len();
        
        let avg_trace_diversity = if !self.serendipity_traces.is_empty() {
            self.serendipity_traces.iter()
                .map(|t| t.diversity_score())
                .sum::<f32>() / self.serendipity_traces.len() as f32
        } else {
            0.0
        };

        GraphStatistics {
            total_nodes: self.metadata.total_nodes,
            total_edges: self.metadata.total_edges,
            causal_edges,
            correlative_edges,
            cross_domain_edges: cross_domain,
            hypothesis_paths: self.hypothesis_paths.len(),
            serendipity_traces: self.serendipity_traces.len(),
            avg_trace_diversity,
            domains_covered: self.metadata.domains_covered.len(),
        }
    }

    fn update_timestamp(&mut self) {
        self.metadata.last_updated = chrono::Utc::now().to_rfc3339();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStatistics {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub causal_edges: usize,
    pub correlative_edges: usize,
    pub cross_domain_edges: usize,
    pub hypothesis_paths: usize,
    pub serendipity_traces: usize,
    pub avg_trace_diversity: f32,
    pub domains_covered: usize,
}

/// Builder for constructing multi-intent graphs
pub struct MultiIntentGraphBuilder {
    graph: MultiIntentGraph,
}

impl MultiIntentGraphBuilder {
    pub fn new(base_graph: SarsCov2Graph) -> Self {
        Self {
            graph: MultiIntentGraph::new(base_graph),
        }
    }

    pub fn with_biology_node(mut self, virology: VirologyNode, intent: &str, evidence: usize, confidence: f32) -> Self {
        let node = IntentNode {
            id: virology.id,
            intent: intent.into(),
            domain: ResearchDomain::Virology,
            content: NodeContent::Biology(virology),
            metadata: NodeMetadata {
                evidence_count: evidence,
                confidence,
                sources: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
            },
        };
        self.graph.add_node(node);
        self
    }

    pub fn with_immunology_node(mut self, immunology: ImmunologyNode, intent: &str, evidence: usize, confidence: f32) -> Self {
        let node = IntentNode {
            id: immunology.id,
            intent: intent.into(),
            domain: ResearchDomain::Immunology,
            content: NodeContent::Immunology(immunology),
            metadata: NodeMetadata {
                evidence_count: evidence,
                confidence,
                sources: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
            },
        };
        self.graph.add_node(node);
        self
    }

    pub fn with_variant_node(mut self, genomics: GenomicsNode, intent: &str, evidence: usize, confidence: f32) -> Self {
        let node = IntentNode {
            id: genomics.id,
            intent: intent.into(),
            domain: ResearchDomain::Genomics,
            content: NodeContent::Variant(genomics),
            metadata: NodeMetadata {
                evidence_count: evidence,
                confidence,
                sources: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
            },
        };
        self.graph.add_node(node);
        self
    }

    pub fn with_treatment_node(mut self, treatment: TreatmentNode, intent: &str, evidence: usize, confidence: f32) -> Self {
        let node = IntentNode {
            id: treatment.id,
            intent: intent.into(),
            domain: ResearchDomain::Treatment,
            content: NodeContent::Treatment(treatment),
            metadata: NodeMetadata {
                evidence_count: evidence,
                confidence,
                sources: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
            },
        };
        self.graph.add_node(node);
        self
    }

    pub fn with_public_health_node(mut self, ph: PublicHealthNode, intent: &str, evidence: usize, confidence: f32) -> Self {
        let node = IntentNode {
            id: ph.id,
            intent: intent.into(),
            domain: ResearchDomain::PublicHealth,
            content: NodeContent::PublicHealth(ph),
            metadata: NodeMetadata {
                evidence_count: evidence,
                confidence,
                sources: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
            },
        };
        self.graph.add_node(node);
        self
    }

    pub fn with_edge(mut self, edge: GraphEdge) -> Self {
        self.graph.add_edge(edge);
        self
    }

    pub fn with_hypothesis_path(mut self, path: HypothesisPath) -> Self {
        self.graph.add_hypothesis_path(path);
        self
    }

    pub fn with_trace(mut self, trace: SerendipityTrace) -> Self {
        self.graph.add_trace(trace);
        self
    }

    pub fn with_rd_curve(mut self, intent: String, curve: RDCurve) -> Self {
        self.graph.add_rd_curve(intent, curve);
        self
    }

    pub fn build(self) -> MultiIntentGraph {
        self.graph
    }
}
