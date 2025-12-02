// examples/multi_intent_demo.rs
// Comprehensive demo of SARS-CoV-2 multi-intent knowledge graph

use limit_sarscov2::{
    domain::{SarsCov2Graph, ResearchDomain},
    nodes::*,
    edges::{GraphEdge, builders},
    multi_intent_graph::{MultiIntentGraphBuilder, HypothesisPath},
    serendipity_trace::{SerendipityTrace, StepBuilder, HypothesisType, examples},
    rd::{RDCurve, rd_from_batches},
    retrieval::{CorpusDoc, RetrievalBackend},
    metrics::SARSCoV2Metrics,
};
use uuid::Uuid;

fn main() {
    println!("=== SARS-CoV-2 Multi-Intent Knowledge Graph Demo ===\n");

    // 1. Create base graph
    let root = VirusNode {
        id: Uuid::new_v4(),
        name: "SARS-CoV-2".into(),
        genome_kb: 29.9,
    };
    let mut base_graph = SarsCov2Graph::new(root);

    // 2. Create domain-specific nodes
    
    // Biology: Spike protein
    let spike_node = VirologyNode {
        id: Uuid::new_v4(),
        topic: "Spike protein S1/S2 structure".into(),
        details: "RBD binds ACE2 receptor with high affinity".into(),
    };
    base_graph.add_virology(spike_node.clone());

    // Immunology: Antibody response
    let antibody_node = ImmunologyNode {
        id: Uuid::new_v4(),
        topic: "Neutralizing antibody response".into(),
        details: "IgG antibodies target RBD and NTD regions".into(),
    };
    base_graph.add_immunology(antibody_node.clone());

    // Variants: Omicron
    let omicron_node = GenomicsNode {
        id: Uuid::new_v4(),
        variant: "Omicron BA.5".into(),
        mutations: vec!["L452R".into(), "F486V".into(), "R493Q".into()],
    };
    base_graph.add_genomics(omicron_node.clone());

    // Treatment: Paxlovid
    let paxlovid_node = TreatmentNode {
        id: Uuid::new_v4(),
        therapy: "Paxlovid (nirmatrelvir/ritonavir)".into(),
        mechanism: "3CL protease inhibitor".into(),
    };
    base_graph.add_treatment(paxlovid_node.clone());

    // Public Health: Mask mandates
    let mask_node = PublicHealthNode {
        id: Uuid::new_v4(),
        policy: "Indoor mask mandates".into(),
        effect: "Reduced transmission by 20-30% in controlled studies".into(),
    };
    base_graph.add_public_health(mask_node.clone());

    println!("✓ Created base graph with {} domains", 5);

    // 3. Build multi-intent graph
    let mut graph_builder = MultiIntentGraphBuilder::new(base_graph);

    // Add nodes with intents
    graph_builder = graph_builder
        .with_biology_node(spike_node.clone(), "transmissibility", 15, 0.92)
        .with_immunology_node(antibody_node.clone(), "vaccine_efficacy", 12, 0.85)
        .with_variant_node(omicron_node.clone(), "immune_escape", 18, 0.88)
        .with_treatment_node(paxlovid_node.clone(), "treatment_efficacy", 10, 0.90)
        .with_public_health_node(mask_node.clone(), "transmission_reduction", 8, 0.75);

    // 4. Add causal and correlative edges
    
    // Mutation → immune escape
    let edge1 = builders::mutation_to_immune_escape(
        omicron_node.id,
        antibody_node.id,
        "BA.5 mutations",
        vec!["doi:10.1038/s41586-022-04980-y".into()],
        0.85,
    );
    graph_builder = graph_builder.with_edge(edge1.clone());

    // Treatment → reduced hospitalization
    let edge2 = builders::treatment_to_outcome(
        paxlovid_node.id,
        mask_node.id,
        "Paxlovid",
        vec!["doi:10.1056/NEJMoa2118542".into()],
        0.89,
    );
    graph_builder = graph_builder.with_edge(edge2.clone());

    // Variant → transmissibility
    let edge3 = builders::variant_to_transmissibility(
        omicron_node.id,
        spike_node.id,
        "Omicron BA.5",
        vec!["doi:10.1016/j.cell.2022.06.005".into()],
        0.91,
    );
    graph_builder = graph_builder.with_edge(edge3.clone());

    // Policy → transmission
    let edge4 = builders::policy_to_transmission(
        mask_node.id,
        spike_node.id,
        "Mask mandates",
        vec!["doi:10.1073/pnas.2015954118".into()],
        0.72,
    );
    graph_builder = graph_builder.with_edge(edge4);

    println!("✓ Added {} edges (causal and correlative)", 4);

    // 5. Add hypothesis paths
    let path1 = HypothesisPath {
        id: Uuid::new_v4(),
        hypothesis_type: HypothesisType::Transmissibility,
        description: "BA.5 mutations increase transmissibility via enhanced ACE2 binding".into(),
        node_sequence: vec![omicron_node.id, spike_node.id],
        edge_sequence: vec![edge3.id],
        total_confidence: 0.91,
        evidence_coverage: 0.88,
    };

    let path2 = HypothesisPath {
        id: Uuid::new_v4(),
        hypothesis_type: HypothesisType::VaccineEfficacy,
        description: "BA.5 mutations reduce vaccine efficacy through antibody escape".into(),
        node_sequence: vec![omicron_node.id, antibody_node.id],
        edge_sequence: vec![edge1.id],
        total_confidence: 0.85,
        evidence_coverage: 0.82,
    };

    graph_builder = graph_builder
        .with_hypothesis_path(path1)
        .with_hypothesis_path(path2);

    println!("✓ Added {} hypothesis paths", 2);

    // 6. Add serendipity traces
    let trace1 = examples::omicron_exploration_trace();
    let trace2 = examples::paxlovid_treatment_trace();

    graph_builder = graph_builder
        .with_trace(trace1.clone())
        .with_trace(trace2.clone());

    println!("✓ Added {} serendipity traces", 2);
    println!("  - Trace 1: {} steps, diversity={:.2}", 
        trace1.exploration_depth(), trace1.diversity_score());
    println!("  - Trace 2: {} steps, diversity={:.2}", 
        trace2.exploration_depth(), trace2.diversity_score());

    // 7. Add rate-distortion curves
    let rd_transmissibility = rd_from_batches(
        &[5, 10, 15, 20, 25],
        &[0.8, 0.5, 0.3, 0.2, 0.15],
    );
    let rd_vaccine = rd_from_batches(
        &[3, 8, 12, 18, 22],
        &[0.9, 0.6, 0.4, 0.25, 0.18],
    );

    graph_builder = graph_builder
        .with_rd_curve("transmissibility".into(), rd_transmissibility.clone())
        .with_rd_curve("vaccine_efficacy".into(), rd_vaccine.clone());

    println!("✓ Added {} rate-distortion curves", 2);
    if let Some(knee) = rd_transmissibility.knee() {
        println!("  - Transmissibility R-D knee: rate={:.1}, distortion={:.2}", 
            knee.rate, knee.distortion);
    }

    // 8. Build final graph
    let multi_graph = graph_builder.build();

    println!("\n=== Multi-Intent Graph Statistics ===");
    let stats = multi_graph.statistics();
    println!("Total nodes: {}", stats.total_nodes);
    println!("Total edges: {}", stats.total_edges);
    println!("  - Causal edges: {}", stats.causal_edges);
    println!("  - Correlative edges: {}", stats.correlative_edges);
    println!("  - Cross-domain edges: {}", stats.cross_domain_edges);
    println!("Hypothesis paths: {}", stats.hypothesis_paths);
    println!("Serendipity traces: {}", stats.serendipity_traces);
    println!("Avg trace diversity: {:.2}", stats.avg_trace_diversity);
    println!("Domains covered: {}", stats.domains_covered);

    // 9. Demonstrate retrieval and metrics
    println!("\n=== Retrieval & Metrics ===");
    
    let corpus = vec![
        CorpusDoc {
            id: Uuid::new_v4(),
            domain: "Virology".into(),
            text: "Spike protein RBD mutations enhance ACE2 binding affinity".into(),
            source: "doi:10.1038/s41586-022-04980-y".into(),
        },
        CorpusDoc {
            id: Uuid::new_v4(),
            domain: "Genomics".into(),
            text: "Omicron BA.5 contains L452R, F486V mutations conferring immune escape".into(),
            source: "doi:10.1016/j.cell.2022.06.005".into(),
        },
        CorpusDoc {
            id: Uuid::new_v4(),
            domain: "Treatment".into(),
            text: "Paxlovid protease inhibitor reduces hospitalization by 89%".into(),
            source: "doi:10.1056/NEJMoa2118542".into(),
        },
    ];

    let retrieval = RetrievalBackend::new(corpus);
    let virology_results = retrieval.virology_from("Spike").unwrap();
    println!("Virology retrieval: {} results", virology_results.len());

    let metrics = SARSCoV2Metrics::compute(&multi_graph.base_graph);
    println!("Domain coverage:");
    println!("  - Virology: {}", metrics.coverage.virology);
    println!("  - Genomics: {}", metrics.coverage.genomics);
    println!("  - Treatment: {}", metrics.coverage.treatment);
    println!("  - Immunology: {}", metrics.coverage.immunology);
    println!("  - Public Health: {}", metrics.coverage.public_health);
    println!("Serendipity:");
    println!("  - Branching factor: {:.2}", metrics.serendipity.branching_factor);
    println!("  - Evidence diversity: {:.2}", metrics.serendipity.evidence_diversity);

    // 10. Export summary
    println!("\n=== Export Summary ===");
    let json = serde_json::to_string_pretty(&stats).unwrap();
    println!("{}", json);

    println!("\n✓ Multi-intent graph demo complete!");
}
